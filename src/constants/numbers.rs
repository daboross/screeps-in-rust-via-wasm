//! Plain data constants and functions returning plain data.
use super::types::{ResourceType, StructureType};

// OK and ERR_* defined in ReturnCode in `small_enums.rs`

// FIND_* defined in `small_enums.rs`

// directions and colors defined in `small_enums.rs`

// LOOK_* defined in `small_enums.rs`

// OBSTACLE_OBJECT_TYPES not yet implemented

// body parts and their costs defined in `small_enums.rs`

/// Initial ticks_to_live of a creep without any claim parts.
pub const CREEP_LIFE_TIME: u32 = 1500;
/// Initial ticks_to_live of a creep with at least one claim part.
pub const CREEP_CLAIM_LIFE_TIME: u32 = 600;
/// Percentage of TTL-adjusted creep resource costs added to tombstone.
///
/// When creeps die, if they had remaining TTL then a proportion of the cost of
/// the creep, both in energy per body part (bounded by
/// [`CREEP_PART_MAX_ENERGY`]) and resources spent on boosts, are placed in the
/// creep's tombstone.
pub const CREEP_CORPSE_RATE: f32 = 0.2;
/// The upper limit of energy per body part considered for return in tombstones
///
/// Energy spent on a creep body part for parts that cost more that this limit
/// will be capped at this value for tombstone resource calculations
pub const CREEP_PART_MAX_ENERGY: u32 = 125;

/// Store capacity provided per effective carry part.
pub const CARRY_CAPACITY: u32 = 50;
/// Energy harvested from a source per effective work part per
/// [`Creep::harvest`] action.
///
/// [`Creep::harvest`]: crate::objects::Creep::harvest
pub const HARVEST_POWER: u32 = 2;
/// Amount harvested from a mineral per effective work part per
/// [`Creep::harvest`] action.
///
/// [`Creep::harvest`]: crate::objects::Creep::harvest
pub const HARVEST_MINERAL_POWER: u32 = 1;
/// Amount harvested from a deposit per effective work part per
/// [`Creep::harvest`] action.
///
/// [`Creep::harvest`]: crate::objects::Creep::harvest
pub const HARVEST_DEPOSIT_POWER: u32 = 1;
/// Hits repaired per effective work part per [`Creep::repair`] action.
///
/// [`Creep::repair`]: crate::objects::Creep::repair
pub const REPAIR_POWER: u32 = 100;
/// Hits removed per effective work part per [`Creep::dismantle`] action.
///
/// [`Creep::dismantle`]: crate::objects::Creep::dismantle
pub const DISMANTLE_POWER: u32 = 50;
/// Construction site progress added per effective work part per
/// [`Creep::build`] action.
///
/// [`Creep::build`]: crate::objects::Creep::build
pub const BUILD_POWER: u32 = 5;
/// Hits of damage per effective attack part per [`Creep::attack`] action.
///
/// [`Creep::attack`]: crate::objects::Creep::attack
pub const ATTACK_POWER: u32 = 30;
/// Control points added per effective work part per
/// [`Creep::upgrade_controller`] action.
///
/// [`Creep::upgrade_controller`]: crate::objects::Creep::upgrade_controller
pub const UPGRADE_CONTROLLER_POWER: u32 = 1;
/// Hits of damage per effective ranged attack part per [`Creep::ranged_attack`]
/// action.
///
/// [`Creep::ranged_attack`]: crate::objects::Creep::ranged_attack
pub const RANGED_ATTACK_POWER: u32 = 10;
/// Hits of damage healed per effective heal part per [`Creep::heal`] action.
///
/// [`Creep::heal`]: crate::objects::Creep::heal
pub const HEAL_POWER: u32 = 12;
/// Hits of damage healed per effective heal part per [`Creep::ranged_heal`]
/// action.
///
/// [`Creep::ranged_heal`]: crate::objects::Creep::ranged_heal
pub const RANGED_HEAL_POWER: u32 = 4;
/// Cost in energy for each hit repaired by creeps before boosts.
pub const REPAIR_COST: f32 = 0.01;
/// Amount in energy returned to the dismantling creep per hit dismantled.
pub const DISMANTLE_COST: f32 = 0.005;

