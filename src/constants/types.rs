//! `*Type` constants.
use std::{borrow::Cow, str::FromStr};

use num_derive::FromPrimitive;
use parse_display::{Display, FromStr};
use serde::{
    de::{Deserializer, Error as _, Unexpected},
    Deserialize, Serialize, Serializer,
};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Translates `STRUCTURE_*` constants.
///
/// *Note:* This constant's `TryFrom<Value>`, `Serialize` and `Deserialize`
/// implementations only operate on made-up integer constants. If you're ever
/// using these impls manually, use the `__structure_type_num_to_str` and
/// `__structure_type_str_to_num` JavaScript functions,
/// [`FromStr`][std::str::FromStr] or [`StructureType::deserialize_from_str`].
///
/// See the [module-level documentation][crate::constants] for more details.
#[derive(
    Copy, Clone, Debug, Display, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr, FromStr,
)]
#[repr(u8)]
#[display(style = "camelCase")]
pub enum StructureType {
    Spawn = 0,
    Extension = 1,
    Road = 2,
    Wall = 3,
    Rampart = 4,
    KeeperLair = 5,
    Portal = 6,
    Controller = 7,
    Link = 8,
    Storage = 9,
    Tower = 10,
    Observer = 11,
    PowerBank = 12,
    PowerSpawn = 13,
    Extractor = 14,
    Lab = 15,
    Terminal = 16,
    Container = 17,
    Nuker = 18,
    Factory = 19,
    InvaderCore = 20,
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
            KeeperLair | PowerBank | Portal | Controller | InvaderCore => return None,
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
            KeeperLair | PowerBank | Portal | Controller | InvaderCore => 0,
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
            KeeperLair | Portal | Controller => return None,
        };
        Some(hits)
    }

    /// Helper function for deserializing from a string rather than a fake
    /// integer value.
    pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s: Cow<'de, str> = Cow::deserialize(d)?;
        Self::from_str(&s).map_err(|_| {
            D::Error::invalid_value(Unexpected::Str(&s), &"a known STRUCTURE_* constant string")
        })
    }
}

js_deserializable!(StructureType);

/// Translates `SUBSCRIPTION_TOKEN` and `INTERSHARD_RESOURCES` constants.
///
/// *Note:* This constant's `TryFrom<Value>`, `Serialize` and `Deserialize`
/// implementations only operate on made-up integer constants. If you're ever
/// using these impls manually, use the `__resource_type_num_to_str`
/// and `__resource_type_str_to_num` JavaScript functions,
/// [`FromStr`][std::str::FromStr] or
/// [`IntershardResourceType::deserialize_from_str`].
///
/// See the [module-level documentation][crate::constants] for more details.
#[derive(
    Copy, Clone, Debug, Display, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr, FromStr,
)]
#[repr(u16)]
pub enum IntershardResourceType {
    /// `"token"`
    #[display("token")]
    SubscriptionToken = 1001,
    /// `"cpuUnlock"`
    #[display("cpuUnlock")]
    CPUUnlock = 1002,
    /// `"pixel"`
    #[display("pixel")]
    Pixel = 1003,
    /// `"accessKey"`
    #[display("accessKey")]
    AccessKey = 1004,
}

impl IntershardResourceType {
    /// Helper function for deserializing from a string rather than a fake
    /// integer value.
    pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s: Cow<'de, str> = Cow::deserialize(d)?;
        Self::from_str(&s).map_err(|_| {
            D::Error::invalid_value(
                Unexpected::Str(&s),
                &"a known constant string in INTERSHARD_RESOURCES",
            )
        })
    }
}

js_deserializable!(IntershardResourceType);

