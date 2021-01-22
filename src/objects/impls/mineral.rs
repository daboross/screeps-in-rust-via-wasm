use crate::{
    constants::{Density, ResourceType},
    objects::{Room, RoomObject, RoomPosition},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// A [`Mineral`], which can be harvested for resources with an extractor.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Mineral)
    #[wasm_bindgen(extends = RoomObject)]
    pub type Mineral;

    /// The density of the mineral on the next refill after it's depleted.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Mineral.density)
    #[wasm_bindgen(method, getter)]
    pub fn density(this: &Mineral) -> Density;

    /// Type of resource contained in this mineral.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Mineral.mineralType)
    #[wasm_bindgen(method, getter = mineralType)]
    pub fn mineral_type(this: &Mineral) -> ResourceType;

    /// Object ID of the mineral, which can be used to efficiently fetch a fresh
    /// reference to the object on subsequent ticks.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Mineral.id)
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Mineral) -> JsString;

    /// The number of ticks until this mineral regenerates from depletion.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Mineral.ticksToRegeneration)
    #[wasm_bindgen(method, getter = ticksToRegeneration)]
    pub fn ticks_to_regeneration(this: &Mineral) -> u32;
}

impl HasId for Mineral {
    fn id(&self) -> Option<JsString> {
        Some(Self::id(self))
    }
}
impl HasPosition for Mineral {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl RoomObjectProperties for Mineral {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