/// Hits lost per decay period for ramparts
pub const RAMPART_DECAY_AMOUNT: u32 = 300;
/// Ticks between rampart decays, losing [`RAMPART_DECAY_AMOUNT`] hits.
pub const RAMPART_DECAY_TIME: u32 = 100;
/// Initial hits for rampart structures when built; consider using the
/// [`StructureType::initial_hits`] function.
pub const RAMPART_HITS: u32 = 1;
/// Max rampart hits at RCL 2; consider using the [`rampart_hits_max`] function.
pub const RAMPART_HITS_MAX_RCL2: u32 = 300_000;
/// Max rampart hits at RCL 3; consider using the [`rampart_hits_max`] function.
pub const RAMPART_HITS_MAX_RCL3: u32 = 1_000_000;
/// Max rampart hits at RCL 4; consider using the [`rampart_hits_max`] function.
pub const RAMPART_HITS_MAX_RCL4: u32 = 3_000_000;
/// Max rampart hits at RCL 5; consider using the [`rampart_hits_max`] function.
pub const RAMPART_HITS_MAX_RCL5: u32 = 10_000_000;
/// Max rampart hits at RCL 6; consider using the [`rampart_hits_max`] function.
pub const RAMPART_HITS_MAX_RCL6: u32 = 30_000_000;
/// Max rampart hits at RCL 7; consider using the [`rampart_hits_max`] function.
pub const RAMPART_HITS_MAX_RCL7: u32 = 100_000_000;
/// Max rampart hits at RCL 8; consider using the [`rampart_hits_max`] function.
pub const RAMPART_HITS_MAX_RCL8: u32 = 300_000_000;

/// Translates the `RAMPART_HITS_MAX` constant, the maximum rampart hits for a
/// given room control level.
#[inline]
pub fn rampart_hits_max(rcl: u32) -> u32 {
    match rcl {
        r if r < 2 => 0,
        2 => RAMPART_HITS_MAX_RCL2,
        3 => RAMPART_HITS_MAX_RCL3,
        4 => RAMPART_HITS_MAX_RCL4,
        5 => RAMPART_HITS_MAX_RCL5,
        6 => RAMPART_HITS_MAX_RCL6,
        7 => RAMPART_HITS_MAX_RCL7,
        8 | _ => RAMPART_HITS_MAX_RCL8,
    }
}

/// Ticks to source regen after first [`Creep::harvest`] since last regen.
///
/// [`Creep::harvest`]: crate::objects::Creep::harvest
pub const ENERGY_REGEN_TIME: u32 = 300;
/// The total amount of a resource that must be accumulated in a dropped
/// [`Resource`] for one unit of that resource to decay each tick, rounded up.
///
/// [`Resource`]: crate::objects::Resource
pub const ENERGY_DECAY: u32 = 1000;

/// Initial hits for spawn structures; consider using the
/// [`StructureType::initial_hits`] function.
pub const SPAWN_HITS: u32 = 5000;
/// Initial energy for spawn structures when built.
pub const SPAWN_ENERGY_START: u32 = 300;
/// Maximum energy capacity of spawn structures.
pub const SPAWN_ENERGY_CAPACITY: u32 = 300;
/// Ticks taken to spawn each creep body part, before power creep effects.
pub const CREEP_SPAWN_TIME: u32 = 3;

/// Additional TTL bonus, and reduction in energy cost, when renewing a creep
/// compared to spawning.
///
/// Quoting from [`StructureSpawn.renewCreep`] documentation:
///
/// > Each execution increases the creep's timer by amount of ticks according to
/// this formula:
/// > `floor(600/body_size)`
/// >
/// > Energy required for each execution is determined using this formula:
/// >
/// > `ceil(creep_cost/2.5/body_size)`
///
/// 600 in the TTL addition formula is calculated as [`SPAWN_RENEW_RATIO`] *
/// [`CREEP_LIFE_TIME`] / [`CREEP_SPAWN_TIME`], or `1.2 * 1500 / 3 == 600`
///
/// 2.5 in the cost formula is calculated as [`CREEP_SPAWN_TIME`] /
/// [`SPAWN_RENEW_RATIO`], or `3 / 1.2 == 2.5`
///
/// [`StructureSpawn.renewCreep`]: https://docs.screeps.com/api/#StructureSpawn.renewCreep
pub const SPAWN_RENEW_RATIO: f32 = 1.2;

/// Source energy capacity immediately after regeneration in owned and reserved
/// rooms.
pub const SOURCE_ENERGY_CAPACITY: u32 = 3000;
/// Source energy capacity immediately after regeneration in neutral rooms.
pub const SOURCE_ENERGY_NEUTRAL_CAPACITY: u32 = 1500;
/// Source energy capacity immediately after regeneration in source keeper
/// (sector center) rooms.
pub const SOURCE_ENERGY_KEEPER_CAPACITY: u32 = 4000;

/// Initial hits for wall structures when built; consider using the
/// [`StructureType::initial_hits`] function.
pub const WALL_HITS: u32 = 1;
/// Maximum hits for wall structures.
pub const WALL_HITS_MAX: u32 = 300_000_000;

/// Initial hits for extension structures; consider using the
/// [`StructureType::initial_hits`] function.
pub const EXTENSION_HITS: u32 = 1000;

/// Translates the `EXTENSION_ENERGY_CAPACITY` constant, the energy capacity of
/// each source structure at a given room control level.
#[inline]
pub fn extension_energy_capacity(rcl: u32) -> u32 {
    match rcl {
        r if r < 7 => 50,
        7 => 100,
        8 | _ => 200,
    }
}

