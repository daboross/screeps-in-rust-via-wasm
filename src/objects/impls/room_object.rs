use crate::{
    objects::{Room, RoomPosition},
    prelude::*,
};
use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Parent class for all objects in rooms in the game world.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room)
    pub type RoomObject;

    /// Effects applied to the object.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomObject.effects)
    #[wasm_bindgen(method, getter)]
    pub fn effects(this: &RoomObject) -> Array;

    /// Position of the object, or `None` if the object is a power creep not
    /// spawned on the current shard.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomObject.pos)
    #[wasm_bindgen(method, getter)]
    pub fn pos(this: &RoomObject) -> Option<RoomPosition>;

    /// A link to the room that the object is currently in, or `None` if the
    /// object is a power creep not spawned on the current shard, or a flag or
    /// construction site not in a visible room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomObject.room)
    #[wasm_bindgen(method, getter)]
    pub fn room(this: &RoomObject) -> Option<Room>;
}

impl HasPosition for RoomObject {
    fn pos(&self) -> Option<RoomPosition> {
        Self::pos(self.as_ref())
    }
}
impl RoomObjectProperties for RoomObject {
    fn effects(&self) -> Array {
        Self::effects(self)
    }

    fn room(&self) -> Option<Room> {
        Self::room(self)
    }
}
