//! Constants for use with [`Room::look_for_at`] and related functions.
//!
//! *Note:* Types in this module have purposefully ambiguous names, and are
//! intended to be used as, for example, `look::CREEPS`, not `CREEPS`.
//!
//! You can do this my importing the module itself, rather than any individual
//! constant, and then just referring to the constants relative to the module.
//!
//! [`Room::look_for_at`]: crate::objects::Room::look_for_at
use std::{borrow::Cow, str::FromStr};

use parse_display::FromStr;
use serde::{
    de::{Deserializer, Error as _, Unexpected},
    Deserialize,
};
use serde_repr::{Deserialize_repr, Serialize_repr};
use stdweb::Value;

use super::Terrain;
use crate::{
    objects::{
        ConstructionSite, Creep, Deposit, Flag, Mineral, Nuke, PowerCreep, Resource, Ruin, Source,
        Structure, Tombstone,
    },
    traits::{IntoExpectedType, TryInto},
};

#[cfg(feature = "enable-score")]
use crate::objects::{ScoreCollector, ScoreContainer};

#[cfg(feature = "enable-symbols")]
use crate::objects::{SymbolContainer, SymbolDecoder};

/// Internal enum representing each LOOK_* constant.
///
/// It's recommended to use the constants in the `look` module instead for type
/// safety.
///
/// *Note:* This constant's `TryFrom<Value>`, `Serialize` and `Deserialize`
/// implementations only operate on made-up integer constants. If you're ever
/// using these impls manually, use the `__look_num_to_str` and
/// `__look_str_to_num` JavaScript functions, [`FromStr`][std::str::FromStr] or
/// [`Look::deserialize_from_str`].
///
/// See the [module-level documentation][crate::constants] for more details.
#[doc(hidden)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr, FromStr)]
#[repr(u16)]
pub enum Look {
    #[display("creep")]
    Creeps = 0,
    #[display("energy")]
    Energy = 1,
    #[display("resource")]
    Resources = 2,
    #[display("source")]
    Sources = 3,
    #[display("mineral")]
    Minerals = 4,
    // LOOK_DEPOSITS is defined here in constants.js but is implemented below to prevent
    // renumbering
    #[display("structure")]
    Structures = 5,
    #[display("flag")]
    Flags = 6,
    #[display("constructionSite")]
    ConstructionSites = 7,
    #[display("nuke")]
    Nukes = 8,
    #[display("terrain")]
    Terrain = 9,
    #[display("tombstone")]
    Tombstones = 10,
    #[display("powerCreep")]
    PowerCreeps = 11,
    #[display("deposit")]
    Deposits = 12,
    #[display("ruin")]
    Ruins = 13,
    #[cfg(feature = "enable-score")]
    #[display("scoreContainer")]
    ScoreContainers = 10011,
    #[cfg(feature = "enable-score")]
    #[display("scoreCollector")]
    ScoreCollectors = 10012,
    #[cfg(feature = "enable-symbols")]
    #[display("symbolContainer")]
    SymbolContainers = 10021,
    #[cfg(feature = "enable-symbols")]
    #[display("symbolDecoder")]
    SymbolDecoders = 10022,
}

js_deserializable!(Look);

impl Look {
    /// Helper function for deserializing from a string rather than a fake
    /// integer value.
    pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s: Cow<'de, str> = Cow::deserialize(d)?;
        Self::from_str(&s).map_err(|_| {
            D::Error::invalid_value(Unexpected::Str(&s), &"a known LOOK_* constant string")
        })
    }
}

pub unsafe trait LookConstant {
    type Item;

    fn convert_and_check_items(reference: Value) -> Vec<Self::Item>;

    fn look_code(&self) -> Look;
}

typesafe_look_constants! {
    pub struct CREEPS = (Look::Creeps, Creep, IntoExpectedType::into_expected_type);
    pub struct ENERGY = (Look::Energy, Resource, IntoExpectedType::into_expected_type);
    pub struct RESOURCES = (Look::Resources, Resource, IntoExpectedType::into_expected_type);
    pub struct SOURCES = (Look::Sources, Source, IntoExpectedType::into_expected_type);
    pub struct MINERALS = (Look::Minerals, Mineral, IntoExpectedType::into_expected_type);
    pub struct DEPOSITS = (Look::Deposits, Deposit, IntoExpectedType::into_expected_type);
    pub struct STRUCTURES = (Look::Structures, Structure, IntoExpectedType::into_expected_type);
    pub struct FLAGS = (Look::Flags, Flag, IntoExpectedType::into_expected_type);
    pub struct CONSTRUCTION_SITES = (Look::ConstructionSites, ConstructionSite,
        IntoExpectedType::into_expected_type);
    pub struct NUKES = (Look::Nukes, Nuke, IntoExpectedType::into_expected_type);
    pub struct TERRAIN = (Look::Terrain, Terrain, TryInto::try_into);
    pub struct TOMBSTONES = (Look::Tombstones, Tombstone, IntoExpectedType::into_expected_type);
    pub struct POWER_CREEPS = (Look::PowerCreeps, PowerCreep, IntoExpectedType::into_expected_type);
    pub struct RUINS = (Look::Ruins, Ruin, IntoExpectedType::into_expected_type);
}

#[cfg(feature = "enable-score")]
typesafe_look_constants! {
    pub struct SCORE_CONTAINERS = (Look::ScoreContainers, ScoreContainer, IntoExpectedType::into_expected_type);
    pub struct SCORE_COLLECTORS = (Look::ScoreCollectors, ScoreCollector, IntoExpectedType::into_expected_type);
}

#[cfg(feature = "enable-symbols")]
typesafe_look_constants! {
    pub struct SYMBOL_CONTAINERS = (Look::SymbolContainers, SymbolContainer, IntoExpectedType::into_expected_type);
    pub struct SYMBOL_DECODERS = (Look::SymbolDecoders, SymbolDecoder, IntoExpectedType::into_expected_type);
}