/// Maximum hits for road structures, before swamp/tunnel multipliers
///
/// Cost, hits, and decay rate are multiplied in the cases of building on
/// swamps or tunneling (see [`CONSTRUCTION_COST_ROAD_SWAMP_RATIO`] and
/// [`CONSTRUCTION_COST_ROAD_WALL_RATIO`] terrain multipliers).
pub const ROAD_HITS: u32 = 5000;
/// Tick penalty to a road's decay, per creep body part, when a creep steps on
/// it.
pub const ROAD_WEAROUT: u32 = 1;
/// Tick penalty to a road's decay when a power creep steps on it.
pub const ROAD_WEAROUT_POWER_CREEP: u32 = 100;
/// Hits lost upon decay for roads, before swamp/tunnel multipliers
///
/// Cost, hits, and decay rate are multiplied in the cases of building on
/// swamps or tunneling (see [`CONSTRUCTION_COST_ROAD_SWAMP_RATIO`] and
/// [`CONSTRUCTION_COST_ROAD_WALL_RATIO`] terrain multipliers).
pub const ROAD_DECAY_AMOUNT: u32 = 100;
/// Ticks between road decay events without traffic
///
/// The number of ticks between roads losing hits to decay, before reduction due
/// to creep traffic wear-out.
pub const ROAD_DECAY_TIME: u32 = 1000;

/// Initial hits for link structures; consider using the
/// [`StructureType::initial_hits`] function.
pub const LINK_HITS: u32 = 1000;
/// Energy capacity of link structures.
pub const LINK_CAPACITY: u32 = 800;
/// Ticks of link cooldown after transferring energy per distance to
/// destination.
pub const LINK_COOLDOWN: u32 = 1;
/// Percentage of the energy that is lost when transferred by
/// [`StructureLink::transfer_energy`].
///
/// [`StructureLink::transfer_energy`]:
/// crate::objects::StructureLink::transfer_energy
pub const LINK_LOSS_RATIO: f32 = 0.03;

/// Store capacity for storage structures without power creep effects.
pub const STORAGE_CAPACITY: u32 = 1_000_000;
/// Initial hits for storage structures; consider using the
/// [`StructureType::initial_hits`] function.
pub const STORAGE_HITS: u32 = 10_000;

// Structure types and CONSTRUCTION_COST defined in `types.rs`

/// Build, decay, and hits multiplier for roads built on swamp tiles.
pub const CONSTRUCTION_COST_ROAD_SWAMP_RATIO: u32 = 5;
/// Build, decay, and hits multiplier for roads built on natural wall tiles
/// (tunnels)
pub const CONSTRUCTION_COST_ROAD_WALL_RATIO: u32 = 150;

/// Translates the `CONTROLLER_LEVELS` constant.
///
/// The number of control points required to upgrade to the next level at each
/// room control level.
///
/// Returns `Some` for levels 1-7, `None` for all others.
#[inline]
pub fn controller_levels(current_rcl: u32) -> Option<u32> {
    match current_rcl {
        1 => Some(200),
        2 => Some(45_000),
        3 => Some(135_000),
        4 => Some(405_000),
        5 => Some(1_215_000),
        6 => Some(3_645_000),
        7 => Some(10_935_000),
        _ => None,
    }
}

// CONTROLLER_STRUCTURES defined in `types.rs`

/// Translates the `CONTROLLER_DOWNGRADE` constant, the maximum value of
/// [`StructureController::ticks_to_downgrade`] for each controller level.
///
/// Note that rooms that upgrade or downgrade in level start at half of this
/// maximum value for their new level, and that a controller will not upgrade to
/// the next level unless filled completely.
///
/// Returns `Some` for levels 1-8, `None` for all others.
///
/// [`StructureController::ticks_to_downgrade`]:
/// crate::objects::StructureController::ticks_to_downgrade
#[inline]
pub fn controller_downgrade(rcl: u32) -> Option<u32> {
    match rcl {
        1 => Some(20_000),
        2 => Some(10_000),
        3 => Some(20_000),
        4 => Some(40_000),
        5 => Some(80_000),
        6 => Some(120_000),
        7 => Some(150_000),
        8 => Some(200_000),
        _ => None,
    }
}

/// Ticks added to a [`StructureController::ticks_to_downgrade`] timer on each
/// tick that at least one creep successfully used
/// [`Creep::upgrade_controller`].
///
/// [`StructureController::ticks_to_downgrade`]:
/// crate::objects::StructureController::ticks_to_downgrade
/// [`Creep::upgrade_controller`]: crate::objects::Creep::upgrade_controller
pub const CONTROLLER_DOWNGRADE_RESTORE: u32 = 100;

