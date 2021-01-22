//! Screeps object wrappers.
//!
//! # Unsafe traits
//!
//! This module contains a number of unsafe traits. Each is unsafe purely to
//! prevent accidental implementations on things which don't uphold the trait
//! contracts that have not been put into code. There is no unsafe code
//! in this crate which relies on these traits being implemented correctly,
//! only code which will panic if they are not.
//!
//! Even though this crate does not contain any, other crate unsafe could
//! rely on these contracts being upheld as long as JavaScript code does not
//! do anything mischievous, like removing properties from objects or sticking
//! unexpected things into dictionaries which we trust.

// use stdweb::{Reference, ReferenceType, Value};
// use stdweb_derive::ReferenceType;

// use crate::{
//     constants::{ResourceType, ReturnCode, StructureType},
//     local::{ObjectId, Position, RawObjectId},
//     traits::{IntoExpectedType, TryFrom, TryInto},
//     ConversionError,
// };

// mod creep_shared;
mod impls;
//mod structure;

pub use self::impls::{
    ConstructionSite, CostMatrix, Creep, Deposit, Flag, Mineral, Nuke, OwnedStructure, Owner,
    PowerCreep, Reservation, Resource, Room, RoomObject, RoomPosition, RoomTerrain, Ruin, Sign,
    Source, Spawning, Store, Structure, StructureContainer, StructureController,
    StructureExtension, StructureExtractor, StructureFactory, StructureInvaderCore,
    StructureKeeperLair, StructureLab, StructureLink, StructureNuker, StructureObserver,
    StructurePortal, StructurePowerBank, StructurePowerSpawn, StructureRampart, StructureRoad,
    StructureSpawn, StructureStorage, StructureTower, StructureWall, Tombstone,
};

#[cfg(not(feature = "disable-terminal"))]
pub use self::impls::StructureTerminal;

#[cfg(feature = "enable-score")]
pub use self::impls::{ScoreCollector, ScoreContainer};

// pub use self::{
//     creep_shared::{MoveToOptions, SharedCreepProperties},
//     impls::{
//         AttackEvent, AttackType, Bodypart, BuildEvent, CircleStyle, Effect,
// Event, EventType,         ExitEvent, FindOptions, FontStyle, HarvestEvent,
// HealEvent, HealType, LineDrawStyle,         LineStyle, LookResult,
// ObjectDestroyedEvent, Path, PolyStyle, PortalDestination,
//         PositionedLookResult, RectStyle, RepairEvent, Reservation,
// ReserveControllerEvent,         RoomVisual, Sign, SpawnOptions, Step,
// TextAlign, TextStyle, UpgradeControllerEvent, Visual,     },
//     structure::Structure,
// };

