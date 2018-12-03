use std::{fmt, marker::PhantomData, mem, ops::Range};

use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde_json;
use stdweb::{Reference, Value};

use {
    constants::{
        find::Exit, Color, Direction, FindConstant, LookConstant, ReturnCode, StructureType,
        Terrain,
    },
    memory::MemoryReference,
    objects::{
        ConstructionSite, Creep, Flag, HasPosition, Mineral, Nuke, Resource, Room, RoomPosition,
        RoomTerrain, Source, Structure, StructureController, StructureStorage, StructureTerminal,
        Tombstone,
    },
    pathfinder::CostMatrix,
    positions::LocalRoomName,
    traits::{TryFrom, TryInto},
    ConversionError,
};

simple_accessors! {
    Room;
    (controller -> controller -> Option<StructureController>),
    (energy_available -> energyAvailable -> u32),
    (energy_capacity_available -> energyCapacityAvailable -> u32),
    (name -> name -> String),
    (storage -> storage -> Option<StructureStorage>),
    (terminal -> terminal -> Option<StructureTerminal>),
    // todo: visual
}

scoped_thread_local!(static COST_CALLBACK: Box<Fn(String, Reference) -> Option<Reference>>);

impl Room {
    pub fn serialize_path(&self, path: &[Step]) -> String {
        js_unwrap!{@{self.as_ref()}.serializePath(@{path})}
    }

    pub fn deserialize_path(&self, path: &str) -> Vec<Step> {
        js_unwrap!{@{self.as_ref()}.deserializePath(@{path})}
    }

    pub fn create_construction_site<T>(&self, at: T, ty: StructureType) -> ReturnCode
    where
        T: HasPosition,
    {
        let pos = at.pos();
        js_unwrap!(@{self.as_ref()}.createConstructionSite(
            @{pos.as_ref()},
            __structure_type_num_to_str(@{ty as u32})
        ))
    }

    pub fn create_named_construction_site<T>(
        &self,
        at: T,
        ty: StructureType,
        name: &str,
    ) -> ReturnCode
    where
        T: HasPosition,
    {
        let pos = at.pos();
        js_unwrap!(@{self.as_ref()}.createConstructionSite(
            @{pos.as_ref()},
            __structure_type_num_to_str(@{ty as u32}),
            @{name}
        ))
    }

    pub fn create_flag<T>(
        &self,
        at: T,
        name: &str,
        main_color: Color,
        secondary_color: Color,
    ) -> ReturnCode
    where
        T: HasPosition,
    {
        let pos = at.pos();
        js_unwrap!(@{self.as_ref()}.createFlag(
            @{pos.as_ref()},
            @{name},
            @{main_color as u32},
            @{secondary_color as u32}
        ))
    }

    pub fn find<T>(&self, ty: T) -> Vec<T::Item>
    where
        T: FindConstant,
    {
        js_unwrap_ref!(@{self.as_ref()}.find(@{ty.find_code()}))
    }

    pub fn find_exit_to(&self, room: &Room) -> Result<Exit, ReturnCode> {
        let code_val = js! {return @{self.as_ref()}.findExitTo(@{room.as_ref()});};
        let code_int: i32 = code_val.try_into().unwrap();

        if code_int < 0 {
            Err(code_int.try_into().unwrap())
        } else {
            Ok(code_int.try_into().unwrap())
        }
    }

    pub fn get_event_log(&self) -> Vec<Event> {
        serde_json::from_str(&self.get_event_log_raw()).expect("Malformed Event Log")
    }

    pub fn get_event_log_raw(&self) -> String {
        js_unwrap!{@{self.as_ref()}.getEventLog(true)}
    }

    pub fn get_position_at(&self, x: u32, y: u32) -> Option<RoomPosition> {
        js_unwrap!{@{self.as_ref()}.getPositionAt(@{x}, @{y})}
    }

    pub fn get_terrain(&self) -> RoomTerrain {
        js_unwrap!(@{self.as_ref()}.getTerrain())
    }

    pub fn look_at<T: HasPosition>(&self, target: &T) -> Vec<LookResult> {
        let rp = target.pos();
        js_unwrap!(@{self.as_ref()}.lookAt(@{rp.as_ref()}))
    }

    pub fn look_at_xy(&self, x: u32, y: u32) -> Vec<LookResult> {
        js_unwrap!(@{self.as_ref()}.lookAt(@{x}, @{y}))
    }

