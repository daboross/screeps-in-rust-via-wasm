use crate::{
    objects::{Room, RoomObject, RoomPosition, Store},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// A [`Tombstone`], which represents a dead creep and can have resources
    /// withdrawn from it.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Tombstone)
    #[wasm_bindgen(extends = RoomObject)]
    pub type Tombstone;

    /// The dead [`Creep`] or [`PowerCreep`] that this tombstone represents.
    /// Note that this object is not fully safe to use, and needs to be cast
    /// into the correct type.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Tombstone.creep)
    #[wasm_bindgen(method, getter)]
    pub fn creep(this: &Tombstone) -> RoomObject;

    /// The tick that the creep was killed.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Tombstone.deathTime)
    #[wasm_bindgen(method, getter = deathTime)]
    pub fn death_time(this: &Tombstone) -> u32;

    /// Object ID of the tombstone, which can be used to efficiently fetch a
    /// fresh reference to the object on subsequent ticks.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Tombstone.id)
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Tombstone) -> JsString;

    /// The [`Store`] of the tombstone, which contains any resources in the
    /// tombstone.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Tombstone.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &Tombstone) -> Store;

    /// The number of ticks until this tombstone disappears.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Tombstone.ticksToDecay)
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &Tombstone) -> u32;
}

impl CanDecay for Tombstone {
    fn ticks_to_decay(&self) -> u32 {
        Self::ticks_to_decay(self)
    }
}
impl HasId for Tombstone {
    fn id(&self) -> Option<JsString> {
        Some(Self::id(self))
    }
}
impl HasPosition for Tombstone {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl HasStore for Tombstone {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
impl RoomObjectProperties for Tombstone {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
