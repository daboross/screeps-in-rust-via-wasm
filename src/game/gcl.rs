//! Global Control Level information.
//!
//! [Screeps documentation](https://docs.screeps.com/api/#Game.gcl)
use crate::constants::{GCL_MULTIPLY, GCL_POW};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Object with info about your Global Control Level from [`Game::gcl`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.gcl)
    ///
    /// [`Game::gcl`]: crate::game::Game::gcl
    #[wasm_bindgen]
    pub type GclInfo;

    /// Your current Global Control Level, which determines the number of rooms
    /// you are allowed to claim.
    #[wasm_bindgen(method, getter)]
    pub fn level(this: &GclInfo) -> u32;

    /// Your progress toward the next Global Control Level.
    #[wasm_bindgen(method, getter)]
    pub fn progress(this: &GclInfo) -> f64;

    /// Total progress needed to reach the next Global Control Level.
    #[wasm_bindgen(method, getter = progressTotal)]
    pub fn progress_total(this: &GclInfo) -> f64;
}

/// Provides the total number of control points needed to achieve each level of
/// GCL.
///
/// Calculates the total number of control points needed to achieve a given
/// Global Control Level. The resulting value for your current level, added to
/// your [`GclInfo::progress`], would calculate your total lifetime control
/// points.
pub fn total_for_level(level: u32) -> f64 {
    // formula from
    // https://github.com/screeps/engine/blob/6d498f2f0db4e0744fa6bf8563836d36b49b6a29/src/game/game.js#L117
    ((level - 1) as f64).powf(GCL_POW as f64) * GCL_MULTIPLY as f64
}
