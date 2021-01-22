//! Interface for Screeps [`InterShardMemory`], allowing communication between
//! instances of your code running on different shards.
//!
//! [Screeps documentation](https://docs.screeps.com/api/#InterShardMemory)

use js_sys::JsString;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type InterShardMemory;

    /// Get the current local [`JsString`] intershard memory for this shard.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#InterShardMemory.getLocal)
    #[wasm_bindgen(static_method_of = InterShardMemory, js_name = getLocal)]
    pub fn get_local() -> Option<JsString>;

    /// Overwrite the current shard's intershard memory segment with new
    /// contents.  Maximum allowed length of [`INTER_SHARD_MEMORY_SIZE_LIMIT`]
    /// bytes.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#InterShardMemory.setLocal)
    ///
    /// [`INTER_SHARD_MEMORY_SIZE_LIMIT`]:
    /// crate::constants::INTER_SHARD_MEMORY_SIZE_LIMIT
    #[wasm_bindgen(static_method_of = InterShardMemory, js_name = setLocal)]
    pub fn set_local(val: &JsString);

    /// Get the data that another shard's code instance has written to its
    /// intershard memory segment.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#InterShardMemory.getRemote)
    #[wasm_bindgen(static_method_of = InterShardMemory, js_name = getRemote)]
    pub fn get_remote(val: &JsString) -> Option<JsString>;
}

// //! An interface for communicating between shards.
// //!
// //! Quoting from [`InterShardMemory`] documentation:
// //!
// //! > InterShardMemory object provides an interface for communicating between
// //! > shards. Your script is executed separately on each shard, and their
// //! > Memory objects are isolated from each other. In order to pass messages and
// //! > data between shards, you need to use InterShardMemory instead.
// //! >
// //! > Every shard can have its own data string that can be accessed by all other
// //! > shards. A shard can write only to its own data, other shards' data is
// //! > read-only.
// //! >
// //! > This data has nothing to do with Memory contents, it's a separate data
// //! > container.
// //!
// //! [`InterShardMemory`]: https://docs.screeps.com/api/#InterShardMemory

// /// Returns the string contents of the current shard's data, `None` if it hasn't
// /// been set or on a private server without the intershard memory interface
// pub fn get_local() -> Option<String> {
//     js_unwrap!(typeof(InterShardMemory) == "object" && InterShardMemory.getLocal() || null)
// }

// /// Replace the current shard's data with the new value. Maximum allowed length
// /// of 102400 bytes.
// pub fn set_local(value: &str) {
//     js! {
//         typeof(InterShardMemory) == "object" && InterShardMemory.setLocal(@{value});
//     }
// }

// /// Returns the string contents of another shard's data.
// ///
// /// Consider using [`game::cpu::shard_limits`] to retrieve shard names - invalid
// /// shard names will cause an error in the game API
// ///
// /// [`game::cpu::shard_limits`]: crate::game::cpu::shard_limits
// pub fn get_remote(shard: &str) -> Option<String> {
//     js_unwrap!(typeof(InterShardMemory) == "object" && InterShardMemory.getRemote(@{shard}) || null)
// }