// reference_wrappers! {
//     #[reference(instance_of = "ConstructionSite")]
//     pub struct ConstructionSite(...);
//     #[reference(instance_of = "Creep")]
//     pub struct Creep(...);
//     #[reference(instance_of = "Deposit")]
//     pub struct Deposit(...);
//     #[reference(instance_of = "Flag")]
//     pub struct Flag(...);
//     #[reference(instance_of = "Mineral")]
//     pub struct Mineral(...);
//     #[reference(instance_of = "Nuke")]
//     pub struct Nuke(...);
//     #[reference(instance_of = "OwnedStructure")]
//     pub struct OwnedStructure(...);
//     #[reference(instance_of = "Resource")]
//     pub struct Resource(...);
//     #[reference(instance_of = "Room")]
//     pub struct Room(...);
//     #[reference(instance_of = "RoomObject")]
//     pub struct RoomObject(...);
//     #[reference(instance_of = "Room.Terrain")]
//     pub struct RoomTerrain(...);
//     #[reference(instance_of = "Ruin")]
//     pub struct Ruin(...);
//     #[reference(instance_of = "Source")]
//     pub struct Source(...);
//     #[reference(instance_of = "StructureContainer")]
//     pub struct StructureContainer(...);
//     #[reference(instance_of = "StructureController")]
//     pub struct StructureController(...);
//     #[reference(instance_of = "StructureExtension")]
//     pub struct StructureExtension(...);
//     #[reference(instance_of = "StructureExtractor")]
//     pub struct StructureExtractor(...);
//     #[reference(instance_of = "StructureFactory")]
//     pub struct StructureFactory(...);
//     #[reference(instance_of = "StructureInvaderCore")]
//     pub struct StructureInvaderCore(...);
//     #[reference(instance_of = "StructureKeeperLair")]
//     pub struct StructureKeeperLair(...);
//     #[reference(instance_of = "StructureLab")]
//     pub struct StructureLab(...);
//     #[reference(instance_of = "StructureLink")]
//     pub struct StructureLink(...);
//     #[reference(instance_of = "StructureNuker")]
//     pub struct StructureNuker(...);
//     #[reference(instance_of = "StructureObserver")]
//     pub struct StructureObserver(...);
//     #[reference(instance_of = "StructurePowerBank")]
//     pub struct StructurePowerBank(...);
//     #[reference(instance_of = "StructurePowerSpawn")]
//     pub struct StructurePowerSpawn(...);
//     #[reference(instance_of = "StructurePortal")]
//     pub struct StructurePortal(...);
//     #[reference(instance_of = "StructureRampart")]
//     pub struct StructureRampart(...);
//     #[reference(instance_of = "StructureRoad")]
//     pub struct StructureRoad(...);
//     #[reference(instance_of = "StructureSpawn")]
//     pub struct StructureSpawn(...);
//     #[reference(instance_of = "StructureSpawn.Spawning")]
//     pub struct Spawning(...);
//     #[reference(instance_of = "StructureStorage")]
//     pub struct StructureStorage(...);
//     #[reference(instance_of = "StructureTerminal")]
//     pub struct StructureTerminal(...);
//     #[reference(instance_of = "StructureTower")]
//     pub struct StructureTower(...);
//     #[reference(instance_of = "StructureWall")]
//     pub struct StructureWall(...);
//     // this is implemented later
//     // #[reference(instance_of = "Structure")]
//     // pub struct Structure,
//     #[reference(instance_of = "Tombstone")]
//     pub struct Tombstone(...);
//     #[reference(instance_of = "PowerCreep")]
//     pub struct PowerCreep(...);
//     // representation returned by game::power_creeps::*, which may be alive
// on the current shard or not     #[reference(instance_of =
// "AccountPowerCreep")]     pub struct AccountPowerCreep(...);
// }

// /// Trait for things which have positions in the Screeps world.
// ///
// /// This can be freely implemented for anything with a way to get a position.
// pub trait HasPosition {
//     fn pos(&self) -> Position;
// }

// impl HasPosition for Position {
//     fn pos(&self) -> Position {
//         self.clone()
//     }
// }

// /// All `RoomObject`s have positions.
// impl<T> HasPosition for T
// where
//     T: RoomObjectProperties,
// {
//     fn pos(&self) -> Position {
//         Position::from_packed(js_unwrap!(@{self.as_ref()}.pos.__packedPos))
//     }
// }

// /// Trait covering all objects with an id.
// pub unsafe trait HasId: RoomObjectProperties {
//     /// Retrieves this object's id as an untyped, packed value.
//     ///
//     /// This has no major differences from [`HasId::id`] except for the
// return     /// value not being typed by the kind of thing it points to. As
// the type of     /// an `ObjectId` can be freely changed, that isn't a big
// deal.     fn untyped_id(&self) -> RawObjectId {
//         RawObjectId::from_packed_js_val(js_unwrap!(object_id_to_packed(@
// {self.as_ref()}.id)))             .expect("expected HasId type's JavaScript
// id to be a 12-byte number encoded in hex")     }

//     /// Retrieves this object's id as a typed, packed value.
//     ///
//     /// This can be helpful for use with [`game::get_object_typed`][1], as it
//     /// will force rust to infer the proper return type.
//     ///
//     /// If an ID without these protections is needed, use
// [`HasId::untyped_id`],     /// or `RawObjectId::from(x.id())`.
//     ///
//     /// Note that the ID returned is also stored as a packed, 12-byte value
// on     /// the stack, so it's fairly efficient to move and copy around.
//     ///
//     /// [1]: crate::game::get_object_typed
//     fn id(&self) -> ObjectId<Self>
//     where
//         Self: Sized,
//     {
//         self.untyped_id().into()
//     }
// }