    pub fn look_at_area(
        &self,
        top: u32,
        left: u32,
        bottom: u32,
        right: u32,
    ) -> Vec<PositionedLookResult> {
        js_unwrap!(@{self.as_ref()}.lookAtArea(@{top}, @{left}, @{bottom}, @{right}, true))
    }

    pub fn find_path<'a, O, T, F>(&self, from_pos: &O, to_pos: &T, opts: FindOptions<'a, F>) -> Path
    where
        O: HasPosition,
        T: HasPosition,
        F: Fn(String, CostMatrix) -> Option<CostMatrix<'a>> + 'a,
    {
        let from = from_pos.pos();
        let to = to_pos.pos();

        // This callback is the one actually passed to JavaScript.
        fn callback(room_name: String, cost_matrix: Reference) -> Option<Reference> {
            COST_CALLBACK.with(|callback| callback(room_name, cost_matrix))
        }

        // User provided callback: rust String, CostMatrix -> Option<CostMatrix>
        let raw_callback = opts.cost_callback;

        // Wrapped user callback: rust String, Reference -> Option<Reference>
        let callback_boxed = move |room_name, cost_matrix_ref| {
            let cmatrix = CostMatrix {
                inner: cost_matrix_ref,
                lifetime: PhantomData,
            };
            raw_callback(room_name, cmatrix).map(|cm| cm.inner)
        };

        // Type erased and boxed callback: no longer a type specific to the closure passed in,
        // now unified as Box<Fn>
        let callback_type_erased: Box<Fn(String, Reference) -> Option<Reference> + 'a> =
            Box::new(callback_boxed);

        // Overwrite lifetime of box inside closure so it can be stuck in scoped_thread_local storage:
        // now pretending to be static data so that it can be stuck in scoped_thread_local. This should
        // be entirely safe because we're only sticking it in scoped storage and we control the only use
        // of it, but it's still necessary because "some lifetime above the current scope but otherwise
        // unknown" is not a valid lifetime to have PF_CALLBACK have.
        let callback_lifetime_erased: Box<
            Fn(String, Reference) -> Option<Reference> + 'static,
        > = unsafe { mem::transmute(callback_type_erased) };

        let FindOptions {
            ignore_creeps,
            ignore_destructible_structures,
            max_ops,
            heuristic_weight,
            serialize,
            max_rooms,
            range,
            plain_cost,
            swamp_cost,
            ..
        } = opts;

        // Store callback_lifetime_erased in COST_CALLBACK for the duration of the PathFinder call and
        // make the call to PathFinder.
        //
        // See https://docs.rs/scoped-tls/0.1/scoped_tls/
        COST_CALLBACK.set(&callback_lifetime_erased, || {
            let v = js!{
                return @{&self.as_ref()}.search(@{from.as_ref()}, @{to.as_ref()}, {
                    ignoreCreeps: @{ignore_creeps},
                    ignoreDestructibleStructures: @{ignore_destructible_structures}
                    costCallback: @{callback},
                    maxOps: @{max_ops},
                    heuristicWeight: @{heuristic_weight},
                    serialize: @{serialize},
                    maxRooms: @{max_rooms},
                    range: @{range},
                    plainCost: @{plain_cost},
                    swampCost: @{swamp_cost}
                });
            };
            if serialize {
                Path::Serialized(v.try_into().unwrap())
            } else {
                Path::Vectorized(v.try_into().unwrap())
            }
        })
    }

    pub fn look_for_at<T, U>(&self, ty: T, target: U) -> Vec<T::Item>
    where
        T: LookConstant,
        U: HasPosition,
    {
        let pos = target.pos();
        T::convert_and_check_items(js_unwrap!(@{self.as_ref()}.lookForAt(
            __look_num_to_str(@{ty.look_code() as u32}),
            @{pos.as_ref()},
        )))
    }

    pub fn look_for_at_xy<T>(&self, ty: T, x: u32, y: u32) -> Vec<T::Item>
    where
        T: LookConstant,
    {
        T::convert_and_check_items(js_unwrap!(@{self.as_ref()}.lookForAt(
            __look_num_to_str(@{ty.look_code() as u32}),
            @{x},
            @{y},
        )))
    }

    /// Looks for a given thing over a given area of bounds.
    ///
    /// To keep with `Range` convention, the start is inclusive, and the end
    /// is _exclusive_.
    ///
    /// Note: to ease the implementation and efficiency of the rust interface, this is limited to
    /// returning an array of values without their positions. If position data is needed, all room
    /// objects *should* contain positions alongside them. (for terrain data, I would recommend
    /// using a different method?)
    ///
    /// If you really do need more information here, I would recommend making a PR to add it!
    ///
    /// # Panics
    ///
    /// Panics if start>end for either range, or if end>50 for either range.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # let room: ::screeps::Room = unimplemented!();
    /// use screeps::constants::look;
    /// room.look_for_at_area(look::ENERGY, 20..26, 20..26);
    /// ```
    pub fn look_for_at_area<T>(&self, ty: T, horiz: Range<u8>, vert: Range<u8>) -> Vec<T::Item>
    where
        T: LookConstant,
    {
        assert!(horiz.start <= horiz.end);
        assert!(vert.start <= vert.end);
        assert!(horiz.end <= 50);
        assert!(vert.end <= 50);

        T::convert_and_check_items(js_unwrap!{@{self.as_ref()}.lookForAtArea(
            __look_num_to_str(@{ty.look_code() as u32}),
            @{vert.start},
            @{horiz.start},
            @{vert.end},
            @{horiz.end},
            true
        ).map((obj) => obj[__look_num_to_str(@{ty.look_code() as u32})])})
    }

    pub fn memory(&self) -> MemoryReference {
        js_unwrap!(@{self.as_ref()}.memory)
    }

    pub fn name_local(&self) -> LocalRoomName {
        js_unwrap!(@{self.as_ref()}.name)
    }
}

