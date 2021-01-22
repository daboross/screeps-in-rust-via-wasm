//! `*Type` constants.

// use std::{borrow::Cow, str::FromStr};
use enum_iterator::IntoEnumIterator;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use wasm_bindgen::prelude::*;

/// Translates `STRUCTURE_*` constants.
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, IntoEnumIterator)]
pub enum StructureType {
    Spawn = "spawn",
    Extension = "extension",
    Road = "road",
    Wall = "constructedWall",
    Rampart = "rampart",
    KeeperLair = "keeperLair",
    Portal = "portal",
    Controller = "controller",
    Link = "link",
    Storage = "storage",
    Tower = "tower",
    Observer = "observer",
    PowerBank = "powerBank",
    PowerSpawn = "powerSpawn",
    Extractor = "extractor",
    Lab = "lab",
    Terminal = "terminal",
    Container = "container",
    Nuker = "nuker",
    Factory = "factory",
    InvaderCore = "invaderCore",
}

impl StructureType {
    /// Translates the `CONSTRUCTION_COST` constant.
    #[inline]
    pub fn construction_cost(self) -> Option<u32> {
        use self::StructureType::*;

        let cost = match self {
            Spawn => 15_000,
            Extension => 3_000,
            Road => 300,
            Wall => 1,
            Rampart => 1,
            Link => 5_000,
            Storage => 30_000,
            Tower => 5_000,
            Observer => 8_000,
            PowerSpawn => 100_000,
            Extractor => 5_000,
            Lab => 50_000,
            Terminal => 100_000,
            Container => 5_000,
            Nuker => 100_000,
            Factory => 100_000,
            _ => return None,
        };
        Some(cost)
    }

    /// Translates the `CONTROLLER_STRUCTURES` constant
    #[inline]
    pub fn controller_structures(self, current_rcl: u32) -> u32 {
        use self::StructureType::*;

        match self {
            Spawn => match current_rcl {
                0 => 0,
                1..=6 => 1,
                7 => 2,
                _ => 3,
            },
            Extension => match current_rcl {
                0 | 1 => 0,
                2 => 5,
                3 => 10,
                4 => 20,
                5 => 30,
                6 => 40,
                7 => 50,
                _ => 60,
            },
            Road => 2500,
            Wall => match current_rcl {
                0 | 1 => 0,
                _ => 2500,
            },
            Rampart => match current_rcl {
                0 | 1 => 0,
                _ => 2500,
            },
            Link => match current_rcl {
                0..=4 => 0,
                5 => 2,
                6 => 3,
                7 => 4,
                _ => 6,
            },
            Storage => match current_rcl {
                0..=3 => 0,
                _ => 1,
            },
            Tower => match current_rcl {
                0 | 1 | 2 => 0,
                3 | 4 => 1,
                5 | 6 => 2,
                7 => 3,
                _ => 6,
            },
            Observer => match current_rcl {
                0..=7 => 0,
                _ => 1,
            },
            PowerSpawn => match current_rcl {
                0..=7 => 0,
                _ => 1,
            },
            Extractor => match current_rcl {
                0..=5 => 0,
                _ => 1,
            },
            Lab => match current_rcl {
                0..=5 => 0,
                6 => 3,
                7 => 6,
                _ => 10,
            },
            Terminal => match current_rcl {
                0..=5 => 0,
                _ => 1,
            },
            Container => 5,
            Nuker => match current_rcl {
                0..=7 => 0,
                _ => 1,
            },
            Factory => match current_rcl {
                0..=6 => 0,
                _ => 1,
            },
            _ => 0,
        }
    }

    /// Translates the `*_HITS` constants, initial hits for structures
    #[inline]
    pub fn initial_hits(self) -> Option<u32> {
        use self::StructureType::*;
        use super::numbers::*;

        let hits = match self {
            Spawn => SPAWN_HITS,
            Extension => EXTENSION_HITS,
            Road => ROAD_HITS,
            Wall => WALL_HITS,
            Rampart => RAMPART_HITS,
            Link => LINK_HITS,
            Storage => STORAGE_HITS,
            Tower => TOWER_HITS,
            Observer => OBSERVER_HITS,
            PowerBank => POWER_BANK_HITS,
            PowerSpawn => POWER_SPAWN_HITS,
            Extractor => EXTENSION_HITS,
            Lab => LAB_HITS,
            Terminal => TERMINAL_HITS,
            Container => CONTAINER_HITS,
            Nuker => NUKER_HITS,
            Factory => FACTORY_HITS,
            InvaderCore => INVADER_CORE_HITS,
            _ => return None,
        };
        Some(hits)
    }
}