// impl_has_id! {
//     ConstructionSite,
//     Creep,
//     Deposit,
//     Mineral,
//     Nuke,
//     Resource,
//     Ruin,
//     Source,
//     OwnedStructure,
//     Structure,
//     StructureContainer,
//     StructureController,
//     StructureExtension,
//     StructureExtractor,
//     StructureFactory,
//     StructureInvaderCore,
//     StructureKeeperLair,
//     StructureLab,
//     StructureLink,
//     StructureNuker,
//     StructureObserver,
//     StructurePowerBank,
//     StructurePowerSpawn,
//     StructurePortal,
//     StructureRampart,
//     StructureRoad,
//     StructureSpawn,
//     StructureStorage,
//     StructureTerminal,
//     StructureTower,
//     StructureWall,
//     Tombstone,
//     PowerCreep,
// }

// /// Trait for all wrappers over Screeps JavaScript objects extending
// /// the `RoomObject` class.
// ///
// /// # Contracts
// ///
// /// The reference returned by `AsRef<Reference>::as_ref` must reference a
// /// JavaScript object extending the `RoomObject` class.
// pub unsafe trait RoomObjectProperties: AsRef<Reference> + HasPosition {
//     /// The room that the object is in, or `None` if an object is a flag or a
//     /// construction site and is placed in a room that is not visible to you.
//     fn room(&self) -> Option<Room> {
//         js_unwrap_ref!(@{self.as_ref()}.room)
//     }

//     fn effects(&self) -> Vec<Effect> {
//         js_unwrap!(@{self.as_ref()}.effects || [])
//     }
// }

// /// Trait representing things that are both `RoomObjectProperties` and
// `Sized`. ///
// /// These bounds would be on `RoomObjectProperties`, but for the fact that
// they /// then require all `T: RoomObjectProperties` to be `T: Sized`, and
// thus /// disallow creating trait objects like `&dyn RoomObjectProperties` (or
// more /// usefully, `&dyn Attackable` or `&dyn HasStore`)
// ///
// /// This trait is automatically implemented for all structures implementing
// the /// traits it requires, and everything implement `RoomObjectProperties`
// and /// being `Sized` should also implement this.
// pub trait SizedRoomObject:
//     Into<Reference>
//     + ReferenceType
//     + TryFrom<Value, Error = ConversionError>
//     + TryFrom<Reference, Error = ConversionError>
// {
// }

// impl<T> SizedRoomObject for T where
//     T: RoomObjectProperties
//         + Into<Reference>
//         + ReferenceType
//         + TryFrom<Value, Error = ConversionError>
//         + TryFrom<Reference, Error = ConversionError>
// {
// }

// /// Trait for all wrappers over Screeps JavaScript objects extending
// /// the `Structure` class.
// ///
// /// # Contracts
// ///
// /// The reference returned by `AsRef<Reference>::as_ref` must reference a
// /// JavaScript object extending the `Structure` class.
// pub unsafe trait StructureProperties: RoomObjectProperties + HasId {
//     fn structure_type(&self) -> StructureType {
//         js_unwrap!(__structure_type_str_to_num(@{self.as_ref()}.
// structureType))     }
//     fn destroy(&self) -> ReturnCode {
//         js_unwrap!(@{self.as_ref()}.destroy())
//     }
//     fn is_active(&self) -> bool {
//         js_unwrap!(@{self.as_ref()}.isActive())
//     }
//     /// Usable on either owned structures or neutral structures in owned
// rooms,     /// returns `ReturnCode::NotOwner` otherwise.
//     fn notify_when_attacked(&self, notify_when_attacked: bool) -> ReturnCode
// {         js_unwrap!(@{self.as_ref()}.notifyWhenAttacked(@
// {notify_when_attacked}))     }
//     fn as_structure(self) -> Structure
//     where
//         Self: SizedRoomObject,
//     {
//         Into::<Reference>::into(self)
//             .into_expected_type()
//             .expect("expected converting a StructureProperties to a Structure
// would succeed.")     }
// }