impl PartialEq for Room {
    fn eq(&self, other: &Room) -> bool {
        self.name() == other.name()
    }
}

impl Eq for Room {}

pub struct FindOptions<'a, F>
where
    F: Fn(String, CostMatrix) -> Option<CostMatrix<'a>>,
{
    pub(crate) ignore_creeps: bool,
    pub(crate) ignore_destructible_structures: bool,
    pub(crate) cost_callback: F,
    pub(crate) max_ops: u32,
    pub(crate) heuristic_weight: f64,
    pub(crate) serialize: bool,
    pub(crate) max_rooms: u32,
    pub(crate) range: u32,
    pub(crate) plain_cost: u8,
    pub(crate) swamp_cost: u8,
}

impl Default for FindOptions<'static, fn(String, CostMatrix) -> Option<CostMatrix<'static>>> {
    fn default() -> Self {
        fn cost_matrix(_: String, _: CostMatrix) -> Option<CostMatrix<'static>> {
            None
        }

        // TODO: should we fall back onto the game's default values, or is
        // it alright to copy them here?
        FindOptions {
            ignore_creeps: false,
            ignore_destructible_structures: false,
            cost_callback: cost_matrix,
            max_ops: 2000,
            heuristic_weight: 1.2,
            serialize: false,
            max_rooms: 16,
            range: 0,
            plain_cost: 1,
            swamp_cost: 5,
        }
    }
}

