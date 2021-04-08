//! Constants for use with the [`Room::find`] function.
//!
//! *Note:* Types in this module have purposefully ambiguous names, and are
//! intended to be used as, for example, `find::CREEPS`, not `CREEPS`.
//!
//! You can do this my importing the module itself, rather than any individual
//! constant, and then just referring to the constants relative to the module.
//!
//! # Example
//!
//! ```no_run
//! use screeps::{find, game, Room};
//!
//! let room: Room = game::rooms::get("E23S55".parse().unwrap()).unwrap();
//!
//! let creeps = room.find(find::CREEPS);
//! # let _ = creeps;
//! ```
//!
//! [`Room::find`]: crate::Room::find
//! [`objects::RoomObject`]: crate::RoomObject
use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};
use stdweb::Reference;

use crate::{
    local::Position,
    objects::{
        ConstructionSite, Creep, Deposit, Flag, Mineral, Nuke, OwnedStructure, PowerCreep,
        Resource, Ruin, Source, Structure, StructureSpawn, Tombstone,
    },
    traits::FromExpectedType,
};

#[cfg(feature = "enable-score")]
use crate::objects::{ScoreCollector, ScoreContainer};

#[cfg(feature = "enable-symbols")]
use crate::objects::{SymbolContainer, SymbolDecoder};

/// Trait representing things which can be used in the 'find' function.
///
/// Typically used with zero-sized structs in the
/// [`find`][crate::constants::find] module.
pub unsafe trait FindConstant {
    type Item: FromExpectedType<Reference>;

    fn find_code(&self) -> i16;
}

/// Useful for finding any [`RoomObject`] with
/// a dynamically-chosen find constant.
///
/// If you know ahead of time what constant you'll use, then the
/// all-upper-case constants in [this module][crate::constants::find] will
/// be more helpful, and won't require casting the result types.
///
/// *Note*: To avoid ambiguity with [`RoomObject`], you should refer to this
/// enum as `find::RoomObject` rather than importing it directly.
///
/// [`RoomObject`]: crate::objects::RoomObject
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i16)]
pub enum RoomObject {
    Creeps = 101,
    MyCreeps = 102,
    HostileCreeps = 103,
    SourcesActive = 104,
    Sources = 105,
    DroppedResources = 106,
    Structures = 107,
    MyStructures = 108,
    HostileStructures = 109,
    Flags = 110,
    ConstructionSites = 111,
    MySpawns = 112,
    HostileSpawns = 113,
    MyConstructionSites = 114,
    HostileConstructionSites = 115,
    Minerals = 116,
    Nukes = 117,
    Tombstones = 118,
    PowerCreeps = 119,
    MyPowerCreeps = 120,
    HostilePowerCreeps = 121,
    Deposits = 122,
    Ruins = 123,
    #[cfg(feature = "enable-score")]
    ScoreContainers = 10011,
    #[cfg(feature = "enable-score")]
    ScoreCollectors = 10012,
    #[cfg(feature = "enable-symbols")]
    SymbolContainers = 10021,
    #[cfg(feature = "enable-symbols")]
    SymbolDecoders = 10022,
}

unsafe impl FindConstant for RoomObject {
    type Item = crate::objects::RoomObject;

    #[inline]
    fn find_code(&self) -> i16 {
        *self as i16
    }
}

#[derive(
    Copy, Clone, Debug, FromPrimitive, Deserialize_repr, Serialize_repr, PartialEq, Eq, Hash,
)]
#[repr(i16)]
pub enum Exit {
    Top = 1,
    Right = 3,
    Bottom = 5,
    Left = 7,
    All = 10,
}

impl Exit {
    #[inline]
    pub fn top() -> Self {
        Exit::Top
    }

    #[inline]
    pub fn right() -> Self {
        Exit::Right
    }

    #[inline]
    pub fn bottom() -> Self {
        Exit::Bottom
    }

    #[inline]
    pub fn left() -> Self {
        Exit::Left
    }

    #[inline]
    pub fn all() -> Self {
        Exit::All
    }
}

unsafe impl FindConstant for Exit {
    type Item = Position;

    #[inline]
    fn find_code(&self) -> i16 {
        *self as i16
    }
}

typesafe_find_constants! {
    pub struct CREEPS = (101, Creep);
    pub struct MY_CREEPS = (102, Creep);
    pub struct HOSTILE_CREEPS = (103, Creep);
    pub struct SOURCES_ACTIVE = (104, Source);
    pub struct SOURCES = (105, Source);
    pub struct DROPPED_RESOURCES = (106, Resource);
    pub struct STRUCTURES = (107, Structure);
    pub struct MY_STRUCTURES = (108, OwnedStructure);
    pub struct HOSTILE_STRUCTURES = (109, OwnedStructure);
    pub struct FLAGS = (110, Flag);
    pub struct CONSTRUCTION_SITES = (111, ConstructionSite);
    pub struct MY_SPAWNS = (112, StructureSpawn);
    pub struct HOSTILE_SPAWNS = (113, StructureSpawn);
    pub struct MY_CONSTRUCTION_SITES = (114, ConstructionSite);
    pub struct HOSTILE_CONSTRUCTION_SITES = (115, ConstructionSite);
    pub struct MINERALS = (116, Mineral);
    pub struct NUKES = (117, Nuke);
    pub struct TOMBSTONES = (118, Tombstone);
    pub struct POWER_CREEPS = (119, PowerCreep);
    pub struct MY_POWER_CREEPS = (120, PowerCreep);
    pub struct HOSTILE_POWER_CREEPS = (121, PowerCreep);
    pub struct DEPOSITS = (122, Deposit);
    pub struct RUINS = (123, Ruin);
    pub struct EXIT_TOP = (Exit::Top as i16, Position);
    pub struct EXIT_RIGHT = (Exit::Right as i16, Position);
    pub struct EXIT_BOTTOM = (Exit::Bottom as i16, Position);
    pub struct EXIT_LEFT = (Exit::Left as i16, Position);
    pub struct EXIT = (Exit::All as i16, Position);
}

#[cfg(feature = "enable-score")]
typesafe_find_constants! {
    pub struct SCORE_CONTAINERS = (10011, ScoreContainer);
    pub struct SCORE_COLLECTORS = (10012, ScoreCollector);
}

#[cfg(feature = "enable-symbols")]
typesafe_find_constants! {
    pub struct SYMBOL_CONTAINERS = (10021, SymbolContainer);
    pub struct SYMBOL_DECODERS = (10022, SymbolDecoder);
}
