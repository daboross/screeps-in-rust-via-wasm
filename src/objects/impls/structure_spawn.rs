use crate::{
    objects::{Creep, OwnedStructure, Owner, Room, RoomObject, RoomPosition, Store, Structure},
    prelude::*,
};
use js_sys::{Array, JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureSpawn`], which creates your creeps.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureSpawn;

    /// A shortcut to `Memory.spawns[spawn.name]`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.memory)
    #[wasm_bindgen(method, getter)]
    pub fn memory(this: &StructureSpawn) -> JsValue;

    /// Sets a new value to `Memory.spawns[spawn.name]`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.memory)
    #[wasm_bindgen(method, setter)]
    pub fn set_memory(this: &StructureSpawn, val: &JsValue);

    /// The spawn's name as an owned reference to a [`JsString`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.name)
    #[wasm_bindgen(method, getter)]
    pub fn name(this: &StructureSpawn) -> JsString;

    /// Information about the spawning creep, if one is currently being spawned.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.spawning)
    #[wasm_bindgen(method, getter)]
    pub fn spawning(this: &StructureSpawn) -> Option<Spawning>;

    /// The [`Store`] of the spawn, which contains information about what
    /// resources it is it holding.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureSpawn) -> Store;

    /// Create a new creep with the specified body part [`Array`], name
    /// [`JsString`], and optional spawning options. Note that successfully
    /// spawning will store data in `Memory.creeps[creep_name]` _regardless
    /// of whether any memory data was passed in the options object_ and enable
    /// the default serialization behavior of the `Memory` object, which may
    /// hamper attempts to directly use `RawMemory`. todo, add note+docs
    /// about how to replace Memory and/or delete RawMemory._parsed
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.spawnCreep)
    #[wasm_bindgen(method, js_name = spawnCreep)]
    pub fn spawn_creep(
        this: &StructureSpawn,
        body: &Array,
        name: &JsString,
        options: Option<Object>,
    ) -> i8;

    /// Kill a [`Creep`] in melee range, returning 100% of its TTL-adjusted
    /// resources (5x more than if the creep is killed another way). Can be used
    /// while spawning.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.recycleCreep)
    #[wasm_bindgen(method, js_name = recycleCreep)]
    pub fn recycle_creep(this: &StructureSpawn, creep: &Creep) -> i8;

    /// Renew a [`Creep`] in melee range, removing all boosts adding to its TTL.
    /// Cannot be used while spawning.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.renewCreep)
    #[wasm_bindgen(method, js_name = renewCreep)]
    pub fn renew_creep(this: &StructureSpawn, creep: &Creep) -> i8;
}

impl Attackable for StructureSpawn {
    fn hits(&self) -> u32 {
        Structure::hits(self.as_ref())
    }

    fn hits_max(&self) -> u32 {
        Structure::hits_max(self.as_ref())
    }
}
impl HasId for StructureSpawn {
    fn id(&self) -> Option<JsString> {
        Some(Structure::id(self.as_ref()))
    }
}
impl HasPosition for StructureSpawn {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl HasStore for StructureSpawn {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
impl OwnedStructureProperties for StructureSpawn {
    fn my(&self) -> bool {
        OwnedStructure::my(self.as_ref())
    }

    fn owner(&self) -> Option<Owner> {
        OwnedStructure::owner(self.as_ref())
    }
}
impl RoomObjectProperties for StructureSpawn {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
impl StructureProperties for StructureSpawn {}

#[wasm_bindgen]
extern "C" {
    /// Object with info on what a [`StructureSpawn`] is currently spawning.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn-Spawning)
    #[wasm_bindgen(js_namespace = StructureSpawn)]
    pub type Spawning;

    /// Allowed directions for the creep to exit the spawn; can be changed with
    /// [`Spawning::set_directions`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.Spawning.directions)
    #[wasm_bindgen(method, getter)]
    pub fn directions(this: &Spawning) -> Array;

    /// The name of the spawning creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.Spawning.name)
    #[wasm_bindgen(method, getter)]
    pub fn name(this: &Spawning) -> JsString;

    /// Total time needed to spawn this creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.Spawning.needTime)
    #[wasm_bindgen(method, getter = needTime)]
    pub fn need_time(this: &Spawning) -> u32;

    /// Total time remaining to spawn this creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.Spawning.remainingTime)
    #[wasm_bindgen(method, getter = remainingTime)]
    pub fn remaining_time(this: &Spawning) -> u32;

    /// Get a reference to the spawn.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.Spawning.spawn)
    #[wasm_bindgen(method, getter)]
    pub fn spawn(this: &Spawning) -> StructureSpawn;

    /// Cancel spawning this creep, without refunding any energy.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.Spawning.cancel)
    #[wasm_bindgen(method)]
    pub fn cancel(this: &Spawning) -> i8;

    /// Change allowed directions for the creep to leave the spawn once it's
    /// ready.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.Spawning.setDirections)
    #[wasm_bindgen(method, js_name = setDirections)]
    pub fn set_directions(this: &Spawning) -> i8;
}
