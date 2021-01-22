use crate::{
    objects::{Room, RoomObject, RoomPosition, Store},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`ScoreContainer`], which appears randomly
    /// around the map and contains [`ResourceType::Score`] which can be
    /// collected.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#ScoreContainer)
    ///
    /// [`ResourceType::Score`]: crate::constants::ResourceType::Score
    #[wasm_bindgen(extends = RoomObject)]
    pub type ScoreContainer;

    /// Object ID of the collector, which can be used to efficiently fetch a
    /// fresh reference to the object on subsequent ticks.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#ScoreContainer.id)
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &ScoreContainer) -> JsString;

    /// The [`Store`] of the container, which contains information about what
    /// resources it is it holding.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#ScoreContainer.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &ScoreContainer) -> Store;

    /// The number of ticks until the [`ScoreContainer`] will decay,
    /// disappearing completely.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#ScoreContainer.ticksToDecay)
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &ScoreContainer) -> u32;
}

impl CanDecay for ScoreContainer {
    fn ticks_to_decay(&self) -> u32 {
        Self::ticks_to_decay(self)
    }
}
impl HasId for ScoreContainer {
    fn id(&self) -> Option<JsString> {
        Some(Self::id(self.as_ref()))
    }
}
impl HasPosition for ScoreContainer {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl HasStore for ScoreContainer {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
impl RoomObjectProperties for ScoreContainer {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
