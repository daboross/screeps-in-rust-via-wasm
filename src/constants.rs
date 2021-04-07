//! Constants, most copied from [the game constants].
//!
//! Last updated on 2020-12-13, `e4589666113334bb1f967b9a5540b642141b6dab` from
//! <https://github.com/screeps/common/commits/master/lib/constants.js>.
//!
//! Currently missing:
//! - OBSTACLE_OBJECT_TYPES
//! - WORLD_WIDTH / WORLD_HEIGHT (deprecated in Screeps)
//! - BODYPARTS_ALL, RESOURCES_ALL, COLORS_ALL
//! - POWER_INFO
//!
//! # Notes on Deserialization
//!
//! There are two general types of enum constants in this file. Some are
//! represented by integers in the game - and those are represented by integers
//! in screeps. Their [`serde::Deserialize`], `TryFrom<Value>` and
//! [`num_traits::FromPrimitive`] implementations will convert from these
//! integers.
//!
//! The other type is enums represented by strings in the game, but integers in
//! this repository. This change in representation is done for efficiency, as
//! transferring strings from JavaScript to Rust is much slower than a single
//! integer.
//!
//! This second type of enum will also implement [`serde::Deserialize`],
//! `TryFrom<Value>`, but these will not convert from the made-up integer
//! values, and will fail converting from the constant strings from the game.
//!
//! To convert from constant strings, you have two options, depending on the
//! context.
//!
//! If you need to manually consume from a value in JavaScript, there are two
//! utility JavaScript functions per enum. They generally take the form
//! `__TYPE_num_to_str` and `__TYPE_str_to_num`. For example,
//! `__structure_type_num_to_str` and `__structure_type_str_to_num` convert
//! between [`StructureType`] integer representations and string
//! representations. See documentation on enums for more conversion functions.
//!
//! To use these, call the functions in JavaScript, like so:
//!
//! ```no_run
//! use screeps::{game, traits::TryInto, StructureType};
//! use stdweb::js;
//!
//! let spawns = game::spawns::values();
//! let r: StructureType = (js! {
//!     return __structure_type_str_to_num(@{spawns[0].as_ref()}.structureType);
//! })
//! .try_into()
//! .expect("expected structure type to convert successfully");
//! ```
//!
//! If you need to consume strings already in Rust, use either the [`FromStr`]
//! trait, or one of the `deserialize_from_str` functions on each of these
//! constants. To get back to these values from Rust native enums (instead of
//! the integer representation we serialize to), use the [`Display`] trait or
//! the `to_string` fuctions on the native enums.
//!
//! [the game constants]: https://github.com/screeps/common/blob/master/lib/constants.js
//! [`FromStr`]: std::str::FromStr
//! [`Display`]: std::fmt::Display
pub mod find;
pub mod look;
mod numbers;
mod recipes;
pub mod seasonal;
mod small_enums;
mod types;

pub use self::{
    find::FindConstant,
    look::{Look, LookConstant},
    numbers::*,
    recipes::FactoryRecipe,
    small_enums::*,
    types::*,
};

/// Re-export of all constants related to [`Creep`] behavior and operations.
///
/// [`Creep`]: crate::objects::Creep
pub mod creep {
    pub use super::{
        numbers::{
            ATTACK_POWER, BUILD_POWER, CARRY_CAPACITY, CREEP_CLAIM_LIFE_TIME, CREEP_CORPSE_RATE,
            CREEP_LIFE_TIME, CREEP_PART_MAX_ENERGY, CREEP_SPAWN_TIME, DISMANTLE_COST,
            HARVEST_DEPOSIT_POWER, HARVEST_MINERAL_POWER, HARVEST_POWER, HEAL_POWER,
            MAX_CREEP_SIZE, RANGED_HEAL_POWER, REPAIR_COST, REPAIR_POWER, SPAWN_RENEW_RATIO,
            UPGRADE_CONTROLLER_POWER,
        },
        small_enums::{Part, ReturnCode},
    };
}

