use crate::{
    objects::{Room, RoomObject, RoomPosition, Store, Structure},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureContainer`], which can store
    /// resources and does not block creep movement, but requires regular repair
    /// due to decay.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureContainer)
    #[wasm_bindgen(extends = RoomObject, extends = Structure)]
    pub type StructureContainer;

    /// The [`Store`] of the container, which contains information about what
    /// resources it is it holding.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureContainer.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureContainer) -> Store;

    /// The number of ticks until the container will decay, losing TODO CONSTANT
    /// hits.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureContainer.ticksToDecay)
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &StructureContainer) -> u32;
}

impl Attackable for StructureContainer {
    fn hits(&self) -> u32 {
        Structure::hits(self.as_ref())
    }

    fn hits_max(&self) -> u32 {
        Structure::hits_max(self.as_ref())
    }
}
impl CanDecay for StructureContainer {
    fn ticks_to_decay(&self) -> u32 {
        Self::ticks_to_decay(self)
    }
}
impl HasId for StructureContainer {
    fn id(&self) -> Option<JsString> {
        Some(Structure::id(self.as_ref()))
    }
}
impl HasPosition for StructureContainer {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl HasStore for StructureContainer {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
impl RoomObjectProperties for StructureContainer {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
impl StructureProperties for StructureContainer {}
