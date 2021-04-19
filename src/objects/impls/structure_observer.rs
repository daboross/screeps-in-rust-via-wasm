use crate::{
    constants::ReturnCode,
    objects::{OwnedStructure, RoomObject, Structure},
};
use js_sys::{JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureObserver`], which can grant vision
    /// to remote rooms.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureObserver)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureObserver;

    /// Set the [`StructureObserver`] to provide vision of a target room next
    /// tick.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureObserver.observeRoom)
    #[wasm_bindgen(method, js_name = observeRoom)]
    pub fn observe_room(this: &StructureObserver, target: &JsString) -> ReturnCode;
}