/// Resource type constant for all possible types of resources.
///
/// *Note:* This constant's `TryFrom<Value>`, `Serialize` and `Deserialize`
/// implementations only operate on made-up integer constants. If you're ever
/// using these impls manually, use the `__resource_type_num_to_str`
/// and `__resource_type_str_to_num` JavaScript functions,
/// [`FromStr`][std::str::FromStr] or [`ResourceType::deserialize_from_str`].
///
/// See the [module-level documentation][crate::constants] for more details.
#[derive(
    Copy, Clone, Debug, Display, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr, FromStr,
)]
#[repr(u16)]
pub enum ResourceType {
    /// `"energy"`
    #[display("energy")]
    Energy = 1,
    /// `"power"`
    #[display("power")]
    Power = 2,
    /// `"H"`
    #[display("H")]
    Hydrogen = 3,
    /// `"O"`
    #[display("O")]
    Oxygen = 4,
    /// `"U"`
    #[display("U")]
    Utrium = 5,
    /// `"L"`
    #[display("L")]
    Lemergium = 6,
    /// `"K"`
    #[display("K")]
    Keanium = 7,
    /// `"Z"`
    #[display("Z")]
    Zynthium = 8,
    /// `"X"`
    #[display("X")]
    Catalyst = 9,
    /// `"G"`
    #[display("G")]
    Ghodium = 10,
    // constants.js has these base commodities ordered here, but they're assigned
    // higher integer representations and implemented below to avoid renumbering:
    // RESOURCE_SILICON, RESOURCE_METAL, RESOURCE_BIOMASS, RESOURCE_MIST
    /// `"OH"`
    #[display("OH")]
    Hydroxide = 11,
    /// `"ZK"`
    #[display("ZK")]
    ZynthiumKeanite = 12,
    /// `"UL"`
    #[display("UL")]
    UtriumLemergite = 13,
    /// `"UH"`
    #[display("UH")]
    UtriumHydride = 14,
    /// `"UO"`
    #[display("UO")]
    UtriumOxide = 15,
    /// `"KH"`
    #[display("KH")]
    KeaniumHydride = 16,
    /// `"KO"`
    #[display("KO")]
    KeaniumOxide = 17,
    /// `"LH"`
    #[display("LH")]
    LemergiumHydride = 18,
    /// `"LO"`
    #[display("LO")]
    LemergiumOxide = 19,
    /// `"ZH"`
    #[display("ZH")]
    ZynthiumHydride = 20,
    /// `"ZO"`
    #[display("ZO")]
    ZynthiumOxide = 21,
    /// `"GH"`
    #[display("GH")]
    GhodiumHydride = 22,
    /// `"GO"`
    #[display("GO")]
    GhodiumOxide = 23,
    /// `"UH2O"`
    #[display("UH2O")]
    UtriumAcid = 24,
    /// `"UHO2"`
    #[display("UHO2")]
    UtriumAlkalide = 25,
    /// `"KH2O"`
    #[display("KH2O")]
    KeaniumAcid = 26,
    /// `"KHO2"`
    #[display("KHO2")]
    KeaniumAlkalide = 27,
    /// `"LH2O"`
    #[display("LH2O")]
    LemergiumAcid = 28,
    /// `"LHO2"`
    #[display("LHO2")]
    LemergiumAlkalide = 29,
    /// `"ZH2O"`
    #[display("ZH2O")]
    ZynthiumAcid = 30,
    /// `"ZHO2"`
    #[display("ZHO2")]
    ZynthiumAlkalide = 31,
    /// `"GH2O"`
    #[display("GH2O")]
    GhodiumAcid = 32,
    /// `"GHO2"`
    #[display("GHO2")]
    GhodiumAlkalide = 33,
    /// `"XUH2O"`
    #[display("XUH2O")]
    CatalyzedUtriumAcid = 34,
    /// `"XUHO2"`
    #[display("XUHO2")]
    CatalyzedUtriumAlkalide = 35,
    /// `"XKH2O"`
    #[display("XKH2O")]
    CatalyzedKeaniumAcid = 36,
    /// `"XKHO2"`
    #[display("XKHO2")]
    CatalyzedKeaniumAlkalide = 37,
    /// `"XLH2O"`
    #[display("XLH2O")]
    CatalyzedLemergiumAcid = 38,
    /// `"XLHO2"`
    #[display("XLHO2")]
    CatalyzedLemergiumAlkalide = 39,
    /// `"XZH2O"`
    #[display("XZH2O")]
    CatalyzedZynthiumAcid = 40,
    /// `"XZHO2"`
    #[display("XZHO2")]
    CatalyzedZynthiumAlkalide = 41,
    /// `"XGH2O"`
    #[display("XGH2O")]
    CatalyzedGhodiumAcid = 42,
    /// `"XGHO2"`
    #[display("XGHO2")]
    CatalyzedGhodiumAlkalide = 43,
    /// `"ops"`
    #[display("ops")]
    Ops = 44,
    // these 4 base commodities are ordered earlier in constants.js
    /// `"silicon"`
    #[display("silicon")]
    Silicon = 45,
    /// `"metal"`
    #[display("metal")]
    Metal = 46,
    /// `"biomass"`
    #[display("biomass")]
    Biomass = 47,
    /// `"mist"`
    #[display("mist")]
    Mist = 48,
    /// `"utrium_bar"`
    #[display("utrium_bar")]
    UtriumBar = 49,
    /// `"lemergium_bar"`
    #[display("lemergium_bar")]
    LemergiumBar = 50,
    /// `"zynthium_bar"`
    #[display("zynthium_bar")]
    ZynthiumBar = 51,
    /// `"keanium_bar"`
    #[display("keanium_bar")]
    KeaniumBar = 52,
    /// `"ghodium_melt"`
    #[display("ghodium_melt")]
    GhodiumMelt = 53,
    /// `"oxidant"`
    #[display("oxidant")]
    Oxidant = 54,
    /// `"reductant"`
    #[display("reductant")]
    Reductant = 55,
    /// `"purifier"`
    #[display("purifier")]
    Purifier = 56,
    /// `"battery"`
    #[display("battery")]
    Battery = 57,
    /// `"composite"`
    #[display("composite")]
    Composite = 58,
    /// `"crystal"`
    #[display("crystal")]
    Crystal = 59,
    /// `"liquid"`
    #[display("liquid")]
    Liquid = 60,
    /// `"wire"`
    #[display("wire")]
    Wire = 61,
    /// `"switch"`
    #[display("switch")]
    Switch = 62,
    /// `"transistor"`
    #[display("transistor")]
    Transistor = 63,
    /// `"microchip"`
    #[display("microchip")]
    Microchip = 64,
    /// `"circuit"`
    #[display("circuit")]
    Circuit = 65,
    /// `"device"`
    #[display("device")]
    Device = 66,
    /// `"cell"`
    #[display("cell")]
    Cell = 67,
    /// `"phlegm"`
    #[display("phlegm")]
    Phlegm = 68,
    /// `"tissue"`
    #[display("tissue")]
    Tissue = 69,
    /// `"muscle"`
    #[display("muscle")]
    Muscle = 70,
    /// `"organoid"`
    #[display("organoid")]
    Organoid = 71,
    /// `"organism"`
    #[display("organism")]
    Organism = 72,
    /// `"alloy"`
    #[display("alloy")]
    Alloy = 73,
    /// `"tube"`
    #[display("tube")]
    Tube = 74,
    /// `"fixtures"`
    #[display("fixtures")]
    Fixtures = 75,
    /// `"frame"`
    #[display("frame")]
    Frame = 76,
    /// `"hydraulics"`
    #[display("hydraulics")]
    Hydraulics = 77,
    /// `"machine"`
    #[display("machine")]
    Machine = 78,
    /// `"condensate"`
    #[display("condensate")]
    Condensate = 79,
    /// `"concentrate"`
    #[display("concentrate")]
    Concentrate = 80,
    /// `"extract"`
    #[display("extract")]
    Extract = 81,
    /// `"spirit"`
    #[display("spirit")]
    Spirit = 82,
    /// `"emanation"`
    #[display("emanation")]
    Emanation = 83,
    /// `"essence"`
    #[display("essence")]
    Essence = 84,
    /// `"score"`
    #[cfg(feature = "enable-score")]
    #[display("score")]
    Score = 10001,
    /// `"symbol_aleph"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_aleph")]
    SymbolAleph = 10201,
    /// `"symbol_beth"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_beth")]
    SymbolBeth = 10202,
    /// `"symbol_gimmel"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_gimmel")]
    SymbolGimmel = 10203,
    /// `"symbol_daleth"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_daleth")]
    SymbolDaleth = 10204,
    /// `"symbol_he"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_he")]
    SymbolHe = 10205,
    /// `"symbol_waw"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_waw")]
    SymbolWaw = 10206,
    /// `"symbol_zayin"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_zayin")]
    SymbolZayin = 10207,
    /// `"symbol_heth"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_heth")]
    SymbolHeth = 10208,
    /// `"symbol_teth"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_teth")]
    SymbolTeth = 10209,
    /// `"symbol_yodh"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_yodh")]
    SymbolYodh = 10210,
    /// `"symbol_kaph"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_kaph")]
    SymbolKaph = 10211,
    /// `"symbol_lamedh"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_lamedh")]
    SymbolLamedh = 10212,
    /// `"symbol_mem"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_mem")]
    SymbolMem = 10213,
    /// `"symbol_nun"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_nun")]
    SymbolNun = 10214,
    /// `"symbol_samekh"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_samekh")]
    SymbolSamekh = 10215,
    /// `"symbol_ayin"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_ayin")]
    SymbolAyin = 10216,
    /// `"symbol_pe"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_pe")]
    SymbolPe = 10217,
    /// `"symbol_tsade"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_tsade")]
    SymbolTsade = 10218,
    /// `"symbol_qoph"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_qoph")]
    SymbolQoph = 10219,
    /// `"symbol_res"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_res")]
    SymbolRes = 10220,
    /// `"symbol_sim"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_sim")]
    SymbolSim = 10221,
    /// `"symbol_taw"`
    #[cfg(feature = "enable-symbols")]
    #[display("symbol_taw")]
    SymbolTaw = 10222,
}

