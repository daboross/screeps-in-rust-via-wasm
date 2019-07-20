Unreleased
==========

- `Serialize` and `Deserialize` format changed for most constants and for `LocalRoomPosition`
  (breaking)
  - constants now always serialize as integers (see
    [the module doc](https://docs.rs/screeps-game-api/0.5/screeps/constants/index.html)) (#164)
  - `LocalRoomPosition` will now serialize differently depending on whether the format is readable
    (see the [`LocalRoomPosition`
    doc](https://docs.rs/screeps-game-api/0.5/screeps/struct.LocalRoomPosition.html)) (#171)
- `constants-serde` feature removed in favor of always implementing Serialize and Dserialize for
  constants. (#164)
- Add methods for retrieving coords from `RoomPosition` (#145)
- Implement `Hash` for most constnats (#148)
- Add equality checking between strings and `LocalRoomName` (#149)
- Add `raw_memory::drop_segment` (#160)
- Add `inter_shard_memory` (#163)
- Change `MemoryReference::get` and `get_path` to return `Option<T>` (breaking) (#169)
- Add `Display` implementations for `Direction` and `LocalRoomPosition`
- Fix visibility of `LocalRoomNameParseError`, `Bodypart`, `MoveToOptions` and `PortalDestination`
  (#134)
- Fix `Creep::get_active_bodyparts` (#147)
- Fix `CostMatrix` upload method (#150)
- Fix typos in `StructureController` getters (#151)
- Fix `RoomPosition::try_from` conversion (#152)
- Fix errors on successful flag creation (#157)
- Fix `ConstructionSite::structure_type`, `Mineral::mineral_type` and `StructureLab::mineral_type`
  (#173)
- Change many `StructureController` properties to `Option`s to reflect reality (breaking) (#162)
- Migrate to Rust 2018 (#161)
- Improve code reuse and reduce total amount of code (#164, #166, #175)

Thanks to [@ASalvail], [@Dessix] and [@mettke] for contributing to this release.

0.4.0 (2019-02-15)
==================

- Allow simulation room when using local room names (#106)
- Add `Room::look_for_at_xy` (#104)
- Take `&T` in `Room::create_construction_site` and `look_for_at` rather than `T` for
  `T: HasPosition` (breaking) (#105)
- Remove unnecessary allocation from PathFinder and Room.find calls (#112)
- Add `Neg` implementation for `Direction` allowing unary minus to reverse direction (#113)
- Add `JsVec` structure for transparently wrapping typed JavaScript arrays without immediately
  unwrapping them. (#114)
- Switch to `num-derive` for deriving number->enum transitions (previously `enum-primitive`)
  (#116)
- Fix inability to decode `Structure` at all (#119)
- Remove `Sized` bound on `RoomObjectProperties` and add `SizedRoomObject` trait
- Add utility methods for turning `Structure` into `Option<&SomeTrait>` for a variety of traits -
  for instance, see `as_attackable()` (#122)
- Fix constant name `DROPPED_RESOURCES` (previously: `DROPPED_RESOUCES`) (#126)
- Remove `Attackable` implementation for `Structure` (`StructureController` and `StructurePortal`
  are _not_ attackable). Use `as_attackable()` instead. (#129)

0.3.0 (2018-11-12)
==================

- Implement Serialize and Deserialize for constants under `constants-serde` feature flag (#69)
- Fix Tombstone::get_position_at (#70)
- Remove duplicate Tombstone::id property (#70)
- Implement September 2018 update (#71)
  - Replace old terrain APIs with game::map::get_room_terrain
  - Add event API
  - Add support for named construction sites on RoomPosition
- Add support for `findRoute` and `findExit` (#76)
- Add full support for Creep::body (#78)
- Add support for `moveTo` options (#80)
- Replace all usages of u8 for in-room positions with u32 (#80)
- Remove duplicate properties on StructureSpawn (#82)
- Use u32 for everything non-negative (#83)
- Add StructureSpawn::spawning property (#86)
- Replace use-once builder with more standard options struct in `StructureSpawn::spawn` (#86)
- Add look, lookAt, lookAtArea support (#87)
- Add MemoryReference::get<T>, get_path<T> for retrieving arbitrary types from memory (#96)
- Fix unconditional error in `find_in_range`, `find_closest_by_range` bindings (#99)
- Unbreak `mem_get!` macro broken in last release (#102)
- Rename `Density` enum variants to remove `Density` prefix (#101)
- Make various API functions take `Copy` types by value rather than by reference (#101)

0.2.0 (2018-08-28)
==================

- Remove `Reservation` and `Sign` structures from prelude (breaking)
- Add `Eq`, `PartialEq` implementations to everything which has an ID, as well as `Room`
- Implement `RawMemory` `get` and `set` support
- Create CONTRIBUTING document
- Change all `TryFrom` implementations to perform type checks rather than making assumptions.
- Implement `ReferenceType` for all reference wrapping structures
- Add traits `ExpectedTypeInto` and `ExpectedTypeFrom` for converting `stdweb::Value`s to typed
  structures with or without type checks depending on `screeps-game-api`'s feature flags
  - Add feature `check-all-casts` which is off by default and when enabled adds type checks to
    `ExpectedTypeFrom` conversions
- Create enum `Density` rather than having `DENSITY_LOW`, `_MODERATE`, `_HIGH` and `_ULTRA`
  constants (breaking)
- Implement support for APIs:
  - `RawMemory.get`, `set`
  - `Game.map.findExit`
  - `Game.market.*`
  - `Game.notify`
  - `Creep.drop`, `owner.username`, `moveByPath`, `notifyWhenAttacked`, `saying`
  - `Flag.remove`, `setColor`, `setPosition`
  - `Mineral.density`, `mineralAmount`, `mineralType`, `ticksToRegeneration`
  - `Nuke.launchRoomName`, `timeToLand`
  - `Room.serializePath`, `deserializePath`, `findExitTo`, `getPositionAt`, `findPath`, `memory`,
    `findPathTo`
  - `RoomPosition.isEqualTo` accepting `x`/`y` parameters
  - `StructureController.activateSafeMode`, `unclaim`
  - `StructureKeeperLair.ticksToSpawn`
  - `StructureLab.mineralAmount`, `mineralCapacity`, `mineralType`, `boostCreep`, `runReaction`
  - `StructureLink.transferEnergy`
  - `StructureNuker.ghodium`, `ghodiumCapacity`, `launchNuke`
  - `StructureObserver.observeRoom`
  - `StructurePortal.destination`
  - `StructurePowerBank.power`
  - `StructurePowerSpawn.processPower`
  - `StructureRampart.setPublic`
  - `StructureSpawn.memory`, `recycleCreep`, `renewCreep`
  - `StructureTerminal.send`
  - `StructureTower.attack`, `heal`, `repair`
- Rename `get_object` to `get_object_erased` and add support for `get_object_typed` directly
  fetching a room object knowing its type (breaking)
- Add support for fetching and setting memory keys by path using lodash
- Rename memory `num` method to `f64` and `int` to `i32`
- Add `mem_get!` and `mem_set!` macros for compile-time memory path interpretation.
- Rename `owner` methods to `owner_name`
- Fix `Creep.signController` type signature
- Fix `Room.findClosestByRange` type signature
- Add trait `HasId` representing things which have ids (all `RoomObject`s except `Flag`s)
  - Move `id` methods from individual structures to this structure (breaking)
- Add traits `CanStoreEnergy`, `HasCooldown`, `CanDecay`, `Withdrawable`, `Attackable`
- Move `hits` and `hitsMax` bindings from `StructureProperties` trait to `Attackable` trait
  (breaking)
- Re-export `stdweb::unstable::{TryFrom, TryInto}` in `traits` module

Thanks to [@ASalvail] for planning and implementing the vast majority of this release!

0.1.1 (2018-09-04)
==================

- Add support for StructureController (thanks [@ASalvail]!)

0.1.0 (2018-08-16)
==================

- Work around Rust bug appearing in latest nightly
  (https://github.com/rust-lang/rust/issues/53140)
- Bump to 0.1.0 so we can create bugfix releases such as this one which
  do not contain breaking changes.

0.0.10 (2018-06-16)
===================

- Add ReturnCode::as_result
- Add MemoryReference::from_reference_unchecked
- Support ConstructionSite properties
- Support Room::look_for_at_area
- Make 'Structure' type an enum rather than opaque reference
- Add LocalRoomPosition::remote shorthand

0.0.9 (2018-03-20)
==================

- Add string access API to Memory

0.0.8 (2018-03-12)
==================

- Implement PathFinder interface
- Implement Serialize/Deserialize for LocalRoomPosition
- Add LocalRoomName structure
- Change time to unsigned integers
- Add StructureSpawn::is_spawning
- Add RoomPosition::local utility
- Fix HeapStatistics field names
- Fix console.error hack (see cargo-screeps changes)

0.0.7 (2018-03-06)
==================

- Hotfix for code so it compiles

0.0.6 (2018-03-06)
==================

- Add tombstone support
- Add reaction_time constant calculation method
- Support v8_getheapstatistics
- Replace some get_from_js!() calls with manual functions
- Switch from using .0 to .as_ref() to refer to inner Reference of wrapping objects
- Add support for creating construction sites and flags
- Add Source support
- Add support for spawning creeps
- Make methods of traits have default impls, instead of using macros to implement them

0.0.5 (2018-03-01)
==================

- Fix crates.io distribution to include javascript files

0.0.4 (2018-03-01)
==================

- Fix crates.io distribution to include Web.toml

0.0.3 (2018-03-01)
==================

- No notable changes

0.0.2 (2018-03-01)
==================

- No notable changes

0.0.1 (2018-02-28)
==================

- Initial release


[@ASalvail]: https://github.com/ASalvail
[@Dessix]: https://github.com/Dessix
[@mettke]: https://github.com/mettke
