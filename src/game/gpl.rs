//! Global Power Level information.
//!
//! [Screeps documentation](http://docs.screeps.com/api/#Game.gpl)
use crate::constants::{POWER_LEVEL_MULTIPLY, POWER_LEVEL_POW};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Object with info about your Global Power Level from [`Game::gpl`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.gpl)
    ///
    /// [`Game::gpl`]: crate::game::Game::gpl
    #[wasm_bindgen]
    pub type GplInfo;

    /// Your current Global Power Level, which determines the number of rooms
    /// you are allowed to claim.
    #[wasm_bindgen(method, getter)]
    pub fn level(this: &GplInfo) -> u32;

    /// Your progress toward the next Global Power Level.
    #[wasm_bindgen(method, getter)]
    pub fn progress(this: &GplInfo) -> f64;

    /// Total progress needed to reach the next Global Power Level.
    #[wasm_bindgen(method, getter = progressTotal)]
    pub fn progress_total(this: &GplInfo) -> f64;
}

/// Provides the total number of processed power needed to achieve each level
/// of GPL.
///
/// Calculates the total number of power that need to be processed to achieve a
/// given Global Power Level. The resulting value for your current level, added
/// to your [`GplInfo::progress`], would calculate your total lifetime power
/// points.
pub fn total_for_level(level: u32) -> u64 {
    // formula from
    // https://github.com/screeps/engine/blob/6d498f2f0db4e0744fa6bf8563836d36b49b6a29/src/game/game.js#L120
    (level as u64).pow(POWER_LEVEL_POW) * POWER_LEVEL_MULTIPLY as u64
}
