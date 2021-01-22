use crate::{
    objects::{OwnedStructure, Owner, Room, RoomObject, RoomPosition, Structure},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureRampart`], which is selectively
    /// walkable and protects creeps and structures at the same position.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureRampart)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureRampart;

    /// Whether the [`StructureRampart`] is set to be public, allowing hostile
    /// creeps to walk on it.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureRampart.isPublic)
    #[wasm_bindgen(method, getter = isPublic)]
    pub fn is_public(this: &StructureRampart) -> bool;

    /// The number of ticks until the rampart will decay, losing
    /// [`RAMPART_DECAY_AMOUNT`] hits.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureRampart.ticksToDecay)
    ///
    /// [`RAMPART_DECAY_AMOUNT`]:
    /// crate::constants::numbers::RAMPART_DECAY_AMOUNT
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &StructureRampart) -> u32;

    /// Set whether [`StructureRampart`] is public, allowing hostile creeps to
    /// walk on it.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureRampart.setPublic)
    #[wasm_bindgen(method, js_name = setPublic)]
    pub fn set_public(this: &StructureRampart, val: bool) -> i8;
}

impl Attackable for StructureRampart {
    fn hits(&self) -> u32 {
        Structure::hits(self.as_ref())
    }

    fn hits_max(&self) -> u32 {
        Structure::hits_max(self.as_ref())
    }
}
impl CanDecay for StructureRampart {
    fn ticks_to_decay(&self) -> u32 {
        Self::ticks_to_decay(self)
    }
}

impl HasId for StructureRampart {
    fn id(&self) -> Option<JsString> {
        Some(Structure::id(self.as_ref()))
    }
}

impl HasPosition for StructureRampart {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl OwnedStructureProperties for StructureRampart {
    fn my(&self) -> bool {
        OwnedStructure::my(self.as_ref())
    }

    fn owner(&self) -> Option<Owner> {
        OwnedStructure::owner(self.as_ref())
    }
}
impl RoomObjectProperties for StructureRampart {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
impl StructureProperties for StructureRampart {}
