use crate::{
    objects::{Room, RoomObject, RoomPosition, Structure},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructurePowerBank`], which can be destroyed
    /// for power resources.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePowerBank)
    #[wasm_bindgen(extends = RoomObject, extends = Structure)]
    pub type StructurePowerBank;

    /// The amount of power contained within the [`StructurePowerBank`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePowerBank.power)
    #[wasm_bindgen(method, getter)]
    pub fn power(this: &StructurePowerBank) -> u32;

    /// The number of ticks until the [`StructurePowerBank`] will decay.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePowerBank.ticksToDecay)
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &StructurePowerBank) -> u32;
}

impl Attackable for StructurePowerBank {
    fn hits(&self) -> u32 {
        Structure::hits(self.as_ref())
    }

    fn hits_max(&self) -> u32 {
        Structure::hits_max(self.as_ref())
    }
}
impl CanDecay for StructurePowerBank {
    fn ticks_to_decay(&self) -> u32 {
        Self::ticks_to_decay(self)
    }
}

impl HasId for StructurePowerBank {
    fn id(&self) -> Option<JsString> {
        Some(Structure::id(self.as_ref()))
    }
}
impl HasPosition for StructurePowerBank {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl RoomObjectProperties for StructurePowerBank {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
impl StructureProperties for StructurePowerBank {}
