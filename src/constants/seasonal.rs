//! Constants that apply only to Screeps seasonal servers.
//!
//! [Screeps seasonal documentation](https://docs-season.screeps.com/api/#Season-constants)

/// Constants for Screeps seasonal, season 1
///
/// These constants are relevant to the mechanics in seasonal, season 1.
/// [This mod](https://github.com/screeps/mod_season1) can be used to enable these mechanics
/// on a private server.
#[cfg(feature = "seasonal-season-1")]
pub mod season_1 {
    // RESOURCE_SCORE defined in `types.rs`
    // FIND_SCORE_CONTAINERS defined in `find.rs`
    /// Radius around each [`ScoreCollector`] that starts surrounded with
    /// [`StructureWall`]s with random hit points.
    ///
    /// [`ScoreCollector`]: crate::objects::ScoreCollector
    /// [`StructureWall`]: crate::objects::StructureWall
    pub const WALLS_RADIUS: u8 = 5;
    /// The percentage chance that a given room will have a [`ScoreContainer`]
    /// spawned in it every [`SCORE_CONTAINER_SPAWN_INTERVAL_TICKS`] ticks.
    ///
    /// [`ScoreContainer`]: crate::objects::ScoreContainer
    pub const SCORE_CONTAINER_SPAWN_CHANCE: f32 = 0.01;
    // SCORE_CONTAINER_SPAWN_INTERVAL not implemented due to being no longer
    // used as of https://github.com/screeps/mod-season1/commit/3b4d0aaabfb4bffab80845ac4ea9611f83935e1c
    /// The number of ticks between chances to spawn a [`ScoreContainer`] in
    /// rooms at random.
    ///
    /// [`ScoreContainer`]: crate::objects::ScoreContainer
    pub const SCORE_CONTAINER_SPAWN_INTERVAL_TICKS: u32 = 250;
    /// Amount of capacity in a [`ScoreCollector`] that regenerates each tick.
    ///
    /// [`ScoreCollector`]: crate::objects::ScoreCollector
    pub const SCORE_COLLECTOR_SINK: u32 = 20;
    /// Maximum capacity bucket for a [`ScoreCollector`].
    ///
    /// [`ScoreCollector`]: crate::objects::ScoreCollector
    pub const SCORE_COLLECTOR_MAX_CAPACITY: u32 = 20_000;
}