impl FindOptions<'static, fn(String, CostMatrix) -> Option<CostMatrix<'static>>> {
    /// Creates default SearchOptions
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a, F> FindOptions<'a, F>
where
    F: Fn(String, CostMatrix) -> Option<CostMatrix<'a>>,
{
    /// Sets whether the algorithm considers creeps as walkable. Default: False.
    pub fn ignore_creeps(mut self, ignore: bool) -> Self {
        self.ignore_creeps = ignore;
        self
    }

    /// Sets whether the algorithm considers destructible structure as
    /// walkable. Default: False.
    pub fn ignore_destructible_structures(mut self, ignore: bool) -> Self {
        self.ignore_destructible_structures = ignore;
        self
    }

    /// Sets cost callback - default `|_, _| {}`.
    pub fn cost_callback<'b, F2>(self, cost_callback: F2) -> FindOptions<'b, F2>
    where
        F2: Fn(String, CostMatrix) -> Option<CostMatrix<'b>>,
    {
        let FindOptions {
            ignore_creeps,
            ignore_destructible_structures,
            cost_callback: _,
            max_ops,
            heuristic_weight,
            serialize,
            max_rooms,
            range,
            plain_cost,
            swamp_cost,
        } = self;
        FindOptions {
            ignore_creeps,
            ignore_destructible_structures,
            cost_callback,
            max_ops,
            heuristic_weight,
            serialize,
            max_rooms,
            range,
            plain_cost,
            swamp_cost,
        }
    }

    /// Sets maximum ops - default `2000`.
    pub fn max_ops(mut self, ops: u32) -> Self {
        self.max_ops = ops;
        self
    }

    /// Sets heuristic weight - default `1.2`.
    pub fn heuristic_weight(mut self, weight: f64) -> Self {
        self.heuristic_weight = weight;
        self
    }

    /// Sets whether the returned path should be passed to `Room.serializePath`.
    pub fn serialize(mut self, s: bool) -> Self {
        self.serialize = s;
        self
    }

    /// Sets maximum rooms - default `16`, max `16`.
    pub fn max_rooms(mut self, rooms: u32) -> Self {
        self.max_rooms = rooms;
        self
    }

    pub fn range(mut self, k: u32) -> Self {
        self.range = k;
        self
    }

    /// Sets plain cost - default `1`.
    pub fn plain_cost(mut self, cost: u8) -> Self {
        self.plain_cost = cost;
        self
    }

    /// Sets swamp cost - default `5`.
    pub fn swamp_cost(mut self, cost: u8) -> Self {
        self.swamp_cost = cost;
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Step {
    x: u32,
    y: u32,
    dx: i32,
    dy: i32,
    direction: Direction,
}

js_deserializable!{Step}
js_serializable!{Step}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Path {
    Vectorized(Vec<Step>),
    Serialized(String),
}

js_deserializable!{Path}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {
    pub event: EventType,
    pub object_id: String,
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "camelCase")]
        enum Field {
            Event,
            ObjectId,
            Data,
        };

        struct EventVisitor;

        impl<'de> Visitor<'de> for EventVisitor {
            type Value = Event;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Event")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Event, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut event_type = None;
                let mut obj_id = None;
                let mut data = None;
                let mut data_buffer: Option<serde_json::Value> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Event => {
                            if event_type.is_some() {
                                return Err(de::Error::duplicate_field("event"));
                            }
                            event_type = Some(map.next_value()?);
                        }
                        Field::ObjectId => {
                            if obj_id.is_some() {
                                return Err(de::Error::duplicate_field("objectId"));
                            }
                            obj_id = Some(map.next_value()?);
                        }
                        Field::Data => {
                            if data.is_some() {
                                return Err(de::Error::duplicate_field("data"));
                            }

                            match event_type {
                                None => data_buffer = map.next_value()?,
                                Some(event_id) => {
                                    data = match event_id {
                                        1 => Some(EventType::Attack(map.next_value()?)),
                                        2 => Some(EventType::ObjectDestroyed(map.next_value()?)),
                                        3 => Some(EventType::AttackController),
                                        4 => Some(EventType::Build(map.next_value()?)),
                                        5 => Some(EventType::Harvest(map.next_value()?)),
                                        6 => Some(EventType::Heal(map.next_value()?)),
                                        7 => Some(EventType::Repair(map.next_value()?)),
                                        8 => Some(EventType::ReserveController(map.next_value()?)),
                                        9 => Some(EventType::UpgradeController(map.next_value()?)),
                                        10 => Some(EventType::Exit(map.next_value()?)),
                                        _ => {
                                            return Err(de::Error::custom(format!(
                                                "Event Type Unrecognized: {}",
                                                event_id
                                            )))
                                        }
                                    };
                                }
                            };
                        }
                    }
                }

                if data.is_none() {
                    let err = |e| {
                        de::Error::custom(format_args!(
                            "can't parse event data due to inner error {}",
                            e
                        ))
                    };

                    if let (Some(val), Some(event_id)) = (data_buffer, event_type) {
                        data = match event_id {
                            1 => Some(EventType::Attack(serde_json::from_value(val).map_err(err)?)),
                            2 => Some(EventType::ObjectDestroyed(
                                serde_json::from_value(val).map_err(err)?,
                            )),
                            3 => Some(EventType::AttackController),
                            4 => Some(EventType::Build(serde_json::from_value(val).map_err(err)?)),
                            5 => Some(EventType::Harvest(
                                serde_json::from_value(val).map_err(err)?,
                            )),
                            6 => Some(EventType::Heal(serde_json::from_value(val).map_err(err)?)),
                            7 => Some(EventType::Repair(serde_json::from_value(val).map_err(err)?)),
                            8 => Some(EventType::ReserveController(
                                serde_json::from_value(val).map_err(err)?,
                            )),
                            9 => Some(EventType::UpgradeController(
                                serde_json::from_value(val).map_err(err)?,
                            )),
                            10 => Some(EventType::Exit(serde_json::from_value(val).map_err(err)?)),
                            _ => {
                                return Err(de::Error::custom(format!(
                                    "Event Type Unrecognized: {}",
                                    event_id
                                )))
                            }
                        };
                    }
                }