// /// Trait for all wrappers over Screeps JavaScript objects extending
// /// the `OwnedStructure` class.
// ///
// /// # Contracts
// ///
// /// The reference returned by `AsRef<Reference>::as_ref` must reference a
// /// JavaScript object extending the `OwnedStructure` class.
// pub unsafe trait OwnedStructureProperties: StructureProperties {
//     /// Whether this structure is owned by you (in JS: `my || false`)
//     fn my(&self) -> bool {
//         js_unwrap!(@{self.as_ref()}.my || false)
//     }
//     /// Whether this structure is currently owned by someone (in JS: `owner
// !==     /// undefined`)
//     fn has_owner(&self) -> bool {
//         js_unwrap!(@{self.as_ref()}.owner !== undefined)
//     }
//     /// The name of the owner of this structure, if any.
//     fn owner_name(&self) -> Option<String> {
//         (js! {
//             var self = @{self.as_ref()};
//             if (self.owner) {
//                 return self.owner.username;
//             } else {
//                 return null;
//             }
//         })
//         .try_into()
//         .expect("expected OwnedStructure.owner.username to be a string")
//     }
//     /// Anonymize this as an owned structure.
//     fn as_owned_structure(self) -> OwnedStructure
//     where
//         Self: SizedRoomObject,
//     {
//         OwnedStructure(self.into())
//     }
// }

// /// Trait for all wrappers over Screeps JavaScript objects with a
// /// `store` property.
// ///
// /// # Contracts
// ///
// /// The JavaScript object referenced by the return of
// `AsRef<Reference>::as_ref` /// must have a `store` property.
// ///
// /// The `store` property must be a dict from string resource types to
// integers, /// and have the `getCapacity`, `getFreeCapacity`, and
// `getUsedCapacity` /// functions
// pub unsafe trait HasStore: RoomObjectProperties {
//     fn store_total(&self) -> u32 {
//         js_unwrap!(_.sum(@{self.as_ref()}.store))
//     }

//     fn store_types(&self) -> Vec<ResourceType> {
//         js_unwrap!(Object.keys(@{self.as_ref()}.store).
// map(__resource_type_str_to_num))     }

//     fn store_of(&self, ty: ResourceType) -> u32 {
//         js_unwrap!(@{self.as_ref()}.store[__resource_type_num_to_str(@{ty as
// u32})] || 0)     }

//     fn energy(&self) -> u32 {
//         js_unwrap!(@{self.as_ref()}.store[RESOURCE_ENERGY])
//     }

//     fn store_capacity(&self, resource: Option<ResourceType>) -> u32 {
//         match resource {
//             Some(ty) => {
//
// js_unwrap!(@{self.as_ref()}.store.getCapacity(__resource_type_num_to_str(@{ty
// as u32})) || 0)             }
//             None => js_unwrap!(@{self.as_ref()}.store.getCapacity() || 0),
//         }
//     }

//     fn store_free_capacity(&self, resource: Option<ResourceType>) -> i32 {
//         match resource {
//             Some(ty) => {
//
// js_unwrap!(@{self.as_ref()}.store.
// getFreeCapacity(__resource_type_num_to_str(@{ty as u32})) || 0)             }
//             None => js_unwrap!(@{self.as_ref()}.store.getFreeCapacity() ||
// 0),         }
//     }

//     fn store_used_capacity(&self, resource: Option<ResourceType>) -> u32 {
//         match resource {
//             Some(ty) => {
//
// js_unwrap!(@{self.as_ref()}.store.
// getUsedCapacity(__resource_type_num_to_str(@{ty as u32})) || 0)             }
//             None => js_unwrap!(@{self.as_ref()}.store.getUsedCapacity() ||
// 0),         }
//     }
// }

// /// Used to specify which structures can use their stored energy for spawning
// /// creeps.
// ///
// /// # Contract
// ///
// /// The reference returned from `AsRef<Reference>::as_ref` must be able to be
// /// used by a spawner to create a new creep.
// pub unsafe trait HasEnergyForSpawn: HasStore {}

