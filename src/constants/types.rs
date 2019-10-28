//! `*Type` constants.
use std::{borrow::Cow, str::FromStr};

use num_derive::FromPrimitive;
use parse_display::FromStr;
use serde::{
    de::{Deserializer, Error as _, Unexpected},
    Deserialize,
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
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr, FromStr)]
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
            Terminal => TOWER_HITS,
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
/// using these impls manually, use the `__intershard_resource_type_num_to_str`
/// and `__intershard_resource_type_str_to_num` JavaScript functions,
/// [`FromStr`][std::str::FromStr] or
/// [`IntershardResourceType::deserialize_from_str`].
///
/// See the [module-level documentation][crate::constants] for more details.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr, FromStr)]
#[repr(u8)]
pub enum IntershardResourceType {
    #[display("token")]
    SubscriptionToken = 1,
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
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr, FromStr)]
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
}

impl ResourceType {
    /// Translates the `REACTION_TIME` constant.
    #[inline]
    pub fn reaction_time(self) -> Option<u32> {
        use ResourceType::*;
        let time = match self {
            // these comments copied directly from JavaScript 'constants.js' file.
            // OH: 20,
            Hydroxide => 20,
            // ZK: 5,
            ZynthiumKeanite => 5,
            // UL: 5,
            UtriumLemergite => 5,
            // G: 5,
            Ghodium => 5,
            // UH: 10,
            UtriumHydride => 10,
            // UH2O: 5,
            UtriumAcid => 5,
            // XUH2O: 60,
            CatalyzedUtriumAcid => 60,
            // UO: 10,
            UtriumOxide => 10,
            // UHO2: 5,
            UtriumAlkalide => 5,
            // XUHO2: 60,
            CatalyzedUtriumAlkalide => 60,
            // KH: 10,
            KeaniumHydride => 10,
            // KH2O: 5,
            KeaniumAcid => 5,
            // XKH2O: 60,
            CatalyzedKeaniumAcid => 60,
            // KO: 10,
            KeaniumOxide => 10,
            // KHO2: 5,
            KeaniumAlkalide => 5,
            // XKHO2: 60,
            CatalyzedKeaniumAlkalide => 60,
            // LH: 15,
            LemergiumHydride => 15,
            // LH2O: 10,
            LemergiumAcid => 10,
            // XLH2O: 65,
            CatalyzedLemergiumAcid => 65,
            // LO: 10,
            LemergiumOxide => 10,
            // LHO2: 5,
            LemergiumAlkalide => 5,
            // XLHO2: 60,
            CatalyzedLemergiumAlkalide => 60,
            // ZH: 20,
            ZynthiumHydride => 20,
            // ZH2O: 40,
            ZynthiumAcid => 40,
            // XZH2O: 160,
            CatalyzedZynthiumAcid => 160,
            // ZO: 10,
            ZynthiumOxide => 10,
            // ZHO2: 5,
            ZynthiumAlkalide => 5,
            // XZHO2: 60,
            CatalyzedZynthiumAlkalide => 60,
            // GH: 10,
            GhodiumHydride => 10,
            // GH2O: 15,
            GhodiumAcid => 15,
            // XGH2O: 80,
            CatalyzedGhodiumAcid => 80,
            // GO: 10,
            GhodiumOxide => 10,
            // GHO2: 30,
            GhodiumAlkalide => 30,
            // XGHO2: 150,
            CatalyzedGhodiumAlkalide => 150,
            // non-molecule resources
            _ => return None,
        };
        Some(time)
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

/// Translates the `POWER_CLASS` constants, which are classes of power creeps
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr, FromStr)]
#[repr(u8)]
pub enum PowerCreepClass {
    /// `"operator"`
    #[display("operator")]
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
