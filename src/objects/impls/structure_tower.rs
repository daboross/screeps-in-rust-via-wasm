use crate::{
    objects::{OwnedStructure, Owner, Room, RoomObject, RoomPosition, Store, Structure},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureTower`], which can heal, repair, or
    /// attack anywhere in the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTower)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureTower;

    /// The [`Store`] of the tower, which contains energy which is consumed when
    /// it takes actions.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTower.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureTower) -> Store;

    /// Attack a [`Creep`], [`PowerCreep`], or [`Structure`] in the room,
    /// dealing damage depending on range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTower.attack)
    #[wasm_bindgen(method)]
    pub fn attack(this: &StructureTower, target: &RoomObject) -> i8;

    /// Heal a [`Creep`] or [`PowerCreep`] in the room, adding hit points
    /// depending on range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTower.heal)
    #[wasm_bindgen(method)]
    pub fn heal(this: &StructureTower, target: &RoomObject) -> i8;

    /// Repair a [`Structure`] in the room, adding hit points depending on
    /// range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTower.repair)
    #[wasm_bindgen(method)]
    pub fn repair(this: &StructureTower, target: &Structure) -> i8;
}

impl Attackable for StructureTower {
    fn hits(&self) -> u32 {
        Structure::hits(self.as_ref())
    }

    fn hits_max(&self) -> u32 {
        Structure::hits_max(self.as_ref())
    }
}
impl HasId for StructureTower {
    fn id(&self) -> Option<JsString> {
        Some(Structure::id(self.as_ref()))
    }
}
impl HasPosition for StructureTower {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl HasStore for StructureTower {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
impl OwnedStructureProperties for StructureTower {
    fn my(&self) -> bool {
        OwnedStructure::my(self.as_ref())
    }

    fn owner(&self) -> Option<Owner> {
        OwnedStructure::owner(self.as_ref())
    }
}
impl RoomObjectProperties for StructureTower {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
impl StructureProperties for StructureTower {}
