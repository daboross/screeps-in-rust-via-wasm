use enum_dispatch::enum_dispatch;
use wasm_bindgen::JsCast;

use crate::objects::*;

#[enum_dispatch(Attackable)]
pub enum AttackableObject {
    Creep,
    PowerCreep,
    StructureContainer,
    StructureExtension,
    StructureExtractor,
    StructureFactory,
    StructureInvaderCore,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePowerBank,
    StructurePowerSpawn,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureStorage,
    #[cfg(not(feature = "disable-terminal"))]
    StructureTerminal,
    StructureTower,
    StructureWall,
}

impl From<AttackableObject> for RoomObject {
    fn from(attackable: AttackableObject) -> Self {
        use AttackableObject::*;

        match attackable {
            Creep(o) => RoomObject::from(o),
            PowerCreep(o) => RoomObject::from(o),
            StructureContainer(o) => RoomObject::from(o),
            StructureExtension(o) => RoomObject::from(o),
            StructureExtractor(o) => RoomObject::from(o),
            StructureFactory(o) => RoomObject::from(o),
            StructureInvaderCore(o) => RoomObject::from(o),
            StructureLab(o) => RoomObject::from(o),
            StructureLink(o) => RoomObject::from(o),
            StructureNuker(o) => RoomObject::from(o),
            StructureObserver(o) => RoomObject::from(o),
            StructurePowerBank(o) => RoomObject::from(o),
            StructurePowerSpawn(o) => RoomObject::from(o),
            StructureRampart(o) => RoomObject::from(o),
            StructureRoad(o) => RoomObject::from(o),
            StructureSpawn(o) => RoomObject::from(o),
            StructureStorage(o) => RoomObject::from(o),
            #[cfg(not(feature = "disable-terminal"))]
            StructureTerminal(o) => RoomObject::from(o),
            StructureTower(o) => RoomObject::from(o),
            StructureWall(o) => RoomObject::from(o),
        }
    }
}

impl AsRef<RoomObject> for AttackableObject {
    fn as_ref(&self) -> &RoomObject {
        use AttackableObject::*;

        match self {
            Creep(o) => o.as_ref(),
            PowerCreep(o) => o.as_ref(),
            StructureContainer(o) => o.as_ref(),
            StructureExtension(o) => o.as_ref(),
            StructureExtractor(o) => o.as_ref(),
            StructureFactory(o) => o.as_ref(),
            StructureInvaderCore(o) => o.as_ref(),
            StructureLab(o) => o.as_ref(),
            StructureLink(o) => o.as_ref(),
            StructureNuker(o) => o.as_ref(),
            StructureObserver(o) => o.as_ref(),
            StructurePowerBank(o) => o.as_ref(),
            StructurePowerSpawn(o) => o.as_ref(),
            StructureRampart(o) => o.as_ref(),
            StructureRoad(o) => o.as_ref(),
            StructureSpawn(o) => o.as_ref(),
            StructureStorage(o) => o.as_ref(),
            #[cfg(not(feature = "disable-terminal"))]
            StructureTerminal(o) => o.as_ref(),
            StructureTower(o) => o.as_ref(),
            StructureWall(o) => o.as_ref(),
        }
    }
}

#[enum_dispatch(CanDecay)]
pub enum DecayingObject {
    Deposit,
    Ruin,
    #[cfg(feature = "enable-score")]
    ScoreContainer,
    StructureContainer,
    StructurePortal,
    StructurePowerBank,
    StructureRampart,
    StructureRoad,
    Tombstone,
}

#[enum_dispatch(HasCooldown)]
pub enum CooldownObject {
    Deposit,
    StructureExtractor,
    StructureFactory,
    StructureLab,
    StructureLink,
    StructureNuker,
    #[cfg(not(feature = "disable-terminal"))]
    StructureTerminal,
}

#[enum_dispatch(HasId)]
pub enum ObjectWithId {
    ConstructionSite,
    Creep,
    Deposit,
    Mineral,
    Nuke,
    PowerCreep,
    Resource,
    Ruin,
    #[cfg(feature = "enable-score")]
    ScoreCollector,
    #[cfg(feature = "enable-score")]
    ScoreContainer,
    Source,
    StructureContainer,
    StructureController,
    StructureExtension,
    StructureExtractor,
    StructureFactory,
    StructureInvaderCore,
    StructureKeeperLair,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePortal,
    StructurePowerBank,
    StructurePowerSpawn,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureStorage,
    #[cfg(not(feature = "disable-terminal"))]
    StructureTerminal,
    StructureTower,
    StructureWall,
    Tombstone,
}

