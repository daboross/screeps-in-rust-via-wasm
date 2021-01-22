use crate::{
    constants::ResourceType,
    objects::{OwnedStructure, Owner, Room, RoomObject, RoomPosition, Store, Structure},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureTerminal`], which can send resources
    /// to distant rooms and participate in the market.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTerminal)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureTerminal;

    /// The number of ticks until the [`StructureTerminal`] can use
    /// [`StructureTerminal::send`] or be used in a market transaction again.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTerminal.cooldown)
    #[wasm_bindgen(method, getter)]
    pub fn cooldown(this: &StructureTerminal) -> u32;

    /// The [`Store`] of the terminal, which contains information about what
    /// resources it is it holding.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTerminal.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureTerminal) -> Store;

    /// Send resources to another room's terminal.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTerminal.send)
    #[wasm_bindgen(method)]
    pub fn send(
        this: &StructureTerminal,
        resource_type: ResourceType,
        amount: u32,
        destination: &JsString,
        description: Option<&JsString>,
    ) -> i8;
}

impl Attackable for StructureTerminal {
    fn hits(&self) -> u32 {
        Structure::hits(self.as_ref())
    }

    fn hits_max(&self) -> u32 {
        Structure::hits_max(self.as_ref())
    }
}
impl HasId for StructureTerminal {
    fn id(&self) -> Option<JsString> {
        Some(Structure::id(self.as_ref()))
    }
}
impl HasCooldown for StructureTerminal {
    fn cooldown(&self) -> u32 {
        Self::cooldown(self)
    }
}
impl HasPosition for StructureTerminal {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl HasStore for StructureTerminal {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
impl OwnedStructureProperties for StructureTerminal {
    fn my(&self) -> bool {
        OwnedStructure::my(self.as_ref())
    }

    fn owner(&self) -> Option<Owner> {
        OwnedStructure::owner(self.as_ref())
    }
}
impl RoomObjectProperties for StructureTerminal {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
impl StructureProperties for StructureTerminal {}