#[derive(Copy, Clone, Debug)]
pub enum Boost {
    Harvest(f64),
    BuildAndRepair(f64),
    Dismantle(f64),
    UpgradeController(f64),
    Attack(f64),
    RangedAttack(f64),
    Heal(f64),
    Carry(f64),
    Move(f64),
    Tough(f64),
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
            UtriumHydride => Boost::Attack(2.0),
            // UH2O: {
            //     attack: 3
            // },
            UtriumAcid => Boost::Attack(3.0),
            // XUH2O: {
            //     attack: 4
            // }
            CatalyzedUtriumAcid => Boost::Attack(4.0),
            // UO: {
            //     harvest: 3
            // },
            UtriumOxide => Boost::Harvest(3.0),
            // UHO2: {
            //     harvest: 5
            // },
            UtriumAlkalide => Boost::Harvest(5.0),
            // XUHO2: {
            //     harvest: 7
            // },
            CatalyzedUtriumAlkalide => Boost::Harvest(7.0),
            // KH: {
            //     capacity: 2
            // },
            KeaniumHydride => Boost::Carry(2.0),
            // KH2O: {
            //     capacity: 3
            // },
            KeaniumAcid => Boost::Carry(3.0),
            // XKH2O: {
            //     capacity: 4
            // }
            CatalyzedKeaniumAcid => Boost::Carry(4.0),
            // KO: {
            //     rangedAttack: 2,
            //     rangedMassAttack: 2
            // },
            KeaniumOxide => Boost::RangedAttack(2.0),
            // KHO2: {
            //     rangedAttack: 3,
            //     rangedMassAttack: 3
            // },
            KeaniumAlkalide => Boost::RangedAttack(4.0),
            // XKHO2: {
            //     rangedAttack: 4,
            //     rangedMassAttack: 4
            // }
            CatalyzedKeaniumAlkalide => Boost::RangedAttack(4.0),
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
            LemergiumOxide => Boost::Heal(2.0),
            // LHO2: {
            //     heal: 3,
            //     rangedHeal: 3
            // },
            LemergiumAlkalide => Boost::Heal(3.0),
            // XLHO2: {
            //     heal: 4,
            //     rangedHeal: 4
            // }
            CatalyzedLemergiumAlkalide => Boost::Heal(4.0),
            // ZH: {
            //     dismantle: 2
            // },
            ZynthiumHydride => Boost::Dismantle(2.0),
            // ZH2O: {
            //     dismantle: 3
            // },
            ZynthiumAcid => Boost::Dismantle(3.0),
            // XZH2O: {
            //     dismantle: 4
            // },
            CatalyzedZynthiumAcid => Boost::Dismantle(4.0),
            // ZO: {
            //     fatigue: 2
            // },
            ZynthiumOxide => Boost::Move(2.0),
            // ZHO2: {
            //     fatigue: 3
            // },
            ZynthiumAlkalide => Boost::Move(3.0),
            // XZHO2: {
            //     fatigue: 4
            // }
            CatalyzedZynthiumAlkalide => Boost::Move(4.0),
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

