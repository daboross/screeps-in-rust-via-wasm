use crate::{
    constants::ResourceType,
    objects::{OwnedStructure, Owner, Room, RoomObject, RoomPosition, Store, Structure},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureFactory`], which can compress and
    /// decompress resources and produce commodities for sale.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureFactory)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureFactory;

    /// Ticks until [`StructureFactory::produce`] can be used again.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureFactory.cooldown)
    ///
    /// [`StructureFactory::produce`]: crate::objects::StructureFactory::produce
    #[wasm_bindgen(method, getter)]
    pub fn cooldown(this: &StructureFactory) -> u32;

    /// The level of the factory, which cannot be changed once set by a power
    /// creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureFactory.level)
    #[wasm_bindgen(method, getter)]
    pub fn level(this: &StructureFactory) -> u8;

    /// The [`Store`] of the factory, which contains information about what
    /// resources it is it holding.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureFactory.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureFactory) -> Store;

    /// Produce a commodity in the factory.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureFactory.produce)
    #[wasm_bindgen(method)]
    pub fn produce(this: &StructureFactory, ty: ResourceType) -> i8;
}

impl Attackable for StructureFactory {
    fn hits(&self) -> u32 {
        Structure::hits(self.as_ref())
    }

    fn hits_max(&self) -> u32 {
        Structure::hits_max(self.as_ref())
    }
}
impl HasCooldown for StructureFactory {
    fn cooldown(&self) -> u32 {
        Self::cooldown(self)
    }
}
impl HasId for StructureFactory {
    fn id(&self) -> Option<JsString> {
        Some(Structure::id(self.as_ref()))
    }
}
impl HasPosition for StructureFactory {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl HasStore for StructureFactory {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
impl OwnedStructureProperties for StructureFactory {
    fn my(&self) -> bool {
        OwnedStructure::my(self.as_ref())
    }

    fn owner(&self) -> Option<Owner> {
        OwnedStructure::owner(self.as_ref())
    }
}
impl RoomObjectProperties for StructureFactory {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
impl StructureProperties for StructureFactory {}