// /// Trait for objects which have to cooldown.
// ///
// /// # Contract
// ///
// /// The reference returned from `AsRef<Reference>::as_ref` must be have a
// /// `cooldown` properties.
// pub unsafe trait HasCooldown: RoomObjectProperties {
//     fn cooldown(&self) -> u32 {
//         js_unwrap! { @{self.as_ref()}.cooldown }
//     }
// }

// /// Trait for objects which can decay.
// ///
// /// # Contract
// ///
// /// The reference returned from `AsRef<Reference>::as_ref` must be have a
// /// `ticksToDecay` properties.
// pub unsafe trait CanDecay: RoomObjectProperties {
//     fn ticks_to_decay(&self) -> u32 {
//         js_unwrap! { @{self.as_ref()}.ticksToDecay }
//     }
// }

// /// Trait for all wrappers over Screeps JavaScript objects which can be the
// /// target of `Creep.transfer`.
// ///
// /// # Contracts
// ///
// /// The reference returned from `AsRef<Reference>::as_ref` must be a valid
// /// target for `Creep.transfer`.
// pub unsafe trait Transferable: RoomObjectProperties {}

// /// Trait for all wrappers over Screeps JavaScript objects which can be the
// /// target of `Creep.withdraw`.
// ///
// /// # Contracts
// ///
// /// The reference returned from `AsRef<Reference>::as_ref` must be a valid
// /// target for `Creep.withdraw`.
// pub unsafe trait Withdrawable: RoomObjectProperties {}

// /// Trait for all wrappers over Screeps JavaScript objects which can be the
// /// target of `Creep.harvest`.
// ///
// /// # Contracts
// ///
// /// The reference returned from `AsRef<Reference>::as_ref` must be a valid
// /// target for `Creep.harvest`.
// pub unsafe trait Harvestable: RoomObjectProperties {}

// /// Trait for all wrappers over Screeps JavaScript objects which can be the
// /// target of `Creep.attack`.
// ///
// /// # Contracts
// ///
// /// The reference returned from `AsRef<Reference>::as_ref` must be a valid
// /// target for `Creep.attack`.
// pub unsafe trait Attackable: RoomObjectProperties {
//     /// Retrieve this hits of this structure, or `0` if this structure
// doesn't     /// have a hit count.
//     ///
//     /// For instance, this retrieves the hitpoints of a `Creep`. Or for a
//     /// `StructureWall` that's part of a novice area border, this will return
//     /// `0`.
//     fn hits(&self) -> u32 {
//         js_unwrap! { @{self.as_ref()}.hits || 0 }
//     }

//     /// Retrieve the maximum hits of this structure, or `0` if this structure
//     /// doesn't have a hit count.
//     ///
//     /// For instance, this retrieves the maximum full health of a `Creep`. Or
//     /// for a `StructureWall` that's part of a novice area border, this will
//     /// return `0`.
//     fn hits_max(&self) -> u32 {
//         js_unwrap! { @{self.as_ref()}.hitsMax || 0 }
//     }
// }

// // NOTE: keep impls for Structure* in sync with accessor methods in
// // src/objects/structure.rs

// unsafe impl Transferable for StructureExtension {}
// unsafe impl Transferable for Creep {}
// unsafe impl Transferable for StructureContainer {}
// unsafe impl Transferable for StructureFactory {}
// unsafe impl Transferable for StructureLab {}
// unsafe impl Transferable for StructureLink {}
// unsafe impl Transferable for StructureNuker {}
// unsafe impl Transferable for StructureSpawn {}
// unsafe impl Transferable for StructureStorage {}
// unsafe impl Transferable for StructureTower {}
// unsafe impl Transferable for StructurePowerSpawn {}
// unsafe impl Transferable for StructureTerminal {}
// unsafe impl Transferable for PowerCreep {}

// // NOTE: keep impls for Structure* in sync with accessor methods in
// // src/objects/structure.rs