/// Re-export of all constants related to structures.
pub mod structure {
    pub use super::{
        numbers::{
            extension_energy_capacity, invader_core_creep_spawn_time, invader_core_expand_time,
            rampart_hits_max, ruin_decay_structures, stronghold_rampart_hits,
            CONSTRUCTION_COST_ROAD_SWAMP_RATIO, CONSTRUCTION_COST_ROAD_WALL_RATIO,
            CONTAINER_CAPACITY, CONTAINER_DECAY, CONTAINER_DECAY_TIME, CONTAINER_DECAY_TIME_OWNED,
            CONTAINER_HITS, EXTENSION_HITS, EXTRACTOR_COOLDOWN, EXTRACTOR_HITS, FACTORY_CAPACITY,
            FACTORY_HITS, INVADER_CORE_CONTROLLER_DOWNGRADE, INVADER_CORE_CONTROLLER_POWER,
            INVADER_CORE_HITS, LAB_ENERGY_CAPACITY, LAB_HITS, LAB_MINERAL_CAPACITY, LINK_CAPACITY,
            LINK_COOLDOWN, LINK_HITS, LINK_LOSS_RATIO, NUKER_COOLDOWN, NUKER_ENERGY_CAPACITY,
            NUKER_GHODIUM_CAPACITY, NUKER_HITS, OBSERVER_HITS, POWER_BANK_HITS,
            POWER_SPAWN_ENERGY_CAPACITY, POWER_SPAWN_HITS, POWER_SPAWN_POWER_CAPACITY,
            RAMPART_DECAY_AMOUNT, RAMPART_DECAY_TIME, RAMPART_HITS, RAMPART_HITS_MAX_RCL2,
            RAMPART_HITS_MAX_RCL3, RAMPART_HITS_MAX_RCL4, RAMPART_HITS_MAX_RCL5,
            RAMPART_HITS_MAX_RCL6, RAMPART_HITS_MAX_RCL7, RAMPART_HITS_MAX_RCL8, ROAD_DECAY_AMOUNT,
            ROAD_DECAY_TIME, ROAD_HITS, ROAD_WEAROUT, ROAD_WEAROUT_POWER_CREEP, RUIN_DECAY,
            SPAWN_ENERGY_CAPACITY, SPAWN_HITS, STORAGE_CAPACITY, STORAGE_HITS,
            STRONGHOLD_DECAY_TICKS, TERMINAL_CAPACITY, TERMINAL_HITS, TERMINAL_SEND_COST,
            TOWER_CAPACITY, TOWER_HITS, WALL_HITS, WALL_HITS_MAX,
        },
        types::StructureType,
    };
}

/// Re-export of all constants related to [`Mineral`] behavior and
/// [`StructureLab`] operations.
///
/// [`StructureLab`]: crate::objects::StructureLab
/// [`Mineral`]: crate::objects::Mineral
pub mod minerals {
    pub use super::{
        numbers::{
            mineral_min_amount, LAB_BOOST_ENERGY, LAB_BOOST_MINERAL, LAB_ENERGY_CAPACITY,
            LAB_MINERAL_CAPACITY, LAB_REACTION_AMOUNT, LAB_UNBOOST_ENERGY, LAB_UNBOOST_MINERAL,
            MINERAL_DENSITY_CHANGE, MINERAL_RANDOM_FACTOR, MINERAL_REGEN_TIME,
        },
        small_enums::Density,
        types::ResourceType,
    };
}

/// Re-export of all constants related to [`StructureController`] behavior and
/// GCL.
///
/// [`StructureController`]: crate::objects::StructureController
pub mod control {
    pub use super::numbers::{
        controller_downgrade, controller_levels, CONTROLLER_ATTACK_BLOCKED_UPGRADE,
        CONTROLLER_CLAIM_DOWNGRADE, CONTROLLER_DOWNGRADE_RESTORE,
        CONTROLLER_DOWNGRADE_SAFEMODE_THRESHOLD, CONTROLLER_MAX_UPGRADE_PER_TICK,
        CONTROLLER_NUKE_BLOCKED_UPGRADE, CONTROLLER_RESERVE, CONTROLLER_RESERVE_MAX, GCL_MULTIPLY,
        GCL_NOVICE, GCL_POW, SAFE_MODE_COOLDOWN, SAFE_MODE_COST, SAFE_MODE_DURATION,
        SIGN_PLANNED_AREA, SYSTEM_USERNAME,
    };
}

