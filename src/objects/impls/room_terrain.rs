use js_sys::{JsString, Uint8Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a room's terrain held in the javascript heap.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room-Terrain)
    #[wasm_bindgen(js_namespace = Room, js_name = Terrain)]
    pub type RoomTerrain;

    /// Gets the terrain for any room by name, regardless of current visibility
    /// of the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.Terrain.constructor)
    #[wasm_bindgen(constructor, js_namespace = Room)]
    pub fn new(room_name: &JsString) -> RoomTerrain;

    /// Get the type of type at given coordinates.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.Terrain.get)
    #[wasm_bindgen(method)]
    pub fn get(this: &RoomTerrain, x: u8, y: u8) -> u8;

    /// Get a copy of the underlying Uint8Array with the data about the room's
    /// terrain.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.Terrain.get)
    #[wasm_bindgen(method)]
    pub fn get_raw_buffer(this: &RoomTerrain) -> Uint8Array;

    /// Copy the data about the room's terrain into an existing [`Uint8Array`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.Terrain.get)
    #[wasm_bindgen(method)]
    pub fn get_raw_buffer_to_array(this: &RoomTerrain, destination: &Uint8Array) -> i8;
}