                let data = data.ok_or_else(|| de::Error::missing_field("data"))?;
                let obj_id = obj_id.ok_or_else(|| de::Error::missing_field("objectId"))?;

                Ok(Event {
                    event: data,
                    object_id: obj_id,
                })
            }
        }

        const FIELDS: &[&str] = &["event", "objectId", "data"];
        deserializer.deserialize_struct("Event", FIELDS, EventVisitor)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventType {
    Attack(AttackEvent),
    ObjectDestroyed(ObjectDestroyedEvent),
    AttackController,
    Build(BuildEvent),
    Harvest(HarvestEvent),
    Heal(HealEvent),
    Repair(RepairEvent),
    ReserveController(ReserveControllerEvent),
    UpgradeController(UpgradeControllerEvent),
    Exit(ExitEvent),
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackEvent {
    pub target_id: String,
    pub damage: u32,
    pub attack_type: AttackType,
}

enum_number!(AttackType {
    Melee = 1,
    Ranged = 2,
    RangedMass = 3,
    Dismantle = 4,
    HitBack = 5,
    Nuke = 6,
});

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct ObjectDestroyedEvent {
    #[serde(rename = "type")]
    pub object_type: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildEvent {
    pub target_id: String,
    pub amount: u32,
    pub energy_spent: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HarvestEvent {
    pub target_id: String,
    pub amount: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealEvent {
    pub target_id: String,
    pub amount: u32,
    pub heal_type: HealType,
}

enum_number!(HealType {
    Melee = 1,
    Ranged = 2,
});

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepairEvent {
    pub target_id: String,
    pub amount: u32,
    pub energy_spent: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReserveControllerEvent {
    pub amount: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeControllerEvent {
    pub amount: u32,
    pub energy_spent: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExitEvent {
    pub room: String,
    pub x: u32,
    pub y: u32,
}

pub enum LookResult {
    Creep(Creep),
    Energy(Resource),
    Resource(Resource),
    Source(Source),
    Mineral(Mineral),
    Structure(Structure),
    Flag(Flag),
    ConstructionSite(ConstructionSite),
    Nuke(Nuke),
    Terrain(Terrain),
    Tombstone(Tombstone),
}

impl TryFrom<Value> for LookResult {
    type Error = ConversionError;

    fn try_from(v: Value) -> Result<LookResult, Self::Error> {
        let look_type: String = js!(return @{&v}.type;).try_into()?;

        let lr = match look_type.as_ref() {
            "creep" => LookResult::Creep(js_unwrap_ref!(@{v}.creep)),
            "energy" => LookResult::Energy(js_unwrap_ref!(@{v}.energy)),
            "resource" => LookResult::Resource(js_unwrap_ref!(@{v}.resource)),
            "source" => LookResult::Source(js_unwrap_ref!(@{v}.source)),
            "mineral" => LookResult::Mineral(js_unwrap_ref!(@{v}.mineral)),
            "structure" => LookResult::Structure(js_unwrap_ref!(@{v}.structure)),
            "flag" => LookResult::Flag(js_unwrap_ref!(@{v}.flag)),
            "constructionSite" => {
                LookResult::ConstructionSite(js_unwrap_ref!(@{v}.constructionSite))
            }
            "nuke" => LookResult::Nuke(js_unwrap_ref!(@{v}.nuke)),
            "terrain" => LookResult::Terrain(js_unwrap!(@{v}.terrain)),
            "tombstone" => LookResult::Tombstone(js_unwrap_ref!(@{v}.tombstone)),
            _ => {
                return Err(ConversionError::Custom(format!(
                    "Look result type unknown: {:?}",
                    &look_type
                )))
            }
        };
        Ok(lr)
    }
}

pub struct PositionedLookResult {
    pub x: u32,
    pub y: u32,
    pub look_result: LookResult,
}

impl TryFrom<Value> for PositionedLookResult {
    type Error = ConversionError;

    fn try_from(v: Value) -> Result<PositionedLookResult, Self::Error> {
        let x: u32 = js!(return @{&v}.x;).try_into()?;
        let y: u32 = js!(return @{&v}.y;).try_into()?;
        let look_result: LookResult = v.try_into()?;

        Ok(PositionedLookResult { x, y, look_result })
    }
}