/// Re-export of all constants related to power.
pub mod power {
    pub use super::{
        numbers::{
            POWER_BANK_CAPACITY_MAX, POWER_BANK_CAPACITY_MIN, POWER_BANK_DECAY, POWER_BANK_HITS,
            POWER_BANK_HIT_BACK, POWER_BANK_RESPAWN_TIME, POWER_CREEP_DELETE_COOLDOWN,
            POWER_CREEP_LIFE_TIME, POWER_CREEP_MAX_LEVEL, POWER_CREEP_SPAWN_COOLDOWN,
            POWER_LEVEL_MULTIPLY, POWER_LEVEL_POW, POWER_SPAWN_ENERGY_CAPACITY,
            POWER_SPAWN_ENERGY_RATIO, POWER_SPAWN_POWER_CAPACITY,
        },
        small_enums::PowerClass,
        types::PowerType,
    };
}

/// Re-export of all constants related to [`StructurePortal`] behavior.
///
/// [`StructurePortal`]: crate::objects::StructurePortal
pub mod portal {
    pub use super::numbers::{
        PORTAL_DECAY, PORTAL_MAX_TIMEOUT, PORTAL_MIN_TIMEOUT, PORTAL_UNSTABLE,
    };
}

/// Re-export of all constants related to [`Source`] behavior.
///
/// [`Source`]: crate::objects::Source
pub mod source {
    pub use super::numbers::{
        ENERGY_DECAY, ENERGY_REGEN_TIME, INVADERS_ENERGY_GOAL, SOURCE_ENERGY_CAPACITY,
        SOURCE_ENERGY_KEEPER_CAPACITY, SOURCE_ENERGY_NEUTRAL_CAPACITY,
    };
}

/// Re-export of all constants related to the [market].
///
/// [market]: crate::game::market
pub mod market {
    pub use super::numbers::{
        MARKET_FEE, MARKET_MAX_ORDERS, MARKET_ORDER_LIFE_TIME, TERMINAL_CAPACITY,
        TERMINAL_COOLDOWN, TERMINAL_MIN_SEND, TERMINAL_SEND_COST,
    };
}

/// Re-export of all constants related to [`StructureSpawn`] operations.
///
/// [`StructureSpawn`]: crate::objects::StructureSpawn
pub mod spawn {
    pub use super::numbers::{
        extension_energy_capacity, CREEP_SPAWN_TIME, ENERGY_REGEN_TIME, MAX_CREEP_SIZE,
        SPAWN_ENERGY_CAPACITY, SPAWN_ENERGY_START, SPAWN_RENEW_RATIO,
    };
}

/// Re-export of all constants related to [`StructureTower`] operations.
///
/// [`StructureTower`]: crate::objects::StructureTower
pub mod tower {
    pub use super::numbers::{
        TOWER_CAPACITY, TOWER_ENERGY_COST, TOWER_FALLOFF, TOWER_FALLOFF_RANGE, TOWER_OPTIMAL_RANGE,
        TOWER_POWER_ATTACK, TOWER_POWER_HEAL, TOWER_POWER_REPAIR,
    };
}

/// Re-export of all constants related to [`StructureNuker`] and [`Nuke`].
///
/// [`StructureNuker`]: crate::objects::StructureNuker
/// [`Nuke`]: crate::objects::Nuke
pub mod nuke {
    pub use super::numbers::{
        NUKER_COOLDOWN, NUKER_ENERGY_CAPACITY, NUKER_GHODIUM_CAPACITY, NUKE_DAMAGE_RANGE_0,
        NUKE_DAMAGE_RANGE_2, NUKE_LAND_TIME, NUKE_RANGE,
    };
}

/// Re-export of all constants related to [`StructureObserver`] operations.
///
/// [`StructureObserver`]: crate::objects::StructureObserver
pub mod observer {
    pub use super::numbers::OBSERVER_RANGE;
}

/// Re-export of all constants related to [`Resource`]s.
///
/// [`Resource`]: crate::objects::Resource
pub mod resource {
    pub use super::{numbers::ENERGY_DECAY, types::ResourceType};
}

/// Re-export of all constants related to [`Tombstone`]s.
///
/// [`Tombstone`]: crate::objects::Tombstone
pub mod tombstone {
    pub use super::numbers::{TOMBSTONE_DECAY_PER_PART, TOMBSTONE_DECAY_POWER_CREEP};
}

/// Re-export of all constants related to [`Flag`]s.
///
/// [`Flag`]: crate::objects::Flag
pub mod flags {
    pub use super::{numbers::FLAGS_LIMIT, small_enums::Color};
}
