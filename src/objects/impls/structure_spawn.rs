use stdweb::Reference;

use crate::{
    constants::{Direction, Part, ReturnCode},
    memory::MemoryReference,
    objects::{Creep, HasEnergyForSpawn, SizedRoomObject, Spawning, StructureSpawn},
    traits::TryInto,
};

simple_accessors! {
    impl StructureSpawn {
        pub fn name() -> String = name;
        pub fn spawning() -> Option<Spawning> = spawning;
    }
}

impl StructureSpawn {
    pub fn memory(&self) -> MemoryReference {
        js_unwrap!(@{self.as_ref()}.memory)
    }

    pub fn spawn_creep(&self, body: &[Part], name: &str) -> ReturnCode {
        let ints = body.iter().map(|p| *p as u32).collect::<Vec<u32>>();
        (js! {
            var body = (@{ints}).map(__part_num_to_str);

            return @{self.as_ref()}.spawnCreep(body, @{name});
        })
        .try_into()
        .expect("expected StructureSpawn::spawnCreep to return an integer return code")
    }

    pub fn spawn_creep_with_options(
        &self,
        body: &[Part],
        name: &str,
        opts: &SpawnOptions,
    ) -> ReturnCode {
        let body_ints = body.iter().map(|p| *p as u32).collect::<Vec<u32>>();

        let js_opts = js!(return {dryRun: @{opts.dry_run}};);

        if let Some(ref mem) = opts.memory {
            js! { @(no_return)
                @{&js_opts}.memory = @{mem.as_ref()};
            }
        }
        if !opts.energy_structures.is_empty() {
            js! { @(no_return)
                @{&js_opts}.energyStructures = @{&opts.energy_structures};
            }
        }
        if !opts.directions.is_empty() {
            js! { @(no_return)
                @{&js_opts}.directions = @{&opts.directions};
            }
        }
        (js! {
            var body = (@{body_ints}).map(__part_num_to_str);

            return @{self.as_ref()}.spawnCreep(body, @{name}, @{js_opts});
        })
        .try_into()
        .expect("expected StructureSpawn::spawnCreep to return an integer return code")
    }
///
/// Examples
///
/// Give a new creep a `WORKER` boolean memory
///
/// ```no_run
/// let mem = MemoryReference::new();
/// mem.set("WORKER", true);
/// spawn.spawn_creep_with_options(&body, &name, &SpawnOptions::new().memory(mem));
/// ```
///
    
    // TODO: support actually using Spawning properties.
    pub fn is_spawning(&self) -> bool {
        js_unwrap!(Boolean(@{self.as_ref()}.spawning))
    }

    pub fn recycle_creep(&self, target: &Creep) -> ReturnCode {
        js_unwrap! {@{self.as_ref()}.recycleCreep(@{target.as_ref()})}
    }

    pub fn renew_creep(&self, target: &Creep) -> ReturnCode {
        js_unwrap! {@{self.as_ref()}.renewCreep(@{target.as_ref()})}
    }
}

#[derive(Default)]
pub struct SpawnOptions {
    memory: Option<MemoryReference>,
    energy_structures: Vec<Reference>,
    dry_run: bool,
    directions: Vec<u32>,
}

impl SpawnOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn memory<T: Into<Option<MemoryReference>>>(mut self, mem: T) -> Self {
        self.memory = mem.into();
        self
    }

    /// This is most useful with the `.as_structure()` method on structures.
    pub fn energy_structures<T>(mut self, structures: T) -> Self
    where
        T: IntoIterator,
        <T as IntoIterator>::Item: HasEnergyForSpawn + SizedRoomObject,
    {
        self.energy_structures = structures.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    pub fn directions(mut self, directions: &[Direction]) -> Self {
        self.directions = directions.iter().map(|d| *d as u32).collect();
        self
    }
}

simple_accessors! {
    impl Spawning {
        pub fn directions() -> Vec<Direction> = directions;
        pub fn name() -> String = name;
        pub fn need_time() -> u32 = needTime;
        pub fn remaining_time() -> u32 = remainingTime;
        pub fn spawn() -> StructureSpawn = spawn;
    }
}

impl Spawning {
    pub fn cancel(&self) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.cancel())
    }

    pub fn set_directions(&self, directions: &[Direction]) -> ReturnCode {
        let int_dirs: Vec<u32> = directions.iter().map(|d| *d as u32).collect();
        js_unwrap!(@{self.as_ref()}.setDirections(@{int_dirs}))
    }
}