    /// Helper function for deserializing from a string rather than a fake
    /// integer value.
    pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s: Cow<'de, str> = Cow::deserialize(d)?;
        Self::from_str(&s).map_err(|_| {
            D::Error::invalid_value(
                Unexpected::Str(&s),
                &"a known constant string in RESOURCES_ALL",
            )
        })
    }
}

js_deserializable!(ResourceType);

/// Translates market resource types which can include both `RESOURCE_*`
/// and `INTERSHARD_RESOURCES` constants.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MarketResourceType {
    Resource(ResourceType),
    IntershardResource(IntershardResourceType),
}

impl MarketResourceType {
    /// Helper function for deserializing from a string rather than a fake
    /// integer value.
    pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s: Cow<'de, str> = Cow::deserialize(d)?;

        ResourceType::from_str(&s)
            .map(|ty| MarketResourceType::Resource(ty))
            .or(IntershardResourceType::from_str(&s)
                .map(|ty| MarketResourceType::IntershardResource(ty)))
            .map_err(|_| {
                D::Error::invalid_value(
                    Unexpected::Str(&s),
                    &"a known constant string in RESOURCES_ALL or INTERSHARD_RESOURCES",
                )
            })
    }
}

impl<'de> Deserialize<'de> for MarketResourceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use IntershardResourceType::*;
        use MarketResourceType::*;
        use ResourceType::*;

        let resource = u16::deserialize(deserializer)?;
        let resource_type = match resource {
            1 => Resource(Energy),
            2 => Resource(Power),
            3 => Resource(Hydrogen),
            4 => Resource(Oxygen),
            5 => Resource(Utrium),
            6 => Resource(Lemergium),
            7 => Resource(Keanium),
            8 => Resource(Zynthium),
            9 => Resource(Catalyst),
            10 => Resource(Ghodium),
            11 => Resource(Hydroxide),
            12 => Resource(ZynthiumKeanite),
            13 => Resource(UtriumLemergite),
            14 => Resource(UtriumHydride),
            15 => Resource(UtriumOxide),
            16 => Resource(KeaniumHydride),
            17 => Resource(KeaniumOxide),
            18 => Resource(LemergiumHydride),
            19 => Resource(LemergiumOxide),
            20 => Resource(ZynthiumHydride),
            21 => Resource(ZynthiumOxide),
            22 => Resource(GhodiumHydride),
            23 => Resource(GhodiumOxide),
            24 => Resource(UtriumAcid),
            25 => Resource(UtriumAlkalide),
            26 => Resource(KeaniumAcid),
            27 => Resource(KeaniumAlkalide),
            28 => Resource(LemergiumAcid),
            29 => Resource(LemergiumAlkalide),
            30 => Resource(ZynthiumAcid),
            31 => Resource(ZynthiumAlkalide),
            32 => Resource(GhodiumAcid),
            33 => Resource(GhodiumAlkalide),
            34 => Resource(CatalyzedUtriumAcid),
            35 => Resource(CatalyzedUtriumAlkalide),
            36 => Resource(CatalyzedKeaniumAcid),
            37 => Resource(CatalyzedKeaniumAlkalide),
            38 => Resource(CatalyzedLemergiumAcid),
            39 => Resource(CatalyzedLemergiumAlkalide),
            40 => Resource(CatalyzedZynthiumAcid),
            41 => Resource(CatalyzedZynthiumAlkalide),
            42 => Resource(CatalyzedGhodiumAcid),
            43 => Resource(CatalyzedGhodiumAlkalide),
            44 => Resource(Ops),
            45 => Resource(Silicon),
            46 => Resource(Metal),
            47 => Resource(Biomass),
            48 => Resource(Mist),
            49 => Resource(UtriumBar),
            50 => Resource(LemergiumBar),
            51 => Resource(ZynthiumBar),
            52 => Resource(KeaniumBar),
            53 => Resource(GhodiumMelt),
            54 => Resource(Oxidant),
            55 => Resource(Reductant),
            56 => Resource(Purifier),
            57 => Resource(Battery),
            58 => Resource(Composite),
            59 => Resource(Crystal),
            60 => Resource(Liquid),
            61 => Resource(Wire),
            62 => Resource(Switch),
            63 => Resource(Transistor),
            64 => Resource(Microchip),
            65 => Resource(Circuit),
            66 => Resource(Device),
            67 => Resource(Cell),
            68 => Resource(Phlegm),
            69 => Resource(Tissue),
            70 => Resource(Muscle),
            71 => Resource(Organoid),
            72 => Resource(Organism),
            73 => Resource(Alloy),
            74 => Resource(Tube),
            75 => Resource(Fixtures),
            76 => Resource(Frame),
            77 => Resource(Hydraulics),
            78 => Resource(Machine),
            79 => Resource(Condensate),
            80 => Resource(Concentrate),
            81 => Resource(Extract),
            82 => Resource(Spirit),
            83 => Resource(Emanation),
            84 => Resource(Essence),
            1001 => IntershardResource(SubscriptionToken),
            1002 => IntershardResource(CPUUnlock),
            1003 => IntershardResource(Pixel),
            1004 => IntershardResource(AccessKey),
            #[cfg(feature = "enable-score")]
            10001 => Resource(Score),
            #[cfg(feature = "enable-symbols")]
            10201 => Resource(SymbolAleph),
            #[cfg(feature = "enable-symbols")]
            10202 => Resource(SymbolBeth),
            #[cfg(feature = "enable-symbols")]
            10203 => Resource(SymbolGimmel),
            #[cfg(feature = "enable-symbols")]
            10204 => Resource(SymbolDaleth),
            #[cfg(feature = "enable-symbols")]
            10205 => Resource(SymbolHe),
            #[cfg(feature = "enable-symbols")]
            10206 => Resource(SymbolWaw),
            #[cfg(feature = "enable-symbols")]
            10207 => Resource(SymbolZayin),
            #[cfg(feature = "enable-symbols")]
            10208 => Resource(SymbolHeth),
            #[cfg(feature = "enable-symbols")]
            10209 => Resource(SymbolTeth),
            #[cfg(feature = "enable-symbols")]
            10210 => Resource(SymbolYodh),
            #[cfg(feature = "enable-symbols")]
            10211 => Resource(SymbolKaph),
            #[cfg(feature = "enable-symbols")]
            10212 => Resource(SymbolLamedh),
            #[cfg(feature = "enable-symbols")]
            10213 => Resource(SymbolMem),
            #[cfg(feature = "enable-symbols")]
            10214 => Resource(SymbolNun),
            #[cfg(feature = "enable-symbols")]
            10215 => Resource(SymbolSamekh),
            #[cfg(feature = "enable-symbols")]
            10216 => Resource(SymbolAyin),
            #[cfg(feature = "enable-symbols")]
            10217 => Resource(SymbolPe),
            #[cfg(feature = "enable-symbols")]
            10218 => Resource(SymbolTsade),
            #[cfg(feature = "enable-symbols")]
            10219 => Resource(SymbolQoph),
            #[cfg(feature = "enable-symbols")]
            10220 => Resource(SymbolRes),
            #[cfg(feature = "enable-symbols")]
            10221 => Resource(SymbolSim),
            #[cfg(feature = "enable-symbols")]
            10222 => Resource(SymbolTaw),
            _ => {
                return Err(D::Error::invalid_value(
                    Unexpected::Unsigned(resource as u64),
                    &"a valid RESOURCES_ALL or INTERSHARD_RESOURCES type integer",
                ))
            }
        };
        Ok(resource_type)
    }
}