/// Ticks under 50% of [`controller_downgrade`] max that safe mode becomes
/// unavailable.
///
/// Once the [`StructureController::ticks_to_downgrade`] timer is reduced to a
/// certain level by [`Creep::attack_controller`] or lack of
/// [`Creep::upgrade_controller`] activity, safe mode cannot be activated.
///
/// The point at which this occurs is half of the [`controller_downgrade`] total
/// for the current level, minus this amount. Note that because a room's
/// [`StructureController::ticks_to_downgrade`] is placed at exactly 50% after
/// an upgrade or downgrade.
///
/// Quoting from the [3.2.0 patch notes](https://blog.screeps.com/2018/12/changelog-2018-12-14/):
///
/// > * When the controller gains or loses one level, its downgrade timer is set
/// to 50% instead of 100%.
/// > * Safe mode activation unavailable period starts from this 50% point minus
/// 5000 ticks.
///
/// For example, a newly upgraded RCL7 room will have 75_000 ticks to downgrade
/// out of its 150_000 maximum, and safe mode becomes unavailable if the timer
/// falls below 70_000 ticks.
///
/// [`StructureController::ticks_to_downgrade`]:
/// crate::objects::StructureController::ticks_to_downgrade
/// [`Creep::attack_controller`]: crate::objects::Creep::attack_controller
/// [`Creep::upgrade_controller`]: crate::objects::Creep::upgrade_controller
pub const CONTROLLER_DOWNGRADE_SAFEMODE_THRESHOLD: u32 = 5000;
/// Additional decay of the [`StructureController::ticks_to_downgrade`] timer
/// caused by each claim part used per [`Creep::attack_controller`] action.
///
/// [`StructureController::ticks_to_downgrade`]:
/// crate::objects::StructureController::ticks_to_downgrade
/// [`Creep::attack_controller`]: crate::objects::Creep::attack_controller
pub const CONTROLLER_CLAIM_DOWNGRADE: u32 = 300;
/// Reservation ticks added per claim part per [`Creep::reserve_controller`]
/// action.
///
/// [`Creep::reserve_controller`]: crate::objects::Creep::reserve_controller
pub const CONTROLLER_RESERVE: u32 = 1;
/// Maximum ticks of reservation allowed on a controller
pub const CONTROLLER_RESERVE_MAX: u32 = 5000;
/// Maxiumum energy per tick that can be spent on [`Creep::upgrade_controller`]
/// at room control level 8 without power creep effects or boosts.
///
/// [`Creep::upgrade_controller`]: crate::objects::Creep::upgrade_controller
pub const CONTROLLER_MAX_UPGRADE_PER_TICK: u32 = 15;
/// A controller cannot be attacked or upgraded for this number of ticks after
/// one or more creeps successfully uses [`Creep::attack_controller`] against
/// it.
///
/// [`Creep::attack_controller`]: crate::objects::Creep::attack_controller
pub const CONTROLLER_ATTACK_BLOCKED_UPGRADE: u32 = 1000;
/// Ticks a controller cannot be attacked or upgraded for after a nuke
/// detonation in the room.
pub const CONTROLLER_NUKE_BLOCKED_UPGRADE: u32 = 200;

/// Duration of safe mode once activated, in ticks.
pub const SAFE_MODE_DURATION: u32 = 20_000;
/// Ticks since last safe mode activation before another is allowed.
pub const SAFE_MODE_COOLDOWN: u32 = 50_000;
/// Cost in Ghodium to add a safe mode activation to a controller via
/// [`Creep::generate_safe_mode`]
///
/// [`Creep::generate_safe_mode`]: crate::objects::Creep::generate_safe_mode
pub const SAFE_MODE_COST: u32 = 1000;

/// Initial hits for tower structures; consider using the
/// [`StructureType::initial_hits`] function.
pub const TOWER_HITS: u32 = 3000;
/// Energy capacity of tower structures.
pub const TOWER_CAPACITY: u32 = 1000;
/// Energy cost of each tower action.
pub const TOWER_ENERGY_COST: u32 = 10;
/// Tower damage per [`StructureTower::attack`] before range reduction.
///
/// [`StructureTower::attack`]: crate::objects::StructureTower::attack
pub const TOWER_POWER_ATTACK: u32 = 600;
/// Hits healed per [`StructureTower::heal`] before range reduction.
///
/// [`StructureTower::heal`]: crate::objects::StructureTower::heal
pub const TOWER_POWER_HEAL: u32 = 400;
/// Hits healed per [`StructureTower::repair`] before range reduction.
///
/// [`StructureTower::repair`]: crate::objects::StructureTower::repair
pub const TOWER_POWER_REPAIR: u32 = 800;
/// Tower actions at a range beyond this distance suffer falloff penalties - see
/// [`TOWER_FALLOFF`].
pub const TOWER_OPTIMAL_RANGE: u32 = 5;
/// Tower actions at a range greater than or equal to this distance suffer the
/// maxium falloff penalties - see [`TOWER_FALLOFF`].
pub const TOWER_FALLOFF_RANGE: u32 = 20;
/// Maximum percentage reduction in healing, repair, and attack effectiveness
/// for towers due to range.
///
/// When targets are at range beyond [`TOWER_OPTIMAL_RANGE`] until reaching the
/// maximum penalty at range [`TOWER_FALLOFF_RANGE`], the amount of healing,
/// repair, or damage done by a tower is reduced according to the formula
/// ([source]):
///
/// ```js
/// amount -= amount * TOWER_FALLOFF * (range - TOWER_OPTIMAL_RANGE) / (TOWER_FALLOFF_RANGE - TOWER_OPTIMAL_RANGE)
/// ```
///
/// [source]: https://github.com/screeps/engine/blob/f02d16a44a00c35615ae227fc72a3c9a07a6a39a/src/processor/intents/towers/attack.js#L38
pub const TOWER_FALLOFF: f32 = 0.75;