// unsafe impl Withdrawable for Ruin {}
// unsafe impl Withdrawable for StructureExtension {}
// unsafe impl Withdrawable for StructureContainer {}
// unsafe impl Withdrawable for StructureFactory {}
// unsafe impl Withdrawable for StructureLab {}
// unsafe impl Withdrawable for StructureLink {}
// unsafe impl Withdrawable for StructureSpawn {}
// unsafe impl Withdrawable for StructureStorage {}
// unsafe impl Withdrawable for StructureTower {}
// unsafe impl Withdrawable for StructurePowerSpawn {}
// unsafe impl Withdrawable for StructureTerminal {}
// unsafe impl Withdrawable for Tombstone {}

// unsafe impl Harvestable for Deposit {}
// unsafe impl Harvestable for Mineral {}
// unsafe impl Harvestable for Source {}

// // NOTE: keep impls for Structure* in sync with accessor methods in
// // src/objects/structure.rs

// unsafe impl Attackable for Creep {}
// unsafe impl Attackable for OwnedStructure {}
// unsafe impl Attackable for StructureContainer {}
// unsafe impl Attackable for StructureExtension {}
// unsafe impl Attackable for StructureExtractor {}
// unsafe impl Attackable for StructureFactory {}
// unsafe impl Attackable for StructureInvaderCore {}
// unsafe impl Attackable for StructureKeeperLair {}
// unsafe impl Attackable for StructureLab {}
// unsafe impl Attackable for StructureLink {}
// unsafe impl Attackable for StructureNuker {}
// unsafe impl Attackable for StructureObserver {}
// unsafe impl Attackable for StructurePowerBank {}
// unsafe impl Attackable for StructurePowerSpawn {}
// unsafe impl Attackable for StructureRampart {}
// unsafe impl Attackable for StructureRoad {}
// unsafe impl Attackable for StructureSpawn {}
// unsafe impl Attackable for StructureStorage {}
// unsafe impl Attackable for StructureTerminal {}
// unsafe impl Attackable for StructureTower {}
// unsafe impl Attackable for StructureWall {}
// unsafe impl Attackable for PowerCreep {}

// unsafe impl RoomObjectProperties for ConstructionSite {}
// unsafe impl RoomObjectProperties for Creep {}
// unsafe impl RoomObjectProperties for Deposit {}
// unsafe impl RoomObjectProperties for Flag {}
// unsafe impl RoomObjectProperties for Mineral {}
// unsafe impl RoomObjectProperties for Nuke {}
// unsafe impl RoomObjectProperties for OwnedStructure {}
// unsafe impl RoomObjectProperties for Resource {}
// unsafe impl RoomObjectProperties for RoomObject {}
// unsafe impl RoomObjectProperties for Ruin {}
// unsafe impl RoomObjectProperties for Source {}
// unsafe impl RoomObjectProperties for StructureContainer {}
// unsafe impl RoomObjectProperties for StructureController {}
// unsafe impl RoomObjectProperties for StructureExtension {}
// unsafe impl RoomObjectProperties for StructureExtractor {}
// unsafe impl RoomObjectProperties for StructureFactory {}
// unsafe impl RoomObjectProperties for StructureInvaderCore {}
// unsafe impl RoomObjectProperties for StructureKeeperLair {}
// unsafe impl RoomObjectProperties for StructureLab {}
// unsafe impl RoomObjectProperties for StructureLink {}
// unsafe impl RoomObjectProperties for StructureNuker {}
// unsafe impl RoomObjectProperties for StructureObserver {}
// unsafe impl RoomObjectProperties for StructurePowerBank {}
// unsafe impl RoomObjectProperties for StructurePowerSpawn {}
// unsafe impl RoomObjectProperties for StructurePortal {}
// unsafe impl RoomObjectProperties for StructureRampart {}
// unsafe impl RoomObjectProperties for StructureRoad {}
// unsafe impl RoomObjectProperties for StructureSpawn {}
// unsafe impl RoomObjectProperties for StructureStorage {}
// unsafe impl RoomObjectProperties for StructureTerminal {}
// unsafe impl RoomObjectProperties for StructureTower {}
// unsafe impl RoomObjectProperties for StructureWall {}
// unsafe impl RoomObjectProperties for Structure {}
// unsafe impl RoomObjectProperties for Tombstone {}
// unsafe impl RoomObjectProperties for PowerCreep {}

