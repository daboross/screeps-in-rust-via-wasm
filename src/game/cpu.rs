//! Information about, and functions to manage, your code's resource utilization
//!
//! [Screeps documentation](http://docs.screeps.com/api/#Game.cpu)
use js_sys::Object;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Object with info about your CPU allocations and limits from
    /// [`Game::cpu`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.cpu)
    ///
    /// [`Game::cpu`]: crate::game::Game::cpu
    #[wasm_bindgen]
    pub type CpuInfo;

    /// Your assigned CPU for the current shard.
    #[wasm_bindgen(method, getter)]
    pub fn limit(this: &CpuInfo) -> u32;

    /// The amount of CPU available for execution this tick, which consists of
    /// your [`CpuInfo::limit`] and [`CpuInfo::bucket`] up to a maximum of 500
    /// ([`CPU_TICK_LIMIT_MAX`]), or [`f64::INFINITY`] on sim.
    ///
    /// [`CPU_TICK_LIMIT_MAX`]: crate::constants::extra::CPU_TICK_LIMIT_MAX
    #[wasm_bindgen(method, getter = tickLimit)]
    pub fn tick_limit(this: &CpuInfo) -> f64;

    /// The amount of CPU that has accumulated in your bucket.
    #[wasm_bindgen(method, getter)]
    pub fn bucket(this: &CpuInfo) -> u32;

    /// Your assigned CPU limits for each shard in an [`Object`], with shard
    /// names in [`JsString`] form as keys and numbers as values. This is the
    /// same format accepted by [`CpuInfo::set_shard_limits`].
    #[wasm_bindgen(method, getter = shardLimits)]
    pub fn shard_limits(this: &CpuInfo) -> Object;

    /// Whether your account is unlocked to have full CPU.
    #[wasm_bindgen(method, getter)]
    pub fn unlocked(this: &CpuInfo) -> bool;

    /// If your account has been unlocked for a limited time, contains the time
    /// it's unlocked until in milliseconds since epoch.
    #[wasm_bindgen(method, getter = unlockedTime)]
    pub fn unlocked_time(this: &CpuInfo) -> Option<u64>;

    /// Get information about your script's memory heap usage.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.cpu.getHeapStatistics)
    #[wasm_bindgen(method, js_name = getHeapStatistics)]
    pub fn get_heap_statistics(this: &CpuInfo) -> HeapStatistics;

    /// Get the amount of CPU time used for execution so far this tick.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.cpu.getUsed)
    #[wasm_bindgen(method, js_name = getUsed)]
    pub fn get_used(this: &CpuInfo) -> f64;

    /// Stop execution of your script, starting with a fresh environment next
    /// tick.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.cpu.halt)
    #[wasm_bindgen(method)]
    pub fn halt(this: &CpuInfo);

    /// Sets new shard limits for your script in an [`Object`], with shard names
    /// in [`JsString`] form as keys and numbers as values. This is the same
    /// format accepted by [`CpuInfo::shard_limits`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.cpu.setShardLimits)
    #[wasm_bindgen(method, js_name = setShardLimits)]
    pub fn set_shard_limits(this: &CpuInfo, limits: &Object) -> i8;

    /// Consume a [`CpuUnlock`] to unlock your full CPU for 24 hours.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.cpu.unlock)
    #[wasm_bindgen(method)]
    pub fn unlock(this: &CpuInfo) -> i8;

    /// Generate a [`Pixel`], consuming [`PIXEL_COST`] CPU from your bucket.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.cpu.unlock)
    ///
    /// [`Pixel`]: crate::constants::IntershardResourceType::Pixel
    /// [`PIXEL_COST`]: crate::constants::PIXEL_COST
    #[cfg(not(feature = "disable-generate-pixel"))]
    #[wasm_bindgen(method, js_name = generatePixel)]
    pub fn generate_pixel(this: &CpuInfo) -> i8;
}

#[wasm_bindgen]
extern "C" {
    /// Object with info about the memory heap of your virtual machine.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.cpu.getHeapStatistics)
    #[wasm_bindgen]
    pub type HeapStatistics;

    /// The total heap consumed.
    #[wasm_bindgen(method, getter)]
    pub fn total_heap_size(this: &HeapStatistics) -> u32;

    /// The total heap consumed by executable code.
    #[wasm_bindgen(method, getter)]
    pub fn total_heap_size_executable(this: &HeapStatistics) -> u32;

    /// The total amount of heap committed to memory.
    #[wasm_bindgen(method, getter)]
    pub fn total_physical_size(this: &HeapStatistics) -> u32;

    /// Amount of heap available for allocation.
    #[wasm_bindgen(method, getter)]
    pub fn total_available_size(this: &HeapStatistics) -> u32;

    /// Total heap consumed by application data.
    #[wasm_bindgen(method, getter)]
    pub fn used_heap_size(this: &HeapStatistics) -> u32;

    /// The allowed limit for total heap memory.
    #[wasm_bindgen(method, getter)]
    pub fn heap_size_limit(this: &HeapStatistics) -> u32;

    /// Total amount of memory obtained by malloc.
    #[wasm_bindgen(method, getter)]
    pub fn malloced_memory(this: &HeapStatistics) -> u32;

    /// Maximum amount of memory obtained by malloc.
    #[wasm_bindgen(method, getter)]
    pub fn peak_malloced_memory(this: &HeapStatistics) -> u32;

    /// Whether the virtual machine overwrites memory as it deallocates -
    /// usually 0.
    #[wasm_bindgen(method, getter)]
    pub fn does_zap_garbage(this: &HeapStatistics) -> u32;

    /// External allocations that are outside of the v8 heap but still count
    /// against the memory limit.
    #[wasm_bindgen(method, getter)]
    pub fn externally_allocated_size(this: &HeapStatistics) -> u32;
}
