use std::{fmt, marker::PhantomData, mem, ops::Range};

use num_traits::FromPrimitive;
use serde::{
    self,
    de::{self, Deserializer, MapAccess, Visitor},
    Deserialize, Serialize,
};
use serde_json;
use serde_repr::{Deserialize_repr, Serialize_repr};
use stdweb::{Reference, Value};

use crate::{
    constants::{
        Color, Direction, EffectType, ExitDirection, FindConstant, Look, LookConstant, PowerType,
        ResourceType, ReturnCode, StructureType, Terrain,
    },
    local::{Position, RoomName},
    memory::MemoryReference,
    objects::{
        ConstructionSite, Creep, Deposit, Flag, HasPosition, Mineral, Nuke, PowerCreep, Resource,
        Room, RoomTerrain, RoomVisual, Ruin, Source, Structure, StructureController,
        StructureStorage, StructureTerminal, Tombstone,
    },
    pathfinder::{CostMatrix, RoomCostResult, SingleRoomCostResult},
    traits::{TryFrom, TryInto},
    ConversionError,
};

simple_accessors! {
    impl Room {
        pub fn controller() -> Option<StructureController> = controller;
        pub fn energy_available() -> u32 = energyAvailable;
        pub fn energy_capacity_available() -> u32 = energyCapacityAvailable;
        pub fn name() -> RoomName = name;
        pub fn storage() -> Option<StructureStorage> = storage;
        pub fn terminal() -> Option<StructureTerminal> = terminal;
    }
}

impl Room {
    pub fn serialize_path(path: &[Step]) -> String {
        js_unwrap! {Room.serializePath(@{path})}
    }

    pub fn deserialize_path(path: &str) -> Vec<Step> {
        js_unwrap! {Room.deserializePath(@{path})}
    }

    pub fn create_construction_site<T>(&self, at: &T, ty: StructureType) -> ReturnCode
    where
        T: ?Sized + HasPosition,
    {
        let pos = at.pos();
        js_unwrap!(@{self.as_ref()}.createConstructionSite(
            pos_from_packed(@{pos.packed_repr()}),
            __structure_type_num_to_str(@{ty as u32})
        ))
    }

    pub fn create_named_construction_site<T>(
        &self,
        at: &T,
        ty: StructureType,
        name: &str,
    ) -> ReturnCode
    where
        T: ?Sized + HasPosition,
    {
        let pos = at.pos();
        js_unwrap!(@{self.as_ref()}.createConstructionSite(
            // pos_from_packed(@{pos.packed_repr()}),
            // workaround - passing with a position and a name
            // currently broken, use x,y instead
            @{pos.x()},
            @{pos.y()},
            __structure_type_num_to_str(@{ty as u32}),
            @{name}
        ))
    }

    pub fn create_flag<T>(
        &self,
        at: &T,
        name: &str,
        main_color: Color,
        secondary_color: Color,
    ) -> Result<String, ReturnCode>
    where
        T: ?Sized + HasPosition,
    {
        let pos = at.pos();
        Flag::interpret_creation_ret_value(js! {
            return @{self.as_ref()}.createFlag(
                pos_from_packed(@{pos.packed_repr()}),
                @{name},
                @{main_color as u32},
                @{secondary_color as u32}
            );
        })
        .expect("expected Room.createFlag to return ReturnCode or String name")
    }

    pub fn find<T>(&self, ty: T) -> Vec<T::Item>
    where
        T: FindConstant,
    {
        js_unwrap_ref!(@{self.as_ref()}.find(@{ty.find_code()}))
    }

    pub fn find_exit_to(&self, room: &Room) -> Result<ExitDirection, ReturnCode> {
        let code_val = js! {return @{self.as_ref()}.findExitTo(@{room.as_ref()});};
        let code_int: i32 = code_val.try_into().unwrap();

        if code_int < 0 {
            Err(ReturnCode::from_i32(code_int)
                .expect("expected find_exit_to return value < 0 to be a valid ReturnCode"))
        } else {
            Ok(ExitDirection::from_i32(code_int)
                .expect("expected find_exit_to return value >= 0 to be a valid Exit"))
        }
    }

    pub fn get_event_log(&self) -> Vec<Event> {
        serde_json::from_str(&self.get_event_log_raw()).expect("Malformed Event Log")
    }

    pub fn get_event_log_raw(&self) -> String {
        js_unwrap! {@{self.as_ref()}.getEventLog(true)}
    }

    pub fn get_position_at(&self, x: u32, y: u32) -> Option<Position> {
        let v = js! {
            let value = @{self.as_ref()}.getPositionAt(@{x}, @{y});
            if (value == null) {
                return null;
            } else {
                return value.__packedPos;
            }
        };
        match v {
            Value::Number(_) => Some(
                v.try_into()
                    .expect("expected Position::try_from(pos.__packedPos) to succeed"),
            ),
            Value::Null => None,
            _ => panic!(
                "unexpected return value for JS binding to Room.getPositionAt. \
                 expected null or number, found {:?}",
                v
            ),
        }
    }