/// Translates `SUBSCRIPTION_TOKEN` and `INTERSHARD_RESOURCES` constants.
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, IntoEnumIterator)]
pub enum IntershardResourceType {
    // no longer used, not implemented
    // SubscriptionToken = "token",
    CpuUnlock = "cpuUnlock",
    Pixel = "pixel",
    AccessKey = "AccessKey",
}

/// Resource type constant for all possible types of resources.
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, IntoEnumIterator)]
pub enum ResourceType {
    Energy = "energy",
    Power = "power",
    Hydrogen = "H",
    Oxygen = "O",
    Utrium = "U",
    Lemergium = "L",
    Keanium = "K",
    Zynthium = "Z",
    Catalyst = "X",
    Ghodium = "G",
    Silicon = "silicon",
    Metal = "metal",
    Biomass = "biomass",
    Mist = "mist",
    Hydroxide = "OH",
    ZynthiumKeanite = "ZK",
    UtriumLemergite = "UL",
    UtriumHydride = "UH",
    UtriumOxide = "UO",
    KeaniumHydride = "KH",
    KeaniumOxide = "KO",
    LemergiumHydride = "LH",
    LemergiumOxide = "LO",
    ZynthiumHydride = "ZH",
    ZynthiumOxide = "ZO",
    GhodiumHydride = "GH",
    GhodiumOxide = "GO",
    UtriumAcid = "UH2O",
    UtriumAlkalide = "UHO2",
    KeaniumAcid = "KH2O",
    KeaniumAlkalide = "KHO2",
    LemergiumAcid = "LH2O",
    LemergiumAlkalide = "LHO2",
    ZynthiumAcid = "ZH2O",
    ZynthiumAlkalide = "ZHO2",
    GhodiumAcid = "GH2O",
    GhodiumAlkalide = "GHO2",
    CatalyzedUtriumAcid = "XUH2O",
    CatalyzedUtriumAlkalide = "XUHO2",
    CatalyzedKeaniumAcid = "XKH2O",
    CatalyzedKeaniumAlkalide = "XKHO2",
    CatalyzedLemergiumAcid = "XLH2O",
    CatalyzedLemergiumAlkalide = "XLHO2",
    CatalyzedZynthiumAcid = "XZH2O",
    CatalyzedZynthiumAlkalide = "XZHO2",
    CatalyzedGhodiumAcid = "XGH2O",
    CatalyzedGhodiumAlkalide = "XGHO2",
    Ops = "ops",
    UtriumBar = "utrium_bar",
    LemergiumBar = "lemergium_bar",
    ZynthiumBar = "zynthium_bar",
    KeaniumBar = "keanium_bar",
    GhodiumMelt = "ghodium_melt",
    Oxidant = "oxidant",
    Reductant = "reductant",
    Purifier = "purifier",
    Battery = "battery",
    Composite = "composite",
    Crystal = "crystal",
    Liquid = "liquid",
    Wire = "wire",
    Switch = "switch",
    Transistor = "transistor",
    Microchip = "microchip",
    Circuit = "circuit",
    Device = "device",
    Cell = "cell",
    Phlegm = "phlegm",
    Tissue = "tissue",
    Muscle = "muscle",
    Organoid = "organoid",
    Organism = "organism",
    Alloy = "alloy",
    Tube = "tube",
    Fixtures = "fixtures",
    Frame = "frame",
    Hydraulics = "hydraulics",
    Machine = "machine",
    Condensate = "condensate",
    Concentrate = "concentrate",
    Extract = "extract",
    Spirit = "spirit",
    Emanation = "emanation",
    Essence = "essence",
    #[cfg(feature = "enable-score")]
    Score = "score",
}

