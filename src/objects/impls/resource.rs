use crate::{
    constants::ResourceType,
    objects::{Room, RoomObject, RoomPosition},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// A [`Resource`] is an object representing resources that have been
    /// dropped and can be picked up.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Resource)
    #[wasm_bindgen(extends = RoomObject)]
    pub type Resource;

    /// Amount of resource this contains.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Resource.amount)
    #[wasm_bindgen(method, getter)]
    pub fn amount(this: &Resource) -> u32;

    /// Object ID of the resource, which can be used to efficiently fetch a
    /// fresh reference to the object on subsequent ticks.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Resource.id)
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Resource) -> JsString;

    /// The type of resource this contains.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Resource.resourceType)
    #[wasm_bindgen(method, getter = resourceType)]
    pub fn resource_type(this: &Resource) -> ResourceType;
}

impl HasId for Resource {
    fn id(&self) -> Option<JsString> {
        Some(Self::id(self))
    }
}
impl HasPosition for Resource {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl RoomObjectProperties for Resource {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
