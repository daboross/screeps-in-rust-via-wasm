//! Interface for Screeps [`RawMemory`] global object.
//!
//! [`RawMemory`]: https://docs.screeps.com/api/#RawMemory

use js_sys::{Array, JsString, Object};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type RawMemory;

    // todo docs; Reflect::get_u32
    /// Get an [`Object`] with all of the segments requested on the previous
    /// tick, with segment numbers as keys and segment data in [`JsString`] form
    /// as values.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RawMemory.segments)
    #[wasm_bindgen(static_method_of = RawMemory, getter)]
    pub fn segments() -> Object;

    // todo ForeignSegment struct
    /// Get the foreign memory segment belonging to another player requested
    /// last tick.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RawMemory.foreignSegment)
    #[wasm_bindgen(static_method_of = RawMemory, getter = ForeignSegment)]
    pub fn foreign_segment() -> Option<Object>;

    /// Get the stored serialized memory as a [`JsString`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RawMemory.get)
    #[wasm_bindgen(static_method_of = RawMemory)]
    pub fn get() -> JsString;

    /// Overwrite the stored memory with a new [`JsString`].  Maximum size
    /// [`MEMORY_SIZE_LIMIT`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RawMemory.set)
    ///
    /// [`MEMORY_SIZE_LIMIT`]: crate::constants::MEMORY_SIZE_LIMIT
    #[wasm_bindgen(static_method_of = RawMemory)]
    pub fn set(val: &JsString);

    /// Sets available memory segments for the next tick, as an array of numbers
    /// from 0 to 99.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RawMemory.setActiveSegments)
    #[wasm_bindgen(static_method_of = RawMemory, js_name = setActiveSegments)]
    pub fn set_active_segments(segment_ids: &Array);

    /// Sets available foreign memory segment for the next tick to a memory
    /// segment marked as public by another user. If no id is passed, the user's
    /// default public segment is retrieved.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RawMemory.setActiveForeignSegment)
    #[wasm_bindgen(static_method_of = RawMemory, js_name = setActiveForeignSegment)]
    pub fn set_active_foreign_segment(username: &JsString, segment_id: Option<u8>);

    /// Sets your default foreign memory segment for other players to read, or
    /// remove your public segment with `None`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RawMemory.setDefaultPublicSegment)
    #[wasm_bindgen(static_method_of = RawMemory, js_name = setDefaultPublicSegment)]
    pub fn set_default_public_segment(segment_id: Option<u8>);

    /// Sets available memory segments for the next tick, as an array of numbers
    /// from 0 to 99.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RawMemory.setPublicSegments)
    #[wasm_bindgen(static_method_of = RawMemory, js_name = setPublicSegments)]
    pub fn set_public_segments(segment_ids: &Array);
}

// use serde::Deserialize;

// #[derive(Deserialize, Debug)]
// pub struct ForeignSegment {
//     username: String,
//     id: String,
//     data: String,
// }

// js_deserializable!(ForeignSegment);

// pub fn get_active_segments() -> Vec<u32> {
//     js_unwrap!(Object.keys(RawMemory.segments).map(Number))
// }

// /// Sets active segments (max 10 ids).
// pub fn set_active_segments(ids: &[u32]) {
//     assert!(
//         ids.len() <= 10,
//         "can't set more than 10 active segments at a time"
//     );
//     js! { @(no_return)
//         RawMemory.setActiveSegments(@{ids});
//     }
// }

// pub fn get_segment(id: u32) -> Option<String> {
//     js_unwrap!(RawMemory.segments[@{id}])
// }

// pub fn set_segment(id: u32, data: &str) {
//     js! { @(no_return)
//         RawMemory.segments[@{id}] = @{data};
//     }
// }

// /// This drops the reference to a segment; it doesn't affect the content of the
// /// segment.
// ///
// /// This is the equivalent of doing `delete RawMemory.segments[id]`. Again, this
// /// only deletes the local view of the segment, not the serialized one. It may
// /// be used to `set_segment` a new segment that wasn't part of the original 10
// /// active segments.
// pub fn drop_segment(id: u32) {
//     js! { @(no_return)
//         delete RawMemory.segments[@{id}];
//     }
// }

// pub fn get_foreign_segment() -> ForeignSegment {
//     js_unwrap!(RawMemory.foreignSegment)
// }

// /// Implements `RawMemory.setActiveForeignSegment`
// ///
// /// To use the default public segment of `username` (as set with
// /// [`set_default_public_segment`]), Use `None` instead of `Some(id)`.
// ///
// /// To clear the foreign segment, pass the empty string `""` as a username.
// pub fn set_active_foreign_segment(username: &str, id: Option<u32>) {
//     if username == "" {
//         js! { @(no_return)
//             RawMemory.setActiveForeignSegment(null);
//         }
//     } else {
//         match id {
//             Some(id) => js! { @(no_return)
//                 RawMemory.setActiveForeignSegment(@{username}, @{id});
//             },
//             None => js! { @(no_return)
//                 RawMemory.setActiveForeignSegment(@{username});
//             },
//         };
//     };
// }

// pub fn set_default_public_segment(id: u32) {
//     js! { @(no_return)
//         RawMemory.setDefaultPublicSegment(@{id});
//     }
// }

// pub fn set_public_segments(ids: &[u32]) {
//     js! { @(no_return)
//         RawMemory.setPublicSegments(@{ids});
//     }
// }

// pub fn get() -> String {
//     js_unwrap!(RawMemory.get())
// }

// pub fn set(value: &str) {
//     js! { @(no_return)
//         RawMemory.set(@{value});
//     }
// }
