use crate::{
    objects::{OwnedStructure, Owner, Room, RoomObject, RoomPosition, Structure},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureExtractor`], which can be placed on
    /// top of a [`Mineral`] to extract resources.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureExtractor)
    ///
    /// [`Mineral`]: crate::objects::Mineral
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureExtractor;

    /// Ticks until this extractor can be used to [`Creep::harvest`] its
    /// [`Mineral`] after a previous harvest.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureExtractor.cooldown)
    ///
    /// [`Creep::harvest`]: crate::objects::Creep::harvest
    /// [`Mineral`]: crate::objects::Mineral
    #[wasm_bindgen(method, getter)]
    pub fn cooldown(this: &StructureExtractor) -> u32;
}

impl Attackable for StructureExtractor {
    fn hits(&self) -> u32 {
        Structure::hits(self.as_ref())
    }

    fn hits_max(&self) -> u32 {
        Structure::hits_max(self.as_ref())
    }
}
impl HasCooldown for StructureExtractor {
    fn cooldown(&self) -> u32 {
        Self::cooldown(self)
    }
}
impl HasId for StructureExtractor {
    fn id(&self) -> Option<JsString> {
        Some(Structure::id(self.as_ref()))
    }
}
impl HasPosition for StructureExtractor {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl OwnedStructureProperties for StructureExtractor {
    fn my(&self) -> bool {
        OwnedStructure::my(self.as_ref())
    }

    fn owner(&self) -> Option<Owner> {
        OwnedStructure::owner(self.as_ref())
    }
}
impl RoomObjectProperties for StructureExtractor {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
impl StructureProperties for StructureExtractor {}