// todo, can we make this an enum wrapper around both normal and inter-shard types before instead of this?
// or at least add fns to convert back and forth
/// Translates all resource types that can be used on the market
/// Translates `SUBSCRIPTION_TOKEN` and `INTERSHARD_RESOURCES` constants.
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, IntoEnumIterator)]
pub enum MarketResourceType {
    Energy = "energy",
    Power = "power",
    Hydrogen = "H",
    Oxygen = "O",
    Utrium = "U",
    Lemergium = "L",
    Keanium = "K",
    Zynthium = "Z",
    Catalyst = "X",
    Ghodium = "G",
    Silicon = "silicon",
    Metal = "metal",
    Biomass = "biomass",
    Mist = "mist",
    Hydroxide = "OH",
    ZynthiumKeanite = "ZK",
    UtriumLemergite = "UL",
    UtriumHydride = "UH",
    UtriumOxide = "UO",
    KeaniumHydride = "KH",
    KeaniumOxide = "KO",
    LemergiumHydride = "LH",
    LemergiumOxide = "LO",
    ZynthiumHydride = "ZH",
    ZynthiumOxide = "ZO",
    GhodiumHydride = "GH",
    GhodiumOxide = "GO",
    UtriumAcid = "UH2O",
    UtriumAlkalide = "UHO2",
    KeaniumAcid = "KH2O",
    KeaniumAlkalide = "KHO2",
    LemergiumAcid = "LH2O",
    LemergiumAlkalide = "LHO2",
    ZynthiumAcid = "ZH2O",
    ZynthiumAlkalide = "ZHO2",
    GhodiumAcid = "GH2O",
    GhodiumAlkalide = "GHO2",
    CatalyzedUtriumAcid = "XUH2O",
    CatalyzedUtriumAlkalide = "XUHO2",
    CatalyzedKeaniumAcid = "XKH2O",
    CatalyzedKeaniumAlkalide = "XKHO2",
    CatalyzedLemergiumAcid = "XLH2O",
    CatalyzedLemergiumAlkalide = "XLHO2",
    CatalyzedZynthiumAcid = "XZH2O",
    CatalyzedZynthiumAlkalide = "XZHO2",
    CatalyzedGhodiumAcid = "XGH2O",
    CatalyzedGhodiumAlkalide = "XGHO2",
    Ops = "ops",
    UtriumBar = "utrium_bar",
    LemergiumBar = "lemergium_bar",
    ZynthiumBar = "zynthium_bar",
    KeaniumBar = "keanium_bar",
    GhodiumMelt = "ghodium_melt",
    Oxidant = "oxidant",
    Reductant = "reductant",
    Purifier = "purifier",
    Battery = "battery",
    Composite = "composite",
    Crystal = "crystal",
    Liquid = "liquid",
    Wire = "wire",
    Switch = "switch",
    Transistor = "transistor",
    Microchip = "microchip",
    Circuit = "circuit",
    Device = "device",
    Cell = "cell",
    Phlegm = "phlegm",
    Tissue = "tissue",
    Muscle = "muscle",
    Organoid = "organoid",
    Organism = "organism",
    Alloy = "alloy",
    Tube = "tube",
    Fixtures = "fixtures",
    Frame = "frame",
    Hydraulics = "hydraulics",
    Machine = "machine",
    Condensate = "condensate",
    Concentrate = "concentrate",
    Extract = "extract",
    Spirit = "spirit",
    Emanation = "emanation",
    Essence = "essence",
    CpuUnlock = "cpuUnlock",
    Pixel = "pixel",
    AccessKey = "AccessKey",
}

#[derive(Copy, Clone, Debug)]
pub enum Boost {
    Harvest(u32),
    BuildAndRepair(f32),
    Dismantle(u32),
    UpgradeController(f32),
    Attack(u32),
    RangedAttack(u32),
    Heal(u32),
    Carry(u32),
    Move(u32),
    Tough(f32),
}

