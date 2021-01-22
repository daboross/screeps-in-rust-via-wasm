//! Prototypes of Screeps object types, used for constructing some object types
//! without using their default constructor.
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// The object prototype of [`RoomPosition`] objects, for use in manual
    /// object construction
    ///
    /// [`RoomPosition`]: crate::objects::RoomPosition
    #[wasm_bindgen(js_name = prototype, js_namespace = RoomPosition)]
    pub static ROOM_POSITION_PROTOTYPE: Object;

    /// The object prototype of [`CostMatrix`] objects, for use in manual object
    /// construction
    ///
    /// [`CostMatrix`]: crate::objects::CostMatrix
    #[wasm_bindgen(js_name = prototype, js_namespace = ["PathFinder", "CostMatrix"])]
    pub static COST_MATRIX_PROTOTYPE: Object;
}
