//! See [http://docs.screeps.com/api/#Game.gcl]
//!
//! [http://docs.screeps.com/api/#Game.gcl]: http://docs.screeps.com/api/#Game.gcl

use crate::constants::{GCL_MULTIPLY, GCL_POW};

/// See [http://docs.screeps.com/api/#Game.gcl]
///
/// [http://docs.screeps.com/api/#Game.gcl]: http://docs.screeps.com/api/#Game.gcl
pub fn level() -> u32 {
    js_unwrap!(Game.gcl.level)
}

/// See [http://docs.screeps.com/api/#Game.gcl]
///
/// [http://docs.screeps.com/api/#Game.gcl]: http://docs.screeps.com/api/#Game.gcl
pub fn progress() -> f64 {
    js_unwrap!(Game.gcl.progress)
}

/// See [http://docs.screeps.com/api/#Game.gcl]
///
/// [http://docs.screeps.com/api/#Game.gcl]: http://docs.screeps.com/api/#Game.gcl
pub fn progress_total() -> f64 {
    js_unwrap!(Game.gcl.progressTotal)
}

/// Provides the total number of control points needed to achieve each level of
/// GCL
///
/// Calculates the total number of control points needed to achieve a given
/// Global Control Level. The resulting value for your current level, added to
/// your [`gcl::progress`][crate::game::gcl::progress], would calculate your
/// total lifetime control points.
pub fn total_for_level(level: u32) -> f64 {
    // formula from
    // https://github.com/screeps/engine/blob/6d498f2f0db4e0744fa6bf8563836d36b49b6a29/src/game/game.js#L117
    ((level - 1) as f64).powf(GCL_POW as f64) * GCL_MULTIPLY as f64
}
