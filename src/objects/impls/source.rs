use crate::{
    objects::{Room, RoomObject, RoomPosition},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// A [`Source`], which can be harvested for energy.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Source)
    #[wasm_bindgen(extends = RoomObject)]
    pub type Source;

    /// Amount of energy available to be harvested from the source.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Source.energy)
    #[wasm_bindgen(method, getter)]
    pub fn energy(this: &Source) -> u32;

    /// Amount of energy this source will regenerate to after
    /// [`Source::ticks_to_regeneration`] reaches 0.
    ///
    /// Value depends on the type of room the source is in:
    ///
    /// - Owned and reserved rooms: [`SOURCE_ENERGY_CAPACITY`]
    /// - Neutral rooms: [`SOURCE_ENERGY_NEUTRAL_CAPACITY`]
    /// - Source Keeper rooms: [`SOURCE_ENERGY_KEEPER_CAPACITY`]
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Source.energy)
    ///
    /// [`SOURCE_ENERGY_CAPACITY`]: crate::constants::SOURCE_ENERGY_CAPACITY
    /// [`SOURCE_ENERGY_NEUTRAL_CAPACITY`]:
    /// crate::constants::SOURCE_ENERGY_NEUTRAL_CAPACITY
    /// [`SOURCE_ENERGY_KEEPER_CAPACITY`]:
    /// crate::constants::SOURCE_ENERGY_KEEPER_CAPACITY
    #[wasm_bindgen(method, getter = energyCapacity)]
    pub fn energy_capacity(this: &Source) -> u32;

    /// Object ID of the source, which can be used to efficiently fetch a fresh
    /// reference to the object on subsequent ticks.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Source.id)
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Source) -> JsString;

    /// The number of ticks until this source regenerates to its
    /// [`Source::energy_capacity`], or 0 if the timer has not started yet.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Source.ticksToRegeneration)
    #[wasm_bindgen(method, getter = ticksToRegeneration)]
    pub fn ticks_to_regeneration(this: &Source) -> u32;
}

impl HasId for Source {
    fn id(&self) -> Option<JsString> {
        Some(Self::id(self))
    }
}
impl HasPosition for Source {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl RoomObjectProperties for Source {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