    pub fn get_terrain(&self) -> RoomTerrain {
        js_unwrap!(@{self.as_ref()}.getTerrain())
    }

    pub fn look_at<T: ?Sized + HasPosition>(&self, target: &T) -> Vec<LookResult> {
        let pos = target.pos();
        js_unwrap!(@{self.as_ref()}.lookAt(pos_from_packed(@{pos.packed_repr()})))
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

    pub fn find_path<'a, 's, O, T, F>(
        &'s self,
        from_pos: &O,
        to_pos: &T,
        opts: FindOptions<'a, F, SingleRoomCostResult<'a>>,
    ) -> Path
    where
        O: ?Sized + HasPosition,
        T: ?Sized + HasPosition,
        F: FnMut(RoomName, CostMatrix<'a>) -> SingleRoomCostResult<'a> + 'a + 's,
    {
        let from = from_pos.pos();
        let to = to_pos.pos();

        let mut raw_callback = opts.cost_callback;

        let mut callback_boxed = move |room_name: RoomName, cost_matrix_ref: Reference| -> Value {
            let cmatrix = CostMatrix {
                inner: cost_matrix_ref,
                lifetime: PhantomData,
            };

            raw_callback(room_name, cmatrix).into()
        };

        // Type erased and boxed callback: no longer a type specific to the closure
        // passed in, now unified as &Fn
        let callback_type_erased: &mut (dyn FnMut(RoomName, Reference) -> Value + 'a) =
            &mut callback_boxed;

        // Overwrite lifetime of reference so it can be passed to javascript.
        // It's now pretending to be static data. This should be entirely safe
        // because we control the only use of it and it remains valid during the
        // pathfinder callback. This transmute is necessary because "some lifetime
        // above the current scope but otherwise unknown" is not a valid lifetime.
        let callback_lifetime_erased: &'static mut dyn FnMut(RoomName, Reference) -> Value =
            unsafe { mem::transmute(callback_type_erased) };

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

        let v = js! {
            let cb = @{callback_lifetime_erased};
            let res = @{&self.as_ref()}.findPath(
                pos_from_packed(@{from.packed_repr()}),
                pos_from_packed(@{to.packed_repr()}),
                {
                    ignoreCreeps: @{ignore_creeps},
                    ignoreDestructibleStructures: @{ignore_destructible_structures},
                    costCallback: cb,
                    maxOps: @{max_ops},
                    heuristicWeight: @{heuristic_weight},
                    serialize: @{serialize},
                    maxRooms: @{max_rooms},
                    range: @{range},
                    plainCost: @{plain_cost},
                    swampCost: @{swamp_cost}
                }
            );
            cb.drop();
            return res;
        };

        if serialize {
            Path::Serialized(v.try_into().unwrap())
        } else {
            Path::Vectorized(v.try_into().unwrap())
        }
    }

    pub fn look_for_at<T, U>(&self, ty: T, target: &U) -> Vec<T::Item>
    where
        T: LookConstant,
        U: HasPosition,
    {
        let pos = target.pos();
        T::convert_and_check_items(js_unwrap!(@{self.as_ref()}.lookForAt(
            __look_num_to_str(@{ty.look_code() as u32}),
            pos_from_packed(@{pos.packed_repr()}),
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
    /// Note: to ease the implementation and efficiency of the rust interface,
    /// this is limited to returning an array of values without their
    /// positions. If position data is needed, all room objects *should*
    /// contain positions alongside them. (for terrain data, I would recommend
    /// using a different method?)
    ///
    /// If you really do need more information here, I would recommend making a
    /// PR to add it!
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

        T::convert_and_check_items(js_unwrap! {@{self.as_ref()}.lookForAtArea(
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

    pub fn name_local(&self) -> RoomName {
        js_unwrap!(@{self.as_ref()}.name)
    }

    pub fn visual(&self) -> RoomVisual {
        RoomVisual::new(Some(self.name()))
    }
}

impl PartialEq for Room {
    fn eq(&self, other: &Room) -> bool {
        self.name() == other.name()
    }
}

impl Eq for Room {}

pub struct FindOptions<'a, F, R>
where
    F: FnMut(RoomName, CostMatrix<'a>) -> R,
    R: RoomCostResult,
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
    pub(crate) phantom: PhantomData<&'a ()>,
}

impl<'a, R> Default for FindOptions<'a, fn(RoomName, CostMatrix<'a>) -> R, R>
where
    R: RoomCostResult + Default,
{
    fn default() -> Self {
        // TODO: should we fall back onto the game's default values, or is
        // it alright to copy them here?
        FindOptions {
            ignore_creeps: false,
            ignore_destructible_structures: false,
            cost_callback: |_, _| R::default(),
            max_ops: 2000,
            heuristic_weight: 1.2,
            serialize: false,
            max_rooms: 16,
            range: 0,
            plain_cost: 1,
            swamp_cost: 5,
            phantom: PhantomData,
        }
    }
}

impl<'a, R> FindOptions<'a, fn(RoomName, CostMatrix<'a>) -> R, R>
where
    R: RoomCostResult + Default,
{
    /// Creates default SearchOptions
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a, F, R> FindOptions<'a, F, R>
where
    F: FnMut(RoomName, CostMatrix<'a>) -> R,
    R: RoomCostResult,
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
    pub fn cost_callback<'b, F2, R2>(self, cost_callback: F2) -> FindOptions<'b, F2, R2>
    where
        F2: FnMut(RoomName, CostMatrix<'b>) -> R2,
        R2: RoomCostResult,
    {
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
            phantom: PhantomData,
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
    pub x: u32,
    pub y: u32,
    pub dx: i32,
    pub dy: i32,
    pub direction: Direction,
}

js_deserializable! {Step}
js_serializable! {Step}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Path {
    Vectorized(Vec<Step>),
    Serialized(String),
}

js_deserializable! {Path}

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

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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
                                        11 => Some(EventType::Power(map.next_value()?)),
                                        12 => Some(EventType::Transfer(map.next_value()?)),
                                        _ => {
                                            return Err(de::Error::custom(format!(
                                                "Event Type Unrecognized: {}",
                                                event_id
                                            )));
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
                            11 => Some(EventType::Power(serde_json::from_value(val).map_err(err)?)),
                            12 => Some(EventType::Transfer(
                                serde_json::from_value(val).map_err(err)?,
                            )),
                            _ => {
                                return Err(de::Error::custom(format!(
                                    "Event Type Unrecognized: {}",
                                    event_id
                                )));
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
    Power(PowerEvent),
    Transfer(TransferEvent),
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackEvent {
    pub target_id: String,
    pub damage: u32,
    pub attack_type: AttackType,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum AttackType {
    Melee = 1,
    Ranged = 2,
    RangedMass = 3,
    Dismantle = 4,
    HitBack = 5,
    Nuke = 6,
}

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
    // energySpent is documented as being present, but isn't in my testing
    //pub energy_spent: u32,
    #[serde(deserialize_with = "crate::StructureType::deserialize_from_str")]
    pub structure_type: StructureType,
    pub x: u8,
    pub y: u8,
    pub incomplete: bool,
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

#[derive(Clone, Debug, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum HealType {
    Melee = 1,
    Ranged = 2,
}

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

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferEvent {
    pub target_id: String,
    #[serde(deserialize_with = "crate::ResourceType::deserialize_from_str")]
    pub resource_type: ResourceType,
    pub amount: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerEvent {
    pub target_id: String,
    pub power: PowerType,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    pub effect: EffectType,
    pub level: Option<u8>,
    pub ticks_remaining: u32,
}
js_deserializable! {Effect}

pub enum LookResult {
    Creep(Creep),
    Energy(Resource),
    Resource(Resource),
    Source(Source),
    Mineral(Mineral),
    Deposit(Deposit),
    Structure(Structure),
    Flag(Flag),
    ConstructionSite(ConstructionSite),
    Nuke(Nuke),
    Terrain(Terrain),
    Tombstone(Tombstone),
    PowerCreep(PowerCreep),
    Ruin(Ruin),
}

impl TryFrom<Value> for LookResult {
    type Error = ConversionError;

    fn try_from(v: Value) -> Result<LookResult, Self::Error> {
        let look_type = js! (
            return __look_str_to_num(@{&v}.type);
        )
        .try_into()?;

        let lr = match look_type {
            Look::Creeps => LookResult::Creep(js_unwrap_ref!(@{v}.creep)),
            Look::Energy => LookResult::Energy(js_unwrap_ref!(@{v}.energy)),
            Look::Resources => LookResult::Resource(js_unwrap_ref!(@{v}.resource)),
            Look::Sources => LookResult::Source(js_unwrap_ref!(@{v}.source)),
            Look::Minerals => LookResult::Mineral(js_unwrap_ref!(@{v}.mineral)),
            Look::Deposits => LookResult::Deposit(js_unwrap_ref!(@{v}.deposit)),
            Look::Structures => LookResult::Structure(js_unwrap_ref!(@{v}.structure)),
            Look::Flags => LookResult::Flag(js_unwrap_ref!(@{v}.flag)),
            Look::ConstructionSites => {
                LookResult::ConstructionSite(js_unwrap_ref!(@{v}.constructionSite))
            }
            Look::Nukes => LookResult::Nuke(js_unwrap_ref!(@{v}.nuke)),
            Look::Terrain => LookResult::Terrain(js_unwrap!(__terrain_str_to_num(@{v}.terrain))),
            Look::Tombstones => LookResult::Tombstone(js_unwrap_ref!(@{v}.tombstone)),
            Look::PowerCreeps => LookResult::PowerCreep(js_unwrap_ref!(@{v}.powerCreep)),
            Look::Ruins => LookResult::Ruin(js_unwrap_ref!(@{v}.ruin)),
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
