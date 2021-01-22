use crate::{
    objects::{Room, RoomObject, RoomPosition, Structure},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Parent class for all [`Structure`] objects types which are (or can be)
    /// owned by a specific player.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#OwnedStructure)
    #[wasm_bindgen(extends = RoomObject, extends = Structure)]
    pub type OwnedStructure;

    /// Whether this structure is owned by the player.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#OwnedStructure.my)
    #[wasm_bindgen(method, getter)]
    pub fn my(this: &OwnedStructure) -> bool;

    /// The [`Owner`] of this structure that contains the owner's username, or
    /// `None` if it's an ownable structure currently not under a player's
    /// control.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#OwnedStructure.owner)
    #[wasm_bindgen(method, getter)]
    pub fn owner(this: &OwnedStructure) -> Option<Owner>;
}

impl HasPosition for OwnedStructure {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl RoomObjectProperties for OwnedStructure {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}

#[wasm_bindgen]
extern "C" {
    /// Object with owner info for an owned game object.
    #[wasm_bindgen]
    pub type Owner;

    /// The name of the player that owns this structure as a [`JsString`].
    #[wasm_bindgen(method, getter)]
    pub fn username(this: &Owner) -> JsString;
}
