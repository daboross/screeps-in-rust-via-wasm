//! `screeps-game-api`
//!
//! # Cargo Features
//!
//! ## `check-all-casts`
//!
//! By default, `screeps-game-api` assumes that the Screeps JavaScript API calls
//! return the types that they are documented to return and bypasses
//! `instanceof` checks when constructing rust wrappers for those return values.
//!
//! To enable checking all types on all API calls, even ones when the screeps
//! server reliably returns the expected type, depend on `screeps-game-api` with
//! the `"check-all-casts"` feature flag:
//!
//! ```toml
//! [dependencies]
//! # ...
//! screeps-game-api = { version = "0.3", features = ["check-all-casts"] }
//! ```
#![recursion_limit = "128"]

#[macro_use]
pub mod macros;

pub mod constants;
pub mod game;
pub mod inter_shard_memory;
pub mod js_collections;
pub mod memory;
pub mod objects;
pub mod pathfinder;
mod positions;
pub mod raw_memory;
pub mod traits;

pub use stdweb::private::ConversionError;

pub use crate::{
    constants::*,
    js_collections::{JsMap, JsVec},
    objects::*,
    positions::{LocalRoomName, LocalRoomNameParseError, LocalRoomPosition},
    traits::{FromExpectedType, IntoExpectedType},
};

/// Useful for `use screeps::prelude::*;` to bring in screeps traits. Does not
/// contain any structures in order to minimize namespace polution.
pub mod prelude {
    pub use crate::objects::{
        CanDecay, CanStoreEnergy, HasCooldown, HasId, HasPosition, HasStore,
        OwnedStructureProperties, RoomObjectProperties, StructureProperties,
    };
}
