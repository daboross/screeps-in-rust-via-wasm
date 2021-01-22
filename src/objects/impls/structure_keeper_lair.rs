use crate::{
    objects::{OwnedStructure, Owner, Room, RoomObject, RoomPosition, Structure},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureKeeperLair`], which regularly spawns
    /// creeps to defend nearby resources.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureKeeperLair)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureKeeperLair;

    /// The number of ticks until the [`StructureKeeperLair`] will spawn a new
    /// creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureKeeperLair.ticksToSpawn)
    #[wasm_bindgen(method, getter = ticksToSpawn)]
    pub fn ticks_to_spawn(this: &StructureKeeperLair) -> u32;
}

impl HasId for StructureKeeperLair {
    fn id(&self) -> Option<JsString> {
        Some(Structure::id(self.as_ref()))
    }
}
impl HasPosition for StructureKeeperLair {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl OwnedStructureProperties for StructureKeeperLair {
    fn my(&self) -> bool {
        OwnedStructure::my(self.as_ref())
    }

    fn owner(&self) -> Option<Owner> {
        OwnedStructure::owner(self.as_ref())
    }
}
impl RoomObjectProperties for StructureKeeperLair {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
impl StructureProperties for StructureKeeperLair {}
