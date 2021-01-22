use crate::{
    constants::StructureType,
    objects::{Owner, Room, RoomObject, RoomPosition},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// A [`ConstructionSite`] which is an object representing a structure under
    /// construction.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#ConstructionSite)
    #[wasm_bindgen(extends = RoomObject)]
    pub type ConstructionSite;

    /// The Object ID of the [`ConstructionSite`], or `None` if it was created
    /// this tick.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#ConstructionSite.id)
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &ConstructionSite) -> Option<JsString>;

    /// Whether you own the [`ConstructionSite`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#ConstructionSite.my)
    #[wasm_bindgen(method, getter)]
    pub fn my(this: &ConstructionSite) -> bool;

    /// The [`Owner`] of this construction site, which contains the owner's
    /// username.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#ConstructionSite.owner)
    #[wasm_bindgen(method, getter)]
    pub fn owner(this: &ConstructionSite) -> Owner;

    /// The current progress toward completion of the structure being built.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#ConstructionSite.progress)
    #[wasm_bindgen(method, getter)]
    pub fn progress(this: &ConstructionSite) -> u32;

    /// The total progess toward constuction progress needed for the structure
    /// to be completed.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#ConstructionSite.progressTotal)
    #[wasm_bindgen(method, getter = progressTotal)]
    pub fn progress_total(this: &ConstructionSite) -> u32;

    /// The type of structure being constructed.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Structure.structureType)
    #[wasm_bindgen(method, getter = structureType)]
    pub fn structure_type(this: &ConstructionSite) -> StructureType;

    /// Remove the [`ConstructionSite`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#ConstructionSite.remove)
    #[wasm_bindgen(method)]
    pub fn remove(this: &ConstructionSite) -> i8;
}

impl HasId for ConstructionSite {
    fn id(&self) -> Option<JsString> {
        Self::id(self)
    }
}
impl HasPosition for ConstructionSite {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl RoomObjectProperties for ConstructionSite {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