/// Initial hits for observer structures; consider using the
/// [`StructureType::initial_hits`] function.
pub const OBSERVER_HITS: u32 = 500;
/// Maximum range in rooms for [`StructureObserver::observe_room`].
///
/// [`StructureObserver::observe_room`]:
/// crate::objects::StructureObserver::observe_room
pub const OBSERVER_RANGE: u32 = 10;

/// Initial hits for power bank structures; consider using the
/// [`StructureType::initial_hits`] function.
pub const POWER_BANK_HITS: u32 = 2_000_000;
/// Maximum power capacity for power banks, before accounting for
/// [`POWER_BANK_CAPACITY_CRIT`].
pub const POWER_BANK_CAPACITY_MAX: u32 = 5000;
/// Maximum power capacity for power banks.
pub const POWER_BANK_CAPACITY_MIN: u32 = 500;
/// Chance of adding an additional [`POWER_BANK_CAPACITY_MAX`] to the random
/// power amount calculated when spawning a power bank. ([source])
///
/// [source]: https://github.com/screeps/backend-local/blob/81cbe7884afed23f3e1deaa3dcc77411fcbd697b/lib/cronjobs.js#L228
pub const POWER_BANK_CAPACITY_CRIT: f32 = 0.3;
/// Ticks for a power bank to decay if not destroyed.
pub const POWER_BANK_DECAY: u32 = 5000;
/// Percentage of damage dealt to power banks that is dealt back to attacking
/// creeps.
pub const POWER_BANK_HIT_BACK: f32 = 0.5;

/// Initial hits for power spawn structures; consider using the
/// [`StructureType::initial_hits`] function.
pub const POWER_SPAWN_HITS: u32 = 5000;
/// Maximum energy capacity for a power spawn to use in
/// [`StructurePowerSpawn::process_power`].
///
/// [`StructurePowerSpawn::process_power`]:
/// crate::objects::StructurePowerSpawn::process_power
pub const POWER_SPAWN_ENERGY_CAPACITY: u32 = 5000;
/// Maximum power capacity for a power spawn to use in
/// [`StructurePowerSpawn::process_power`].
///
/// [`StructurePowerSpawn::process_power`]:
/// crate::objects::StructurePowerSpawn::process_power
pub const POWER_SPAWN_POWER_CAPACITY: u32 = 100;
/// Energy consumed per point of power processed by
/// [`StructurePowerSpawn::process_power`].
///
/// [`StructurePowerSpawn::process_power`]:
/// crate::objects::StructurePowerSpawn::process_power
pub const POWER_SPAWN_ENERGY_RATIO: u32 = 50;

/// Initial hits for extractor structures; consider using the
/// [`StructureType::initial_hits`] function.
pub const EXTRACTOR_HITS: u32 = 500;
/// Ticks of cooldown for the extractor timer after tick that at least one creep
/// successfully used [`Creep::harvest`].
///
/// [`Creep::harvest`]: crate::objects::Creep::harvest
pub const EXTRACTOR_COOLDOWN: u32 = 5;

/// Initial hits for lab structures; consider using the
/// [`StructureType::initial_hits`] function.
pub const LAB_HITS: u32 = 500;
/// Store capacity for minerals in lab structures.
pub const LAB_MINERAL_CAPACITY: u32 = 3000;
/// Store capacity for energy in lab structures.
pub const LAB_ENERGY_CAPACITY: u32 = 2000;
/// Cost in energy to boost each creep body part.
pub const LAB_BOOST_ENERGY: u32 = 20;
/// Cost in boost minerals to boost each creep body part.
pub const LAB_BOOST_MINERAL: u32 = 30;
/// Amount of compounds consumed and produced per reaction, before power creep
/// effects.
pub const LAB_REACTION_AMOUNT: u32 = 5;
// LAB_COOLDOWN is marked as unused, not implemented
/// Energy refunded by unboost per creep body part (none)
pub const LAB_UNBOOST_ENERGY: u32 = 0;
/// Minerals spent on boosts refunded by unboost per creep body part.
pub const LAB_UNBOOST_MINERAL: u32 = 15;

