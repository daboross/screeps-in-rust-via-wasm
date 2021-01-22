use crate::{
    objects::{OwnedStructure, Owner, Room, RoomObject, RoomPosition, Store, Structure},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureLink`], which can hold energy and
    /// transfer it to other links within the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLink)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureLink;

    /// The number of ticks until the [`StructureLink`] can use
    /// [`StructureLink::transfer`] again.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLink.cooldown)
    #[wasm_bindgen(method, getter)]
    pub fn cooldown(this: &StructureLink) -> u32;

    /// The [`Store`] of the extension, which contains information about the
    /// amount of energy in it.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLink.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureLink) -> Store;

    /// Transfer energy from this [`StructureLink`] to another, losing
    /// [`LINK_LOSS_RATIO`] and incurring a cooldown of [`LINK_COOLDOWN`] per
    /// range to the target.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLink.transferEnergy)
    #[wasm_bindgen(method, js_name = transferEnergy)]
    pub fn transfer_energy(this: &StructureLink, target: &StructureLink) -> i8;
}

impl Attackable for StructureLink {
    fn hits(&self) -> u32 {
        Structure::hits(self.as_ref())
    }

    fn hits_max(&self) -> u32 {
        Structure::hits_max(self.as_ref())
    }
}
impl HasCooldown for StructureLink {
    fn cooldown(&self) -> u32 {
        Self::cooldown(self)
    }
}

impl HasId for StructureLink {
    fn id(&self) -> Option<JsString> {
        Some(Structure::id(self.as_ref()))
    }
}
impl HasPosition for StructureLink {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl HasStore for StructureLink {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
impl OwnedStructureProperties for StructureLink {
    fn my(&self) -> bool {
        OwnedStructure::my(self.as_ref())
    }

    fn owner(&self) -> Option<Owner> {
        OwnedStructure::owner(self.as_ref())
    }
}
impl RoomObjectProperties for StructureLink {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
impl StructureProperties for StructureLink {}
