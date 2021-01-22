use crate::{
    constants::{Color, Direction, Look, StructureType},
    local::Position,
    prelude::*,
    prototypes::ROOM_POSITION_PROTOTYPE,
};
use js_sys::{Array, JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a position in a room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition)
    pub type RoomPosition;

    /// Create a new RoomPosition using the normal constructor, taking
    /// coordinates and the room name.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.constructor)
    #[wasm_bindgen(constructor)]
    pub fn new(x: u8, y: u8, room_name: &JsString) -> RoomPosition;

    /// Name of the room the position is in, as an owned [`JsString`] reference
    /// to a string in Javascript memory.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.roomName)
    #[wasm_bindgen(method, getter = roomName)]
    pub fn room_name(this: &RoomPosition) -> JsString;

    /// Change the room the position refers to; must be a valid room name.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.roomName)
    #[wasm_bindgen(method, setter = roomName)]
    pub fn set_room_name(this: &RoomPosition) -> JsString;

    /// X coordinate of the position within the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.x)
    #[wasm_bindgen(method, getter)]
    pub fn x(this: &RoomPosition) -> u8;

    /// Set a new X coordinate; must be in the range 0..49.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.x)
    #[wasm_bindgen(method, setter)]
    pub fn set_x(this: &RoomPosition) -> u8;

    /// Y coordinate of the position within the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.y)
    #[wasm_bindgen(method, getter)]
    pub fn y(this: &RoomPosition) -> u8;

    /// Set a new Y coordinate; must be in the range 0..49.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.y)
    #[wasm_bindgen(method, setter)]
    pub fn set_y(this: &RoomPosition) -> u8;

    // todo, get this as native
    /// Gets the efficient internal i32 representation of the position.
    #[wasm_bindgen(method, getter = __packedPos)]
    pub fn packed(this: &RoomPosition) -> i32;

    // todo, as native
    /// Sets the position to a new one by passing an i32 that represents a
    /// packed position.
    #[wasm_bindgen(method, setter = __packedPos)]
    pub fn set_packed(this: &RoomPosition, val: i32);

    /// Creates a [`ConstructionSite`] at this position. If it's a
    /// [`StructureSpawn`], a name can optionally be assigned for the structure.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.createConstructionSite)
    ///
    /// [`ConstructionSite`]: crate::objects::ConstructionSite
    #[wasm_bindgen(method, js_name = createConstructionSite)]
    pub fn create_construction_site(
        this: &RoomPosition,
        ty: StructureType,
        name: Option<&JsString>,
    ) -> i8;

    // todo we need to handle the fact that if this succeeds the name of the flag is returned, and maybe also the fact
    // that it'll throw a js exception when created in a non visible room.. hmm
    /// Creates a [`Flag`] at this position.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.createFlag)
    ///
    /// [`Flag`]: crate::objects::Flag
    #[wasm_bindgen(method, js_name = createFlag)]
    pub fn create_flag(
        this: &RoomPosition,
        name: Option<&JsString>,
        color: Option<Color>,
        secondary_color: Option<Color>,
    ) -> i8;

    // todo FindOptions
    /// Find the closest object by path among an [`Array`] of objects, or among
    /// a [`FindType`] to search for all objects of that type in the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findClosestByPath)
    ///
    /// [`Flag`]: crate::objects::Flag
    #[wasm_bindgen(method, js_name = findClosestByPath)]
    pub fn find_closest_by_path(
        this: &RoomPosition,
        goal: &JsValue,
        options: Option<&Object>,
    ) -> Option<Object>;

    // todo FindOptions
    /// Find the closest object by range among an [`Array`] of objects, or among
    /// a [`FindType`] to search for all objects of that type in the room. Will
    /// not work for objects in other rooms.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findClosestByRange)
    #[wasm_bindgen(method, js_name = findClosestByRange)]
    pub fn find_closest_by_range(
        this: &RoomPosition,
        goal: &JsValue,
        options: Option<&Object>,
    ) -> Option<Object>;

    // todo FindOptions
    /// Find all relevant objects within a certain range among an [`Array`] of
    /// objects, or among a [`FindType`] to search all objects of that type in
    /// the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findInRange)
    #[wasm_bindgen(method, js_name = findInRange)]
    pub fn find_in_range(
        this: &RoomPosition,
        goal: &JsValue,
        range: u8,
        options: Option<&Object>,
    ) -> Array;

    // todo FindPathOptions
    /// Find a path from this position to a position or room object, with an
    /// optional options object
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findPathTo)
    #[wasm_bindgen(method, js_name = findPathTo)]
    pub fn find_path_to(this: &RoomPosition, goal: &JsValue, options: Option<&Object>) -> Array;

    // todo FindPathOptions
    /// Find a path from this position to the given coordinates in the same
    /// room, with an optional options object.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findPathTo)
    #[wasm_bindgen(method, js_name = findPathTo)]
    pub fn find_path_to_xy(this: &RoomPosition, x: u8, y: u8, options: Option<&Object>) -> Array;

    /// Get the [`Direction`] toward a position or room object.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.getDirectionTo)
    #[wasm_bindgen(method, js_name = getDirectionTo)]
    pub fn get_direction_to(this: &RoomPosition, goal: &JsValue) -> Direction;

    /// Get the [`Direction`] toward the given coordinates in the same room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.getDirectionTo)
    #[wasm_bindgen(method, js_name = getDirectionTo)]
    pub fn get_direction_to_xy(this: &RoomPosition, x: u8, y: u8) -> Direction;

    /// Get the range to a position or room object in the same room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.getRangeTo)
    #[wasm_bindgen(method, js_name = getRangeTo)]
    pub fn get_range_to(this: &RoomPosition, goal: &JsValue) -> u32;

    /// Get the range to the given coordinates in the same room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.getRangeTo)
    #[wasm_bindgen(method, js_name = getRangeTo)]
    pub fn get_range_to_xy(this: &RoomPosition, x: u8, y: u8) -> u32;

    /// Get the range to a position or room object in the same room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.inRangeTo)
    #[wasm_bindgen(method, js_name = inRangeTo)]
    pub fn in_range_to(this: &RoomPosition, goal: &JsValue, range: u8) -> bool;

    /// Get the range to the given coordinates in the same room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.inRangeTo)
    #[wasm_bindgen(method, js_name = inRangeTo)]
    pub fn in_range_to_xy(this: &RoomPosition, x: u8, y: u8, range: u8) -> bool;

    /// Determine whether this position is at the same position as another
    /// position or room object.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.isEqualTo)
    #[wasm_bindgen(method, js_name = isEqualTo)]
    pub fn is_equal_to(this: &RoomPosition, goal: &JsValue) -> bool;

    /// Determine whether this position is at the given coordinates in the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.isEqualTo)
    #[wasm_bindgen(method, js_name = isEqualTo)]
    pub fn is_equal_to_xy(this: &RoomPosition, x: u8, y: u8) -> bool;

    /// Determine whether this position is within 1 range of another position or
    /// room object.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.isNearTo)
    #[wasm_bindgen(method, js_name = isNearTo)]
    pub fn is_near_to(this: &RoomPosition, goal: &JsValue) -> bool;

    /// Determine whether this position is within 1 range of the given
    /// coordinates in the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.isNearTo)
    #[wasm_bindgen(method, js_name = isNearTo)]
    pub fn is_near_to_xy(this: &RoomPosition, x: u8, y: u8) -> bool;

    /// Get an array of all objects at this position.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.look)
    #[wasm_bindgen(method)]
    pub fn look(this: &RoomPosition) -> Array;

    /// Get an array of all objects of a given type at this position, if any.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.lookFor)
    #[wasm_bindgen(method, js_name = lookFor)]
    pub fn look_for(this: &RoomPosition, ty: Look) -> Option<Array>;
}

impl HasPosition for RoomPosition {
    fn pos(&self) -> Option<Self> {
        let new_pos = RoomPosition::from(JsValue::from(Object::create(&ROOM_POSITION_PROTOTYPE)));
        new_pos.set_packed(self.packed());
        Some(new_pos)
    }
}

impl From<Position> for RoomPosition {
    fn from(pos: Position) -> Self {
        let js_pos = RoomPosition::from(JsValue::from(Object::create(&ROOM_POSITION_PROTOTYPE)));
        js_pos.set_packed(pos.packed_repr());
        js_pos
    }
}
