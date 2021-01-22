use crate::{
    objects::{OwnedStructure, Owner, Room, RoomObject, RoomPosition, Structure},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureInvaderCore`], which is at the
    /// center of NPC strongholds, as well as reserving neutral rooms.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureInvaderCore)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureInvaderCore;

    /// The level of the [`StructureInvaderCore`]; 0 is a lesser invader core
    /// that simply reserves rooms, while levels 1-5 are strongholds which
    /// defend themselves.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureInvaderCore.level)
    #[wasm_bindgen(method, getter)]
    pub fn level(this: &StructureInvaderCore) -> u8;

    /// The number of ticks until the [`StructureInvaderCore`] is fully deployed
    /// and can be attacked.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureInvaderCore.ticksToDeploy)
    #[wasm_bindgen(method, getter = ticksToDeploy)]
    pub fn ticks_to_deploy(this: &StructureInvaderCore) -> u32;
}

impl Attackable for StructureInvaderCore {
    fn hits(&self) -> u32 {
        Structure::hits(self.as_ref())
    }

    fn hits_max(&self) -> u32 {
        Structure::hits_max(self.as_ref())
    }
}
impl HasId for StructureInvaderCore {
    fn id(&self) -> Option<JsString> {
        Some(Structure::id(self.as_ref()))
    }
}
impl HasPosition for StructureInvaderCore {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl OwnedStructureProperties for StructureInvaderCore {
    fn my(&self) -> bool {
        OwnedStructure::my(self.as_ref())
    }

    fn owner(&self) -> Option<Owner> {
        OwnedStructure::owner(self.as_ref())
    }
}
impl RoomObjectProperties for StructureInvaderCore {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
impl StructureProperties for StructureInvaderCore {}
