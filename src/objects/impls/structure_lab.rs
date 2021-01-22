use crate::{
    constants::ResourceType,
    objects::{Creep, OwnedStructure, Owner, Room, RoomObject, RoomPosition, Store, Structure},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureLab`], which can be used to create
    /// mineral compounds.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLab)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureLab;

    /// The number of ticks until the [`StructureLab`] can use
    /// [`StructureLab::run_reaction`] or [`StructureLab::unboost_creep`] again.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLab.cooldown)
    #[wasm_bindgen(method, getter)]
    pub fn cooldown(this: &StructureLab) -> u32;

    /// The [`Store`] of the lab, which can contain energy and one type of
    /// resource at a time.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLab.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureLab) -> Store;

    /// Get the type of mineral currently contained in the lab, which can only
    /// hold one type at a time
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLab.mineralType)
    #[wasm_bindgen(method, getter = mineralType)]
    pub fn mineral_type(this: &StructureLab) -> Option<ResourceType>;

    /// Boost a [`Creep`] in melee range, consuming [`LAB_BOOST_ENERGY`] energy
    /// and [`LAB_BOOST_MINERAL`] of the boost compound from the
    /// [`StructureLab::store`] per boosted body part.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLab.boostCreep)
    ///
    /// [`LAB_BOOST_ENERGY`]: crate::constants::numbers::LAB_BOOST_ENERGY
    /// [`LAB_BOOST_MINERAL`]: crate::constants::numbers::LAB_BOOST_MINERAL
    #[wasm_bindgen(method, js_name = boostCreep)]
    pub fn boost_creep(this: &StructureLab, creep: &Creep, body_part_count: Option<u32>) -> i8;

    /// Reverse a reaction, splitting the compound in this [`StructureLab`] into
    /// its components in two other labs.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLab.reverseReaction)
    #[wasm_bindgen(method, js_name = reverseReaction)]
    pub fn reverse_reaction(this: &StructureLab, lab1: &StructureLab, lab2: &StructureLab) -> i8;

    /// Run a reaction, combining components from two other [`StructureLab`]s
    /// into a new compound in this lab.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLab.runReaction)
    #[wasm_bindgen(method, js_name = runReaction)]
    pub fn run_reaction(this: &StructureLab, lab1: &StructureLab, lab2: &StructureLab) -> i8;

    /// Unboost a [`Creep`], removing all boosts from its body and dropping
    /// [`LAB_UNBOOST_MINERAL`] per body part on the ground, with a cooldown
    /// equal to the total time to produce the removed boosts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLab.unboostCreep)
    ///
    /// [`LAB_UNBOOST_ENERGY`]: crate::constants::numbers::LAB_UNBOOST_ENERGY
    #[wasm_bindgen(method, js_name = unboostCreep)]
    pub fn unboost_creep(this: &StructureLab, creep: &Creep) -> i8;
}

impl Attackable for StructureLab {
    fn hits(&self) -> u32 {
        Structure::hits(self.as_ref())
    }

    fn hits_max(&self) -> u32 {
        Structure::hits_max(self.as_ref())
    }
}
impl HasCooldown for StructureLab {
    fn cooldown(&self) -> u32 {
        Self::cooldown(self)
    }
}

impl HasId for StructureLab {
    fn id(&self) -> Option<JsString> {
        Some(Structure::id(self.as_ref()))
    }
}
impl HasPosition for StructureLab {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl HasStore for StructureLab {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
impl OwnedStructureProperties for StructureLab {
    fn my(&self) -> bool {
        OwnedStructure::my(self.as_ref())
    }

    fn owner(&self) -> Option<Owner> {
        OwnedStructure::owner(self.as_ref())
    }
}
impl RoomObjectProperties for StructureLab {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
impl StructureProperties for StructureLab {}
