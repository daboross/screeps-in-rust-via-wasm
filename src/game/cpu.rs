//! See [http://docs.screeps.com/api/#Game.cpu]
//!
//! [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
use std::collections;

use serde::{Deserialize, Serialize};

use crate::{constants::ReturnCode, traits::TryInto};

/// See [`v8_getheapstatistics`]
///
/// [`v8_getheapstatistics`]: https://nodejs.org/dist/latest-v8.x/docs/api/v8.html#v8_v8_getheapstatistics
#[derive(Default, Serialize, Deserialize)]
pub struct HeapStatistics {
    pub total_heap_size: u32,
    pub total_heap_size_executable: u32,
    pub total_physical_size: u32,
    pub total_available_size: i32,
    pub used_heap_size: u32,
    pub heap_size_limit: u32,
    pub malloced_memory: u32,
    pub peak_malloced_memory: u32,
    pub does_zap_garbage: u32,
    pub externally_allocated_size: u32,
}

js_serializable!(HeapStatistics);
js_deserializable!(HeapStatistics);

/// See [http://docs.screeps.com/api/#Game.cpu]
///
/// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
pub fn limit() -> u32 {
    js_unwrap!(Game.cpu.limit)
}

/// See [http://docs.screeps.com/api/#Game.cpu]
///
/// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
pub fn tick_limit() -> u32 {
    js_unwrap!(Game.cpu.tickLimit)
}

/// See [http://docs.screeps.com/api/#Game.cpu]
///
/// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
pub fn bucket() -> u32 {
    js_unwrap!(Game.cpu.bucket)
}

/// See [http://docs.screeps.com/api/#Game.cpu]
///
/// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
pub fn shard_limits() -> collections::HashMap<String, u32> {
    js_unwrap!(Game.cpu.shardLimits)
}

/// Whether you have an active subscription and are able to use your full CPU
/// limit. See [http://docs.screeps.com/api/#Game.cpu]
///
/// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
pub fn unlocked() -> bool {
    // undefined on private servers; return true in that case
    js_unwrap!(Game.cpu.unlocked || Game.cpu.unlocked === undefined)
}

/// Time of expiration of your current CPU subscription in milliseconds since
/// epoch, or None when locked, or unlocked via subscription. See [http://docs.screeps.com/api/#Game.cpu]
///
/// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
pub fn unlocked_time() -> Option<u64> {
    js_unwrap!(Game.cpu.unlockedTime)
}

/// See [https://docs.screeps.com/api/#Game.cpu.getHeapStatistics]
///
/// [https://docs.screeps.com/api/#Game.cpu.getHeapStatistics]: https://docs.screeps.com/api/#Game.cpu.getHeapStatistics
///
/// Returns object with all 0 values if heap statistics are not available.
pub fn get_heap_statistics() -> HeapStatistics {
    use stdweb::Value;

    let heap_stats: Value = js_unwrap!(Game.cpu.getHeapStatistics && Game.cpu.getHeapStatistics());

    match heap_stats {
        Value::Null | Value::Undefined | Value::Bool(false) => HeapStatistics::default(),
        other => other.try_into().expect(
            "expected Game.cpu.getHeapStatistics() to return an object with a known format",
        ),
    }
}

/// See [https://docs.screeps.com/api/#Game.cpu.getUsed]
///
/// [https://docs.screeps.com/api/#Game.cpu.getUsed]: https://docs.screeps.com/api/#Game.cpu.getUsed
pub fn get_used() -> f64 {
    js_unwrap!(Game.cpu.getUsed())
}

/// Reset your runtime environment and wipe all data in heap memory.
///
/// See [`Game.cpu.halt`](https://docs.screeps.com/api/#Game.cpu.halt).
pub fn halt() {
    js! {
        Game.cpu.halt();
    }
}

/// See [https://docs.screeps.com/api/#Game.cpu.setShardLimits]
///
/// [https://docs.screeps.com/api/#Game.cpu.setShardLimits]: https://docs.screeps.com/api/#Game.cpu.setShardLimits
pub fn set_shard_limits(limits: collections::HashMap<String, u32>) -> ReturnCode {
    js_unwrap!(Game.cpu.setShardLimits(@{limits}))
}

/// Spend a [`CPUUnlock`] from your intershard resource inventory to unlock your
/// full CPU limit for 24 hours
///
/// See [`Game.cpu.unlock`](https://docs.screeps.com/api/#Game.cpu.unlock).
///
/// [`CPUUnlock`]: crate::constants::types::IntershardResourceType::CPUUnlock
pub fn unlock() -> ReturnCode {
    // undefined on private servers, return OK in that case
    js_unwrap!(typeof(Game.cpu.unlock) == "function" && Game.cpu.unlock() || 0)
}

/// Generate a [`Pixel`], spending [`PIXEL_CPU_COST`] from [`game::cpu::bucket`]
///
/// See [`Game.cpu.generatePixel`](https://docs.screeps.com/api/#Game.cpu.generatePixel).
///
/// [`Pixel`]: crate::constants::IntershardResourceType::Pixel
/// [`PIXEL_CPU_COST`]: crate::constants::PIXEL_CPU_COST
/// [`game::cpu::bucket`]: crate::game::cpu::bucket
#[cfg(not(feature = "disable-generate-pixel"))]
pub fn generate_pixel() -> ReturnCode {
    // undefined on private servers, return OK in that case
    js_unwrap!(typeof(Game.cpu.generatePixel) == "function" && Game.cpu.generatePixel() || 0)
}
