//! Various constants translated as small enums.
// use std::{borrow::Cow, fmt, str::FromStr};

use enum_iterator::IntoEnumIterator;
use num_derive::FromPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;
use wasm_bindgen::prelude::*;
// use parse_display::FromStr;
use serde::{Deserialize, Serialize};

// bindgen can't take an i8, needs custom boundary functions
/// Translates return code constants.
//#[wasm_bindgen]
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum ReturnCode {
    Ok = 0,
    NotOwner = -1,
    NoPath = -2,
    NameExists = -3,
    Busy = -4,
    NotFound = -5,
    NotEnough = -6,
    InvalidTarget = -7,
    Full = -8,
    NotInRange = -9,
    InvalidArgs = -10,
    Tired = -11,
    NoBodypart = -12,
    RclNotEnough = -14,
    GclNotEnough = -15,
}

// impl ReturnCode {
//     /// Turns this return code into a result.
//     ///
//     /// `ReturnCode::Ok` is turned into `Result::Ok`, all other codes are turned
//     /// into `Result::Err(code)`
//     #[inline]
//     pub fn as_result(self) -> Result<(), Self> {
//         match self {
//             ReturnCode::Ok => Ok(()),
//             other => Err(other),
//         }
//     }
// }

// js_deserializable!(ReturnCode);

/// Translates `FIND_*` constants.
#[wasm_bindgen]
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(u16)]
pub enum Find {
    /// Find all exit positions at the top of the room
    ExitTop = 1,
    ExitRight = 3,
    ExitBottom = 5,
    ExitLeft = 7,
    Exit = 10,
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
    // todo these seem to not work when conditionally compiled out - they're not hurting to leave in but need to figure that out
    //#[cfg(feature = "enable-score")]
    ScoreContainers = 10011,
    //#[cfg(feature = "enable-score")]
    ScoreCollectors = 10012,
}

/// Translates direction constants.
#[wasm_bindgen]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    FromPrimitive,
    Serialize_repr,
    Deserialize_repr,
    IntoEnumIterator,
)]
#[repr(u8)]
pub enum Direction {
    Top = 1,
    TopRight = 2,
    Right = 3,
    BottomRight = 4,
    Bottom = 5,
    BottomLeft = 6,
    Left = 7,
    TopLeft = 8,
}

impl ::std::ops::Neg for Direction {
    type Output = Direction;

    /// Negates this direction. Top goes to Bottom, TopRight goes to BottomLeft,
    /// etc.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps::Direction::*;
    ///
    /// assert_eq!(-Top, Bottom);
    /// assert_eq!(-BottomRight, TopLeft);
    /// assert_eq!(-Left, Right);
    /// ```
    #[inline]
    fn neg(self) -> Direction {
        use Direction::*;

        match self {
            Top => Bottom,
            TopRight => BottomLeft,
            Right => Left,
            BottomRight => TopLeft,
            Bottom => Top,
            BottomLeft => TopRight,
            Left => Right,
            TopLeft => BottomRight,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match *self {
            Direction::Top => "↑",
            Direction::TopRight => "↗",
            Direction::Right => "→",
            Direction::BottomRight => "↘",
            Direction::Bottom => "↓",
            Direction::BottomLeft => "↙",
            Direction::Left => "←",
            Direction::TopLeft => "↖",
        };
        f.write_str(ch)
    }
}

/// Type used for when the game returns a direction to an exit.
///
/// Restricted more than `Direction` in that it can't be diagonal. Used as the
/// result of [`Room::find_exit_to`].
///
/// Can be converted to [`Find`] for immediate use of [`Room::find`]
/// and [`Direction`].
///
/// [`Room::find`]: crate::objects::Room::find
/// [`Room::find_exit_to`]: crate::objects::Room::find_exit_to
#[wasm_bindgen]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    FromPrimitive,
    Serialize_repr,
    Deserialize_repr,
    IntoEnumIterator,
)]
#[repr(u8)]
pub enum ExitDirection {
    Top = 1,
    Right = 3,
    Bottom = 5,
    Left = 7,
}

impl From<ExitDirection> for Find {
    #[inline]
    fn from(dir: ExitDirection) -> Self {
        match dir {
            ExitDirection::Top => Find::ExitTop,
            ExitDirection::Right => Find::ExitRight,
            ExitDirection::Bottom => Find::ExitBottom,
            ExitDirection::Left => Find::ExitLeft,
        }
    }
}

impl From<ExitDirection> for Direction {
    #[inline]
    fn from(dir: ExitDirection) -> Self {
        match dir {
            ExitDirection::Top => Direction::Top,
            ExitDirection::Right => Direction::Right,
            ExitDirection::Bottom => Direction::Bottom,
            ExitDirection::Left => Direction::Left,
        }
    }
}

