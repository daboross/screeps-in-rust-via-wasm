use crate::{
    objects::{OwnedStructure, Owner, Room, RoomObject, RoomPosition, Structure},
    prelude::*,
};
use js_sys::{Array, JsString};
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
    pub fn observe_room(this: &StructureObserver, target: &JsString) -> i8;
}

impl Attackable for StructureObserver {
    fn hits(&self) -> u32 {
        Structure::hits(self.as_ref())
    }

    fn hits_max(&self) -> u32 {
        Structure::hits_max(self.as_ref())
    }
}
impl HasId for StructureObserver {
    fn id(&self) -> Option<JsString> {
        Some(Structure::id(self.as_ref()))
    }
}

impl HasPosition for StructureObserver {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl OwnedStructureProperties for StructureObserver {
    fn my(&self) -> bool {
        OwnedStructure::my(self.as_ref())
    }

    fn owner(&self) -> Option<Owner> {
        OwnedStructure::owner(self.as_ref())
    }
}
impl RoomObjectProperties for StructureObserver {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
impl StructureProperties for StructureObserver {}