impl ResourceType {
    /// Translates the `BOOSTS` constant.
    #[inline]
    pub fn boost(self) -> Option<Boost> {
        use ResourceType::*;
        let boost = match self {
            // these comments copied directly from JavaScript 'constants.js' file.
            // UH: {
            //     attack: 2
            // },
            UtriumHydride => Boost::Attack(2),
            // UH2O: {
            //     attack: 3
            // },
            UtriumAcid => Boost::Attack(3),
            // XUH2O: {
            //     attack: 4
            // }
            CatalyzedUtriumAcid => Boost::Attack(4),
            // UO: {
            //     harvest: 3
            // },
            UtriumOxide => Boost::Harvest(3),
            // UHO2: {
            //     harvest: 5
            // },
            UtriumAlkalide => Boost::Harvest(5),
            // XUHO2: {
            //     harvest: 7
            // },
            CatalyzedUtriumAlkalide => Boost::Harvest(7),
            // KH: {
            //     capacity: 2
            // },
            KeaniumHydride => Boost::Carry(2),
            // KH2O: {
            //     capacity: 3
            // },
            KeaniumAcid => Boost::Carry(3),
            // XKH2O: {
            //     capacity: 4
            // }
            CatalyzedKeaniumAcid => Boost::Carry(4),
            // KO: {
            //     rangedAttack: 2,
            //     rangedMassAttack: 2
            // },
            KeaniumOxide => Boost::RangedAttack(2),
            // KHO2: {
            //     rangedAttack: 3,
            //     rangedMassAttack: 3
            // },
            KeaniumAlkalide => Boost::RangedAttack(4),
            // XKHO2: {
            //     rangedAttack: 4,
            //     rangedMassAttack: 4
            // }
            CatalyzedKeaniumAlkalide => Boost::RangedAttack(4),
            // LH: {
            //     build: 1.5,
            //     repair: 1.5
            // },
            LemergiumHydride => Boost::BuildAndRepair(1.5),
            // LH2O: {
            //     build: 1.8,
            //     repair: 1.8
            // },
            LemergiumAcid => Boost::BuildAndRepair(1.8),
            // XLH2O: {
            //     build: 2,
            //     repair: 2
            // },
            CatalyzedLemergiumAcid => Boost::BuildAndRepair(2.0),
            // LO: {
            //     heal: 2,
            //     rangedHeal: 2
            // },
            LemergiumOxide => Boost::Heal(2),
            // LHO2: {
            //     heal: 3,
            //     rangedHeal: 3
            // },
            LemergiumAlkalide => Boost::Heal(3),
            // XLHO2: {
            //     heal: 4,
            //     rangedHeal: 4
            // }
            CatalyzedLemergiumAlkalide => Boost::Heal(4),
            // ZH: {
            //     dismantle: 2
            // },
            ZynthiumHydride => Boost::Dismantle(2),
            // ZH2O: {
            //     dismantle: 3
            // },
            ZynthiumAcid => Boost::Dismantle(3),
            // XZH2O: {
            //     dismantle: 4
            // },
            CatalyzedZynthiumAcid => Boost::Dismantle(4),
            // ZO: {
            //     fatigue: 2
            // },
            ZynthiumOxide => Boost::Move(2),
            // ZHO2: {
            //     fatigue: 3
            // },
            ZynthiumAlkalide => Boost::Move(3),
            // XZHO2: {
            //     fatigue: 4
            // }
            CatalyzedZynthiumAlkalide => Boost::Move(4),
            // GH: {
            //     upgradeController: 1.5
            // },
            GhodiumHydride => Boost::UpgradeController(1.5),
            // GH2O: {
            //     upgradeController: 1.8
            // },
            GhodiumAcid => Boost::UpgradeController(1.8),
            // XGH2O: {
            //     upgradeController: 2
            // }
            CatalyzedGhodiumAcid => Boost::UpgradeController(2.0),
            // GO: {
            //     damage: .7
            // },
            GhodiumOxide => Boost::Tough(0.7),
            // GHO2: {
            //     damage: .5
            // },
            GhodiumAlkalide => Boost::Tough(0.5),
            // XGHO2: {
            //     damage: .3
            // }
            CatalyzedGhodiumAlkalide => Boost::Tough(0.3),
            // non-boost resources
            _ => return None,
        };
        Some(boost)
    }
}

// /// Translates market resource types which can include both `RESOURCE_*`
// /// and `INTERSHARD_RESOURCES` constants.
// #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
// pub enum MarketResourceType {
//     Resource(ResourceType),
//     IntershardResource(IntershardResourceType),
// }
//
// impl MarketResourceType {
//     /// Helper function for deserializing from a string rather than a fake
//     /// integer value.
//     pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
//         let s: Cow<'de, str> = Cow::deserialize(d)?;

