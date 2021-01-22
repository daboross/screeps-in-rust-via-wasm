use crate::{
    objects::{OwnedStructure, Owner, Room, RoomObject, RoomPosition, Store, Structure},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureNuker`], which consumes energy and
    /// ghodium to fire [`Nuke`]s.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureNuker)
    ///
    /// [`Nuke`]: crate::objects::Nuke
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureNuker;

    /// The number of ticks until the [`StructureNuker`] can use
    /// [`StructureNuker::launch_nuke`] again.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureNuker.cooldown)
    #[wasm_bindgen(method, getter)]
    pub fn cooldown(this: &StructureNuker) -> u32;

    /// The [`Store`] of the nuker, which can have energy and ghodium
    /// transferred in (but not withdrawn).
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureNuker.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureNuker) -> Store;

    /// Transfer energy from this [`StructureNuker`] to another, losing
    /// [`LINK_LOSS_RATIO`] and incurring a cooldown of [`LINK_COOLDOWN`] per
    /// range to the target.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureNuker.launchNuke)
    #[wasm_bindgen(method, js_name = launchNuke)]
    pub fn launch_nuke(this: &StructureNuker, target: &RoomPosition) -> i8;
}

impl Attackable for StructureNuker {
    fn hits(&self) -> u32 {
        Structure::hits(self.as_ref())
    }

    fn hits_max(&self) -> u32 {
        Structure::hits_max(self.as_ref())
    }
}
impl HasCooldown for StructureNuker {
    fn cooldown(&self) -> u32 {
        Self::cooldown(self)
    }
}
impl HasId for StructureNuker {
    fn id(&self) -> Option<JsString> {
        Some(Structure::id(self.as_ref()))
    }
}
impl HasPosition for StructureNuker {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl HasStore for StructureNuker {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
impl OwnedStructureProperties for StructureNuker {
    fn my(&self) -> bool {
        OwnedStructure::my(self.as_ref())
    }

    fn owner(&self) -> Option<Owner> {
        OwnedStructure::owner(self.as_ref())
    }
}
impl RoomObjectProperties for StructureNuker {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
impl StructureProperties for StructureNuker {}
