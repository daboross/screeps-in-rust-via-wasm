use crate::{
    objects::{OwnedStructure, Owner, Room, RoomObject, RoomPosition, Store, Structure},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructurePowerSpawn`], which can process
    /// power to contribute to your GPL as well as renewing power creeps.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePowerSpawn)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructurePowerSpawn;

    /// The [`Store`] of the power spawn, which can contain power and energy.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePowerSpawn.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructurePowerSpawn) -> Store;

    /// Process power, consuming 1 power and [`POWER_SPAWN_ENERGY_RATIO`] energy
    /// and increasing your GPL by one point.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePowerSpawn.processPower)
    ///
    /// [`POWER_SPAWN_ENERGY_RATIO`]:
    /// crate::constants::numbers::POWER_SPAWN_ENERGY_RATIO
    #[wasm_bindgen(method)]
    pub fn process_power(this: &StructurePowerSpawn) -> i8;
}

impl Attackable for StructurePowerSpawn {
    fn hits(&self) -> u32 {
        Structure::hits(self.as_ref())
    }

    fn hits_max(&self) -> u32 {
        Structure::hits_max(self.as_ref())
    }
}
impl HasId for StructurePowerSpawn {
    fn id(&self) -> Option<JsString> {
        Some(Structure::id(self.as_ref()))
    }
}
impl HasPosition for StructurePowerSpawn {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl HasStore for StructurePowerSpawn {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
impl OwnedStructureProperties for StructurePowerSpawn {
    fn my(&self) -> bool {
        OwnedStructure::my(self.as_ref())
    }

    fn owner(&self) -> Option<Owner> {
        OwnedStructure::owner(self.as_ref())
    }
}
impl RoomObjectProperties for StructurePowerSpawn {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
impl StructureProperties for StructurePowerSpawn {}