//         ResourceType::from_str(&s)
//             .map(|ty| MarketResourceType::Resource(ty))
//             .or(IntershardResourceType::from_str(&s)
//                 .map(|ty| MarketResourceType::IntershardResource(ty)))
//             .map_err(|_| {
//                 D::Error::invalid_value(
//                     Unexpected::Str(&s),
//                     &"a known constant string in RESOURCES_ALL or INTERSHARD_RESOURCES",
//                 )
//             })
//     }
// }

// impl<'de> Deserialize<'de> for MarketResourceType {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         use IntershardResourceType::*;
//         use MarketResourceType::*;
//         use ResourceType::*;

//         let resource = u16::deserialize(deserializer)?;
//         let resource_type = match resource {
//             1 => Resource(Energy),
//             2 => Resource(Power),
//             3 => Resource(Hydrogen),
//             4 => Resource(Oxygen),
//             5 => Resource(Utrium),
//             6 => Resource(Lemergium),
//             7 => Resource(Keanium),
//             8 => Resource(Zynthium),
//             9 => Resource(Catalyst),
//             10 => Resource(Ghodium),
//             11 => Resource(Hydroxide),
//             12 => Resource(ZynthiumKeanite),
//             13 => Resource(UtriumLemergite),
//             14 => Resource(UtriumHydride),
//             15 => Resource(UtriumOxide),
//             16 => Resource(KeaniumHydride),
//             17 => Resource(KeaniumOxide),
//             18 => Resource(LemergiumHydride),
//             19 => Resource(LemergiumOxide),
//             20 => Resource(ZynthiumHydride),
//             21 => Resource(ZynthiumOxide),
//             22 => Resource(GhodiumHydride),
//             23 => Resource(GhodiumOxide),
//             24 => Resource(UtriumAcid),
//             25 => Resource(UtriumAlkalide),
//             26 => Resource(KeaniumAcid),
//             27 => Resource(KeaniumAlkalide),
//             28 => Resource(LemergiumAcid),
//             29 => Resource(LemergiumAlkalide),
//             30 => Resource(ZynthiumAcid),
//             31 => Resource(ZynthiumAlkalide),
//             32 => Resource(GhodiumAcid),
//             33 => Resource(GhodiumAlkalide),
//             34 => Resource(CatalyzedUtriumAcid),
//             35 => Resource(CatalyzedUtriumAlkalide),
//             36 => Resource(CatalyzedKeaniumAcid),
//             37 => Resource(CatalyzedKeaniumAlkalide),
//             38 => Resource(CatalyzedLemergiumAcid),
//             39 => Resource(CatalyzedLemergiumAlkalide),
//             40 => Resource(CatalyzedZynthiumAcid),
//             41 => Resource(CatalyzedZynthiumAlkalide),
//             42 => Resource(CatalyzedGhodiumAcid),
//             43 => Resource(CatalyzedGhodiumAlkalide),
//             44 => Resource(Ops),
//             45 => Resource(Silicon),
//             46 => Resource(Metal),
//             47 => Resource(Biomass),
//             48 => Resource(Mist),
//             49 => Resource(UtriumBar),
//             50 => Resource(LemergiumBar),
//             51 => Resource(ZynthiumBar),
//             52 => Resource(KeaniumBar),
//             53 => Resource(GhodiumMelt),
//             54 => Resource(Oxidant),
//             55 => Resource(Reductant),
//             56 => Resource(Purifier),
//             57 => Resource(Battery),
//             58 => Resource(Composite),
//             59 => Resource(Crystal),
//             60 => Resource(Liquid),
//             61 => Resource(Wire),
//             62 => Resource(Switch),
//             63 => Resource(Transistor),
//             64 => Resource(Microchip),
//             65 => Resource(Circuit),
//             66 => Resource(Device),
//             67 => Resource(Cell),
//             68 => Resource(Phlegm),
//             69 => Resource(Tissue),
//             70 => Resource(Muscle),
//             71 => Resource(Organoid),
//             72 => Resource(Organism),
//             73 => Resource(Alloy),
//             74 => Resource(Tube),
//             75 => Resource(Fixtures),
//             76 => Resource(Frame),
//             77 => Resource(Hydraulics),
//             78 => Resource(Machine),
//             79 => Resource(Condensate),
//             80 => Resource(Concentrate),
//             81 => Resource(Extract),
//             82 => Resource(Spirit),
//             83 => Resource(Emanation),
//             84 => Resource(Essence),
//             1001 => IntershardResource(SubscriptionToken),
//             1002 => IntershardResource(CPUUnlock),
//             1003 => IntershardResource(Pixel),
//             1004 => IntershardResource(AccessKey),
//             _ => {
//                 return Err(D::Error::invalid_value(
//                     Unexpected::Unsigned(resource as u64),
//                     &"a valid RESOURCES_ALL or INTERSHARD_RESOURCES type integer",
//                 ))
//             }
//         };
//         Ok(resource_type)
//     }
// }