/// Exponential growth rate of control points needed per global control level
/// (GCL).
///
/// Use the [`game::gcl::total_for_level`] function to calculate for each level
///
/// [`game::gcl::total_for_level`]: crate::game::gcl::total_for_level
pub const GCL_POW: f64 = 2.4;
/// Base growth rate of control points needed per global control level (GCL).
///
/// Use the [`game::gcl::total_for_level`] function to calculate for each level
///
/// [`game::gcl::total_for_level`]: crate::game::gcl::total_for_level
pub const GCL_MULTIPLY: u32 = 1_000_000;
/// Maximum GCL for players allowed to spawn in a Novice area.
pub const GCL_NOVICE: u32 = 3;

// TERRAIN_* defined in `small_enums.rs`

/// Maximum allowed construction sites at once per player.
pub const MAX_CONSTRUCTION_SITES: u32 = 100;
/// Maximum body parts per creep.
pub const MAX_CREEP_SIZE: u32 = 50;

/// Ticks after depletion for minerals to regenerate.
pub const MINERAL_REGEN_TIME: u32 = 50_000;

/// Translates the `MINERAL_MIN_AMOUNT` constant; currently unused in game (see
/// [`Density::amount`] instead).
///
/// [`Density::amount`]: crate::constants::Density::amount
#[inline]
pub fn mineral_min_amount(mineral: ResourceType) -> Option<u32> {
    match mineral {
        ResourceType::Hydrogen => Some(35_000),
        ResourceType::Oxygen => Some(35_000),
        ResourceType::Lemergium => Some(35_000),
        ResourceType::Keanium => Some(35_000),
        ResourceType::Zynthium => Some(35_000),
        ResourceType::Utrium => Some(35_000),
        ResourceType::Catalyst => Some(35_000),
        _ => None,
    }
}

/// Currently unused in game (see [`Density::probability`] instead).
///
/// [`Density::probability`]: crate::constants::Density::probability
pub const MINERAL_RANDOM_FACTOR: u32 = 2;

// MINERAL_DENSITY, MINERAL_DENSITY_PROBABILITY defined in `small_enums.rs`

/// Percentage chance to randomly determine a new density when currently
/// moderate or high density (a new density is always chosen when low or ultra).
pub const MINERAL_DENSITY_CHANGE: f32 = 0.05;

// DENSITY_* defined in `small_enums.rs`

/// Multiplier for deposit cooldown determination.
///
/// Cooldown is determined by the formula ([source]):
///
/// ```js
/// cooldown = ceil(DEPOSIT_EXHAUST_MULTIPLY * total_harvested ^ DEPOSIT_EXHAUST_POW)
/// ```
///
/// [source]: https://github.com/screeps/engine/blob/f02d16a44a00c35615ae227fc72a3c9a07a6a39a/src/processor/intents/creeps/harvest.js#L134
pub const DEPOSIT_EXHAUST_MULTIPLY: f32 = 0.001;

/// Exponential growth rate for deposit cooldown determination.
///
/// Cooldown is determined by the formula ([source]):
///
/// ```js
/// cooldown = ceil(DEPOSIT_EXHAUST_MULTIPLY * total_harvested ^ DEPOSIT_EXHAUST_POW)
/// ```
///
/// [source]: https://github.com/screeps/engine/blob/f02d16a44a00c35615ae227fc72a3c9a07a6a39a/src/processor/intents/creeps/harvest.js#L134
pub const DEPOSIT_EXHAUST_POW: f32 = 1.2;
/// Time since last harvest that a deposit will decay.
pub const DEPOSIT_DECAY_TIME: u32 = 50_000;

/// Initial hits for terminal structures; consider using the
/// [`StructureType::initial_hits`] function.
pub const TERMINAL_HITS: u32 = 3000;
/// Store capacity of terminal structures.
pub const TERMINAL_CAPACITY: u32 = 300_000;
/// Currently unused in game (see [`market::calc_transaction_cost`] and
/// [`TERMINAL_SEND_COST_SCALE`] instead).
///
/// [`market::calc_transaction_cost`]: [`crate::market::calc_transaction_cost`].
/// [`TERMINAL_SEND_COST_SCALE`]:
/// [`crate::constants::TERMINAL_SEND_COST_SCALE`].
pub const TERMINAL_SEND_COST: f32 = 0.1;
/// Currently unused in game.
pub const TERMINAL_MIN_SEND: u32 = 100;
/// Cooldown after a terminal is used before it can be used again.
pub const TERMINAL_COOLDOWN: u32 = 10;

/// Initial hits for container structures; consider using the
/// [`StructureType::initial_hits`] function.
pub const CONTAINER_HITS: u32 = 250_000;
/// Store capacity of container structures.
pub const CONTAINER_CAPACITY: u32 = 2000;
/// Hits lost on the container per decay.
pub const CONTAINER_DECAY: u32 = 5000;
/// Ticks between hit loss due to decay in unowned rooms.
pub const CONTAINER_DECAY_TIME: u32 = 100;
/// Ticks between hit loss due to decay in owned rooms.
pub const CONTAINER_DECAY_TIME_OWNED: u32 = 500;

