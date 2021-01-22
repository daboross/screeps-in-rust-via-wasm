use crate::{
    objects::{Room, RoomObject, RoomPosition, Store, Structure},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// A [`Ruin`], which represents a destroyed structure and can have
    /// resources withdrawn from it.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Ruin)
    #[wasm_bindgen(extends = RoomObject)]
    pub type Ruin;

    /// The tick that the structure was destroyed
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Ruin.destroyTime)
    #[wasm_bindgen(method, getter = destroyTime)]
    pub fn destroy_time(this: &Ruin) -> u32;

    /// Object ID of the ruin, which can be used to efficiently fetch a fresh
    /// reference to the object on subsequent ticks.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Ruin.id)
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Ruin) -> JsString;

    /// The [`Store`] of the ruin, which contains any resources in the ruin.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Ruin.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &Ruin) -> Store;

    /// The destroyed [`Structure`] that this ruin represents. Note that this
    /// object is not fully safe to use as a [`Structure`], missing critical
    /// properties such as position; it's only safe to access basic information
    /// about the structure on this object, like the structure type, owner name,
    /// and id.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Ruin.structure)
    #[wasm_bindgen(method, getter)]
    pub fn structure(this: &Ruin) -> Structure;

    /// The number of ticks until this ruin disappears.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Ruin.ticksToDecay)
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &Ruin) -> u32;
}

impl CanDecay for Ruin {
    fn ticks_to_decay(&self) -> u32 {
        Self::ticks_to_decay(self)
    }
}
impl HasId for Ruin {
    fn id(&self) -> Option<JsString> {
        Some(Self::id(self))
    }
}
impl HasPosition for Ruin {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl HasStore for Ruin {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
impl RoomObjectProperties for Ruin {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