impl Serialize for MarketResourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MarketResourceType::Resource(ty) => ty.serialize(serializer),
            MarketResourceType::IntershardResource(ty) => ty.serialize(serializer),
        }
    }
}

/// Translates the `POWER_CLASS` constants, which are classes of power creeps
#[derive(
    Copy, Clone, Debug, Display, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr, FromStr,
)]
#[repr(u8)]
#[display(style = "camelCase")]
pub enum PowerCreepClass {
    /// `"operator"`
    Operator = 1,
}

js_deserializable!(PowerCreepClass);

/// Translates the `PWR_*` constants, which are types of powers used by power
/// creeps
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Hash, FromPrimitive, Serialize_repr, Deserialize_repr,
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

js_deserializable!(PowerType);

/// Translates the `EFFECT_*` constants, which are natural effect types
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Hash, FromPrimitive, Serialize_repr, Deserialize_repr,
)]
#[repr(u16)]
pub enum NaturalEffectType {
    Invulnerability = 1001,
    CollapseTimer = 1002,
}

js_deserializable!(NaturalEffectType);

/// Translates effect types which can include both `PWR_*` and `EFFECT_*`
/// constants.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum EffectType {
    PowerEffect(PowerType),
    NaturalEffect(NaturalEffectType),
}

impl<'de> Deserialize<'de> for EffectType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let effect = u16::deserialize(deserializer)?;
        let effect_type = match effect {
            1 => EffectType::PowerEffect(PowerType::GenerateOps),
            2 => EffectType::PowerEffect(PowerType::OperateSpawn),
            3 => EffectType::PowerEffect(PowerType::OperateTower),
            4 => EffectType::PowerEffect(PowerType::OperateStorage),
            5 => EffectType::PowerEffect(PowerType::OperateLab),
            6 => EffectType::PowerEffect(PowerType::OperateExtension),
            7 => EffectType::PowerEffect(PowerType::OperateObserver),
            8 => EffectType::PowerEffect(PowerType::OperateTerminal),
            9 => EffectType::PowerEffect(PowerType::DisruptSpawn),
            10 => EffectType::PowerEffect(PowerType::DisruptTower),
            12 => EffectType::PowerEffect(PowerType::Shield),
            13 => EffectType::PowerEffect(PowerType::RegenSource),
            14 => EffectType::PowerEffect(PowerType::RegenMineral),
            15 => EffectType::PowerEffect(PowerType::DisruptTerminal),
            16 => EffectType::PowerEffect(PowerType::OperatePower),
            17 => EffectType::PowerEffect(PowerType::Fortify),
            18 => EffectType::PowerEffect(PowerType::OperateController),
            19 => EffectType::PowerEffect(PowerType::OperateFactory),
            1001 => EffectType::NaturalEffect(NaturalEffectType::Invulnerability),
            1002 => EffectType::NaturalEffect(NaturalEffectType::CollapseTimer),
            _ => {
                return Err(D::Error::invalid_value(
                    Unexpected::Unsigned(effect as u64),
                    &"a valid PWR_* or EFFECT_* type integer",
                ))
            }
        };

        Ok(effect_type)
    }
}