/// Initial hits for nuker structures; consider using the
/// [`StructureType::initial_hits`] function.
pub const NUKER_HITS: u32 = 1000;
/// Cooldown for nuker structers after firing.
pub const NUKER_COOLDOWN: u32 = 100_000;
/// Energy capacity of the nuker, which is spent to fire a nuke.
pub const NUKER_ENERGY_CAPACITY: u32 = 300_000;
/// Ghodium capacity of the nuker, which is spent to fire a nuke.
pub const NUKER_GHODIUM_CAPACITY: u32 = 5000;
/// Tick until impact after firing a nuke.
pub const NUKE_LAND_TIME: u32 = 50_000;
/// Range in rooms of nukers.
pub const NUKE_RANGE: u32 = 10;
/// Damage in hits done by nukes at the point of impact.
pub const NUKE_DAMAGE_RANGE_0: u32 = 10_000_000;
/// Damage in hits done by nukes within range 2.
pub const NUKE_DAMAGE_RANGE_2: u32 = 5_000_000;

/// Initial hits for factory structures; consider using the
/// [`StructureType::initial_hits`] function.
pub const FACTORY_HITS: u32 = 1000;
/// Store capacity of factory structures.
pub const FACTORY_CAPACITY: u32 = 50_000;

/// Ticks per body part in total creep size that a creep's tombstone will
/// remain before decaying.
pub const TOMBSTONE_DECAY_PER_PART: u32 = 5;
/// Ticks that a power creep's tombstone will remain.
pub const TOMBSTONE_DECAY_POWER_CREEP: u32 = 500;

/// Ticks that ruins will last when structures are destroyed.
pub const RUIN_DECAY: u32 = 500;

/// Structures with special rules for their ruins' ticks to live, currently only
/// power banks.
#[inline]
pub fn ruin_decay_structures(structure_type: StructureType) -> Option<u32> {
    match structure_type {
        StructureType::PowerBank => Some(10),
        _ => None,
    }
}

/// Ticks that a portal that has reached the end of its stable lifetime will
/// remain before decaying.
pub const PORTAL_DECAY: u32 = 30_000;

// ORDER_SELL / ORDER_BUY defined in `small_enums.rs`

/// Percentage of order value in credits charged as a fee for market listings.
pub const MARKET_FEE: f32 = 0.05;

/// Maximum number of total orders a player is allowed to have on the market.
pub const MARKET_MAX_ORDERS: u32 = 300;
/// Time, in milliseconds, after which a market order will expire (30 days).
pub const MARKET_ORDER_LIFE_TIME: u32 = 30 * 24 * 3600 * 1000;

/// Maximum number of total flags a player is allowed to have on a shard.
pub const FLAGS_LIMIT: u32 = 10_000;

/// Cost, paid from [`CpuInfo::bucket`], to generate a pixel using
/// [`CpuInfo::generate_pixel`]
///
/// [`CpuInfo::bucket`]: crate::game::cpu::CpuInfo::bucket
/// [`CpuInfo::generate_pixel`]: crate::game::cpu::CpuInfo::generate_pixel
pub const PIXEL_CPU_COST: u32 = 10_000;

// Resources defined in `types.rs`

// REACTIONS defined in `recipes.rs`

// BOOSTS defined in `types.rs`

// REACTION_TIME defined in `recipes.rs`

/// The amount of time after spawning, in milliseconds, that random center room portals will become unstable and begin to decay, disappearing [`PORTAL_DECAY`] ticks later.
pub const PORTAL_UNSTABLE: u32 = 10 * 24 * 3600 * 1000;
/// Minimum time after a portal decays in a center room that a new portal will appear, in milliseconds.
pub const PORTAL_MIN_TIMEOUT: u32 = 12 * 24 * 3600 * 1000;
/// Maximum time after a portal decays in a center room that a new portal will appear, in milliseconds.
pub const PORTAL_MAX_TIMEOUT: u32 = 22 * 24 * 3600 * 1000;

/// Base value for power bank respawn time calculation.
///
/// Calculated respawn time falls randomly in a range from 50% to 125% of this
/// value. Determined by the formula ([source]):
///
/// ```js
/// respawnTime = Math.round(Math.random() * POWER_BANK_RESPAWN_TIME / 2 + POWER_BANK_RESPAWN_TIME * 0.75)
/// ```
///
/// [source]: https://github.com/screeps/backend-local/blob/81cbe7884afed23f3e1deaa3dcc77411fcbd697b/lib/cronjobs.js#L199
pub const POWER_BANK_RESPAWN_TIME: u32 = 50_000;

