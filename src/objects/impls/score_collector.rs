use crate::{
    objects::{Room, RoomObject, RoomPosition, Store},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`ScoreCollector`], which can have
    /// [`ResourceType::Score`] transferred to it in order to score points on
    /// the leaderboard.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#ScoreCollector)
    ///
    /// [`ResourceType::Score`]: crate::constants::ResourceType::Score
    #[wasm_bindgen(extends = RoomObject)]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    pub type ScoreCollector;

    /// Object ID of the collector, which can be used to efficiently fetch a
    /// fresh reference to the object on subsequent ticks.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#ScoreCollector.id)
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &ScoreCollector) -> JsString;

    /// The [`Store`] of the container, which contains information about what
    /// resources it is it holding.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#ScoreCollector.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &ScoreCollector) -> Store;
}

impl HasId for ScoreCollector {
    fn id(&self) -> Option<JsString> {
        Some(Self::id(self.as_ref()))
    }
}

impl HasStore for ScoreCollector {
    fn store(&self) -> Store {
        Self::store(self)
    }
}