// impl Serialize for MarketResourceType {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match self {
//             MarketResourceType::Resource(ty) => ty.serialize(serializer),
//             MarketResourceType::IntershardResource(ty) => ty.serialize(serializer),
//         }
//     }
// }

/// Translates the `POWER_CLASS` constants, which are classes of power creeps
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, IntoEnumIterator)]
pub enum PowerCreepClass {
    Operator = "operator",
}

// js_deserializable!(PowerCreepClass);

/// Translates the `PWR_*` constants, which are types of powers used by power
#[wasm_bindgen]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    FromPrimitive,
    Deserialize_repr,
    Serialize_repr,
    IntoEnumIterator,
)]
#[repr(u8)]
pub enum PowerType {
    GenerateOps = 1,
    OperateSpawn = 2,
    OperateTower = 3,
    OperateStorage = 4,
    OperateLab = 5,
    OperateExtension = 6,
    OperateObserver = 7,
    OperateTerminal = 8,
    DisruptSpawn = 9,
    DisruptTower = 10,
    Shield = 12,
    RegenSource = 13,
    RegenMineral = 14,
    DisruptTerminal = 15,
    OperatePower = 16,
    Fortify = 17,
    OperateController = 18,
    OperateFactory = 19,
}

// js_deserializable!(PowerType);

// /// Translates the `EFFECT_*` constants, which are natural effect types
// #[derive(
//     Copy, Clone, Debug, PartialEq, Eq, Hash, FromPrimitive, Serialize_repr, Deserialize_repr,
// )]
// #[repr(u16)]
// pub enum NaturalEffectType {
//     Invulnerability = 1001,
//     CollapseTimer = 1002,
// }

// js_deserializable!(NaturalEffectType);

// /// Translates effect types which can include both `PWR_*` and `EFFECT_*`
// /// constants.
// #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
// pub enum EffectType {
//     PowerEffect(PowerType),
//     NaturalEffect(NaturalEffectType),
// }

// impl<'de> Deserialize<'de> for EffectType {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let effect = u16::deserialize(deserializer)?;
//         let effect_type = match effect {
//             1 => EffectType::PowerEffect(PowerType::GenerateOps),
//             2 => EffectType::PowerEffect(PowerType::OperateSpawn),
//             3 => EffectType::PowerEffect(PowerType::OperateTower),
//             4 => EffectType::PowerEffect(PowerType::OperateStorage),
//             5 => EffectType::PowerEffect(PowerType::OperateLab),
//             6 => EffectType::PowerEffect(PowerType::OperateExtension),
//             7 => EffectType::PowerEffect(PowerType::OperateObserver),
//             8 => EffectType::PowerEffect(PowerType::OperateTerminal),
//             9 => EffectType::PowerEffect(PowerType::DisruptSpawn),
//             10 => EffectType::PowerEffect(PowerType::DisruptTower),
//             12 => EffectType::PowerEffect(PowerType::Shield),
//             13 => EffectType::PowerEffect(PowerType::RegenSource),
//             14 => EffectType::PowerEffect(PowerType::RegenMineral),
//             15 => EffectType::PowerEffect(PowerType::DisruptTerminal),
//             16 => EffectType::PowerEffect(PowerType::OperatePower),
//             17 => EffectType::PowerEffect(PowerType::Fortify),
//             18 => EffectType::PowerEffect(PowerType::OperateController),
//             19 => EffectType::PowerEffect(PowerType::OperateFactory),
//             1001 => EffectType::NaturalEffect(NaturalEffectType::Invulnerability),
//             1002 => EffectType::NaturalEffect(NaturalEffectType::CollapseTimer),
//             _ => {
//                 return Err(D::Error::invalid_value(
//                     Unexpected::Unsigned(effect as u64),
//                     &"a valid PWR_* or EFFECT_* type integer",
//                 ))
//             }
//         };

//         Ok(effect_type)
//     }
// }