/// Base value for calculating the energy harvest amount that will trigger
/// invader spawns.
///
/// Calculated energy to be harvested in a given room until invader creeps spawn
/// falls randomly in a range from 70% to 130% of this value, then has a chance
/// to have a modifier applied according to the formula ([source]):
///
/// ```js
/// let invaderGoal = Math.floor(INVADERS_ENERGY_GOAL * (Math.random()*0.6 + 0.7));
/// if(Math.random() < 0.1) {
///     invaderGoal *= Math.floor( Math.random() > 0.5 ? 2 : 0.5 );
/// }
/// ```
///
/// Note that due to the use of `Math.floor`, the 0.5 will become a multiplier of 0, which won't be used; this bug is reported [here](https://screeps.com/forum/topic/2846)
///
/// [source]: https://github.com/screeps/backend-local/blob/81cbe7884afed23f3e1deaa3dcc77411fcbd697b/lib/cronjobs.js#L433
pub const INVADERS_ENERGY_GOAL: u32 = 100_000;

/// Owner username of system-owned structures and creeps.
pub const SYSTEM_USERNAME: &str = "Screeps";

/// Text added to signs of unowned rooms when a novice or respawn area is
/// planned for the sector.
pub const SIGN_PLANNED_AREA: &str = "A new Novice or Respawn Area is being planned somewhere \
     in this sector. Please make sure all important rooms are reserved.";

// EVENT_* constants in src/objects/impls/room.rs

/// Base growth rate of processed power needed per global power level (GPL).
///
/// Use the [`game::gpl::total_for_level`] function to calculate for each level
///
/// [`game::gpl::total_for_level`]: crate::game::gpl::total_for_level
pub const POWER_LEVEL_MULTIPLY: u32 = 1000;
/// Exponential growth rate of processed power needed per global power level
/// (GPL).
///
/// Use the [`game::gpl::total_for_level`] function to calculate for each level
///
/// [`game::gpl::total_for_level`]: crate::game::gpl::total_for_level
pub const POWER_LEVEL_POW: u32 = 2;
/// Time, in milliseconds, that a power creep must wait to respawn after dying.
pub const POWER_CREEP_SPAWN_COOLDOWN: u32 = 8 * 3600 * 1000;
/// Time, in milliseconds, after a deletion is started via
/// [`PowerCreep::delete`] that it can no longer be canceled.
///
/// [`PowerCreep::delete`]: crate::objects::PowerCreep::delete
pub const POWER_CREEP_DELETE_COOLDOWN: u32 = 24 * 3600 * 1000;
/// Maximum level for power creeps.
pub const POWER_CREEP_MAX_LEVEL: u32 = 25;
/// Maximum ticks to live for power creeps
pub const POWER_CREEP_LIFE_TIME: u32 = 5000;

// POWER_CLASS, PWR_*, EFFECT_* defined in `types.rs`

/// Initial hits for invader cores; consider using the
/// [`StructureType::initial_hits`] function.
pub const INVADER_CORE_HITS: u32 = 100_000;

/// Ticks between creation of invader cores in rooms in the sector for each
/// level of stronghold.
#[inline]
pub fn invader_core_expand_time(core_level: u32) -> Option<u32> {
    match core_level {
        1 => Some(4000),
        2 => Some(3500),
        3 => Some(3000),
        4 => Some(2500),
        5 => Some(2000),
        _ => None,
    }
}

/// The reservation points added or removed per tick by invader cores.
pub const INVADER_CORE_CONTROLLER_POWER: u32 = 2;
/// Duration of stronghold 'deployment', during which they are invulnerable.
///
/// The name reflects prior behavior by strongholds upgrading controllers
/// in owned rooms, which has been removed.  Now only used for the deploy timer.
pub const INVADER_CORE_CONTROLLER_DOWNGRADE: u32 = 5000;

/// Ticks per body part that invader cores of each level take to spawn defensive
/// creeps.
#[inline]
pub fn invader_core_creep_spawn_time(core_level: u32) -> Option<u32> {
    match core_level {
        1 => Some(0),
        2 => Some(6),
        3 => Some(3),
        4 => Some(2),
        5 => Some(1),
        _ => None,
    }
}

/// Rampart hits for each level of stronghold.
#[inline]
pub fn stronghold_rampart_hits(core_level: u32) -> Option<u32> {
    match core_level {
        1 => Some(100_000),
        2 => Some(200_000),
        3 => Some(500_000),
        4 => Some(1_000_000),
        5 => Some(2_000_000),
        _ => None,
    }
}

/// Average ticks until collapse for a stronghold.
///
/// Calculated lifetime includes a random 10% variation.  Value is determined by
/// the formula ([source]):
///
/// ```js
/// duration = Math.round(STRONGHOLD_DECAY_TICKS * (0.9 + Math.random() * 0.2))
/// ```
///
/// [source]: https://github.com/screeps/engine/blob/b2ac4720abe399837b0ba38712aaadfd4a9e9a7e/src/processor/intents/invader-core/stronghold/stronghold.js#L27
pub const STRONGHOLD_DECAY_TICKS: u32 = 75_000;

// POWER_INFO not yet implemented
// BODYPARTS_ALL, RESOURCES_ALL, COLORS_ALL not yet implemented
// INTERSHARD_RESOURCES defined in `types.rs`
// COMMODITIES defined in `recipes.rs`
