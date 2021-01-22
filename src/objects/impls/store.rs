use crate::constants::ResourceType;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object that represents the cargo within an entity in the game world.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Store)
    #[wasm_bindgen]
    pub type Store;

    /// Get the capacity of the [`Store`] for the specified resource. If the
    /// [`Store`] can contain any resource, passing `None` as the type will get
    /// the general store capacity.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Store.getCapacity)
    #[wasm_bindgen(method, js_name = getCapacity)]
    pub fn get_capacity(this: &Store, ty: Option<ResourceType>) -> u32;

    /// Return the free capacity of the [`Store`] for the specified resource.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Store.getFreeCapacity)
    #[wasm_bindgen(method, js_name = getFreeCapacity)]
    pub fn get_free_capacity(this: &Store, ty: Option<ResourceType>) -> u32;

    /// Return the used capacity of the [`Store`] for the specified resource. If
    /// the [`Store`] can contain any resource, passing `None` as the type will
    /// get the total used capacity.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Store.getUsedCapacity)
    #[wasm_bindgen(method, js_name = getUsedCapacity)]
    pub fn get_used_capacity(this: &Store, ty: Option<ResourceType>) -> u32;
}