#[enum_dispatch(HasPosition)]
pub enum ObjectWithPosition {
    ConstructionSite,
    Creep,
    Deposit,
    Flag,
    Mineral,
    Nuke,
    PowerCreep,
    Resource,
    RoomPosition,
    Ruin,
    #[cfg(feature = "enable-score")]
    ScoreCollector,
    #[cfg(feature = "enable-score")]
    ScoreContainer,
    Source,
    StructureContainer,
    StructureController,
    StructureExtension,
    StructureExtractor,
    StructureFactory,
    StructureInvaderCore,
    StructureKeeperLair,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePortal,
    StructurePowerBank,
    StructurePowerSpawn,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureStorage,
    #[cfg(not(feature = "disable-terminal"))]
    StructureTerminal,
    StructureTower,
    StructureWall,
    Tombstone,
}

#[enum_dispatch(HasStore)]
pub enum StoreObject {
    Creep,
    PowerCreep,
    Ruin,
    #[cfg(feature = "enable-score")]
    ScoreCollector,
    #[cfg(feature = "enable-score")]
    ScoreContainer,
    StructureContainer,
    StructureExtension,
    StructureFactory,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructurePowerSpawn,
    StructureSpawn,
    StructureStorage,
    #[cfg(not(feature = "disable-terminal"))]
    StructureTerminal,
    StructureTower,
    Tombstone,
}

/// Enum used for converting a [`Structure`] into a typed object of its specific
/// structure type.
#[enum_dispatch(OwnedStructureProperties)]
pub enum OwnedStructureObject {
    StructureController,
    StructureExtension,
    StructureExtractor,
    StructureFactory,
    StructureInvaderCore,
    StructureKeeperLair,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePowerSpawn,
    StructureRampart,
    StructureSpawn,
    StructureStorage,
    #[cfg(not(feature = "disable-terminal"))]
    StructureTerminal,
    StructureTower,
}

// todo TryFrom<Structure> for OwnedStructureObject

/// Any enum representing any game object that inherits the [`RoomObject`] type.
#[enum_dispatch(RoomObjectProperties)]
pub enum TypedRoomObject {
    ConstructionSite,
    Creep,
    Deposit,
    Flag,
    Mineral,
    Nuke,
    PowerCreep,
    Resource,
    Ruin,
    #[cfg(feature = "enable-score")]
    ScoreCollector,
    #[cfg(feature = "enable-score")]
    ScoreContainer,
    Source,
    StructureContainer,
    StructureController,
    StructureExtension,
    StructureExtractor,
    StructureFactory,
    StructureInvaderCore,
    StructureKeeperLair,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePortal,
    StructurePowerBank,
    StructurePowerSpawn,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureStorage,
    #[cfg(not(feature = "disable-terminal"))]
    StructureTerminal,
    StructureTower,
    StructureWall,
    Tombstone,
}

#[enum_dispatch(SharedCreepProperties)]
pub enum MovableObject {
    Creep,
    PowerCreep,
}

/// Enum used for converting a [`Structure`] into a typed object of its specific
/// structure type.
#[enum_dispatch(StructureProperties)]
pub enum StructureObject {
    StructureContainer,
    StructureController,
    StructureExtension,
    StructureExtractor,
    StructureFactory,
    StructureInvaderCore,
    StructureKeeperLair,
    StructureLab,
    StructureLink,
    StructureNuker,
    StructureObserver,
    StructurePortal,
    StructurePowerBank,
    StructurePowerSpawn,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureStorage,
    #[cfg(not(feature = "disable-terminal"))]
    StructureTerminal,
    StructureTower,
    StructureWall,
}

impl From<Structure> for StructureObject {
    fn from(structure: Structure) -> Self {
        use crate::constants::StructureType::*;

        match structure.structure_type() {
            Container => Self::StructureContainer(structure.unchecked_into()),
            Controller => Self::StructureController(structure.unchecked_into()),
            Extension => Self::StructureExtension(structure.unchecked_into()),
            Extractor => Self::StructureExtractor(structure.unchecked_into()),
            Factory => Self::StructureFactory(structure.unchecked_into()),
            InvaderCore => Self::StructureInvaderCore(structure.unchecked_into()),
            KeeperLair => Self::StructureKeeperLair(structure.unchecked_into()),
            Lab => Self::StructureLab(structure.unchecked_into()),
            Link => Self::StructureLink(structure.unchecked_into()),
            Nuker => Self::StructureNuker(structure.unchecked_into()),
            Observer => Self::StructureObserver(structure.unchecked_into()),
            Portal => Self::StructurePortal(structure.unchecked_into()),
            PowerBank => Self::StructurePowerBank(structure.unchecked_into()),
            PowerSpawn => Self::StructurePowerSpawn(structure.unchecked_into()),
            Rampart => Self::StructureRampart(structure.unchecked_into()),
            Road => Self::StructureRoad(structure.unchecked_into()),
            Spawn => Self::StructureSpawn(structure.unchecked_into()),
            Storage => Self::StructureStorage(structure.unchecked_into()),
            #[cfg(not(feature = "disable-terminal"))]
            Terminal => Self::StructureTerminal(structure.unchecked_into()),
            Tower => Self::StructureTower(structure.unchecked_into()),
            Wall => Self::StructureWall(structure.unchecked_into()),
            _ => panic!("unknown structure type for conversion into enum"),
        }
    }
}
