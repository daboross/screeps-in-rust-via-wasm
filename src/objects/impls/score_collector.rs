use crate::{
    objects::{Room, RoomObject, RoomPosition, Store},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`ScoreCollector`], which appears randomly
    /// around the map and contains [`ResourceType::Score`] which can be
    /// collected.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#ScoreCollector)
    ///
    /// [`ResourceType::Score`]: crate::constants::ResourceType::Score
    #[wasm_bindgen(extends = RoomObject)]
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
impl HasPosition for ScoreCollector {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl HasStore for ScoreCollector {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
impl RoomObjectProperties for ScoreCollector {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