// impl_structure_properties! {
//     OwnedStructure,
//     Structure,
//     StructureContainer,
//     StructureController,
//     StructureExtension,
//     StructureExtractor,
//     StructureFactory,
//     StructureInvaderCore,
//     StructureKeeperLair,
//     StructureLab,
//     StructureLink,
//     StructureNuker,
//     StructureObserver,
//     StructurePowerBank,
//     StructurePowerSpawn,
//     StructurePortal,
//     StructureRampart,
//     StructureRoad,
//     StructureSpawn,
//     StructureStorage,
//     StructureTerminal,
//     StructureTower,
//     StructureWall
// }

// unsafe impl OwnedStructureProperties for OwnedStructure {}
// unsafe impl OwnedStructureProperties for StructureController {}
// unsafe impl OwnedStructureProperties for StructureExtension {}
// unsafe impl OwnedStructureProperties for StructureExtractor {}
// unsafe impl OwnedStructureProperties for StructureFactory {}
// unsafe impl OwnedStructureProperties for StructureInvaderCore {}
// unsafe impl OwnedStructureProperties for StructureKeeperLair {}
// unsafe impl OwnedStructureProperties for StructureLab {}
// unsafe impl OwnedStructureProperties for StructureLink {}
// unsafe impl OwnedStructureProperties for StructureNuker {}
// unsafe impl OwnedStructureProperties for StructureObserver {}
// unsafe impl OwnedStructureProperties for StructurePowerBank {}
// unsafe impl OwnedStructureProperties for StructurePowerSpawn {}
// unsafe impl OwnedStructureProperties for StructureRampart {}
// unsafe impl OwnedStructureProperties for StructureSpawn {}
// unsafe impl OwnedStructureProperties for StructureStorage {}
// unsafe impl OwnedStructureProperties for StructureTerminal {}
// unsafe impl OwnedStructureProperties for StructureTower {}

// // NOTE: keep impls for Structure* in sync with accessor methods in
// // src/objects/structure.rs

// unsafe impl HasStore for Creep {}
// unsafe impl HasStore for Ruin {}
// unsafe impl HasStore for StructureContainer {}
// unsafe impl HasStore for StructureExtension {}
// unsafe impl HasStore for StructureFactory {}
// unsafe impl HasStore for StructureLab {}
// unsafe impl HasStore for StructureLink {}
// unsafe impl HasStore for StructureNuker {}
// unsafe impl HasStore for StructurePowerSpawn {}
// unsafe impl HasStore for StructureSpawn {}
// unsafe impl HasStore for StructureStorage {}
// unsafe impl HasStore for StructureTerminal {}
// unsafe impl HasStore for StructureTower {}
// unsafe impl HasStore for Tombstone {}
// unsafe impl HasStore for PowerCreep {}

// // NOTE: keep impls for Structure* in sync with accessor methods in
// // src/objects/structure.rs

// unsafe impl HasEnergyForSpawn for StructureExtension {}
// unsafe impl HasEnergyForSpawn for StructureSpawn {}

// // NOTE: keep impls for Structure* in sync with accessor methods in
// // src/objects/structure.rs

// unsafe impl HasCooldown for Deposit {}
// unsafe impl HasCooldown for StructureExtractor {}
// unsafe impl HasCooldown for StructureFactory {}
// unsafe impl HasCooldown for StructureLab {}
// unsafe impl HasCooldown for StructureLink {}
// unsafe impl HasCooldown for StructureNuker {}
// unsafe impl HasCooldown for StructureTerminal {}

// // NOTE: keep impls for Structure* in sync with accessor methods in
// // src/objects/structure.rs

// unsafe impl CanDecay for Deposit {}
// unsafe impl CanDecay for Ruin {}
// unsafe impl CanDecay for StructureContainer {}
// unsafe impl CanDecay for StructurePowerBank {}
// unsafe impl CanDecay for StructurePortal {}
// unsafe impl CanDecay for StructureRampart {}
// unsafe impl CanDecay for StructureRoad {}
// unsafe impl CanDecay for Tombstone {}