/// Translates `LOOK_*` constants.
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, IntoEnumIterator)]
pub enum Look {
    Creeps = "creep",
    Energy = "energy",
    Resources = "resource",
    Sources = "source",
    Minerals = "mineral",
    Structures = "structure",
    Flags = "flag",
    ConstructionSites = "constructionSite",
    Nukes = "nuke",
    Terrain = "terrain",
    Tombstones = "tombstone",
    PowerCreeps = "powerCreep",
    Deposits = "deposit",
    Ruins = "ruin",
}

/// Translates `COLOR_*` constants.
#[wasm_bindgen]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    FromPrimitive,
    Hash,
    Deserialize_repr,
    Serialize_repr,
    IntoEnumIterator,
)]
#[repr(u8)]
pub enum Color {
    Red = 1,
    Purple = 2,
    Blue = 3,
    Cyan = 4,
    Green = 5,
    Yellow = 6,
    Orange = 7,
    Brown = 8,
    Grey = 9,
    White = 10,
}

/// Translates `TERRAIN_*` constants.
#[wasm_bindgen]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    FromPrimitive,
    Serialize_repr,
    Deserialize_repr,
    IntoEnumIterator,
)]
#[repr(u8)]
pub enum Terrain {
    // There's no constant for plains, but the absense of a terrain value indicates a plain
    Plain = 0,
    // TERRAIN_MASK_WALL
    Wall = 1,
    // TERRAIN_MASK_SWAMP
    Swamp = 2,
    /* TERRAIN_MASK_LAVA, unimplemented in game
     * Lava = 4, */
}

/// Translates body part constants.
#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Serialize)]
pub enum Part {
    Move = "move",
    Work = "work",
    Carry = "carry",
    Attack = "attack",
    RangedAttack = "ranged_attack",
    Tough = "tough",
    Heal = "heal",
    Claim = "claim",
}

impl Part {
    /// Translates the `BODYPART_COST` constant.
    #[inline]
    pub fn cost(self) -> u32 {
        match self {
            Part::Move => 50,
            Part::Work => 100,
            Part::Carry => 50,
            Part::Attack => 80,
            Part::RangedAttack => 150,
            Part::Tough => 10,
            Part::Heal => 250,
            Part::Claim => 600,
            // I guess bindgen is adding a `#[non_exhaustive]` onto the enum and forcing us to do
            // this:
            _ => 0,
        }
    }

    // /// Helper function for deserializing from a string rather than a fake
    // /// integer value.
    // pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self,
    // D::Error> {     let s: Cow<'de, str> = Cow::deserialize(d)?;
    //     Self::from_str(&s).map_err(|_| {
    //         D::Error::invalid_value(
    //             Unexpected::Str(&s),
    //             &"a known constant string in BODYPARTS_ALL",
    //         )
    //     })
    // }
}

/// Translates the `DENSITY_*` constants.
#[wasm_bindgen]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    FromPrimitive,
    Hash,
    Serialize_repr,
    Deserialize_repr,
    IntoEnumIterator,
)]
#[repr(u8)]
pub enum Density {
    Low = 1,
    Moderate = 2,
    High = 3,
    Ultra = 4,
}

impl Density {
    /// Translates the `MINERAL_DENSITY` constant, the amount of mineral
    /// generated for each density level
    #[inline]
    pub fn amount(self) -> u32 {
        match self {
            Density::Low => 15_000,
            Density::Moderate => 35_000,
            Density::High => 70_000,
            Density::Ultra => 100_000,
        }
    }

    /// Translates the `MINERAL_DENSITY_PROBABILITY` constant.
    ///
    /// These are values intended for subsequent percentage checks
    /// in the order `Low` -> `Medium` -> `High` -> `Ultra`. Use the
    /// [`Density::iter_values`] iterator to iterate in this order.
    ///
    /// If low or ultra on previous regeneration, or random number rolled at
    /// probability [`MINERAL_DENSITY_CHANGE`], the mineral will determine a
    /// random new value ([source]):
    ///
    ///  - Low: 10% chance
    ///  - Moderate: 40% chance
    ///  - High: 40% chance
    ///  - Ultra: 10% chance
    ///
    /// [source]: https://github.com/screeps/engine/blob/c0cfac8f746f26c660501686f16a1fcdb0396d8d/src/processor/intents/minerals/tick.js#L19
    /// [`MINERAL_DENSITY_CHANGE`]:
    /// crate::constants::numbers::MINERAL_DENSITY_CHANGE
    #[inline]
    pub fn probability(self) -> f32 {
        match self {
            Density::Low => 0.1,
            Density::Moderate => 0.5,
            Density::High => 0.9,
            Density::Ultra => 1.0,
        }
    }

    pub fn iter_values() -> impl Iterator<Item = Density> {
        <Density as enum_iterator::IntoEnumIterator>::into_enum_iter()
    }
}

/// Translates `ORDER_*` constants.
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, IntoEnumIterator)]
pub enum OrderType {
    Sell = "sell",
    Buy = "buy",
}
