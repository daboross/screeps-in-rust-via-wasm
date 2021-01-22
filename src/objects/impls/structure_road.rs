use crate::{
    objects::{Room, RoomObject, RoomPosition, Structure},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureRoad`], which allows creeps to move
    /// onto this position for half of the fatigue of moving onto a plains tile,
    /// as well as through terrain walls.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureRoad)
    #[wasm_bindgen(extends = RoomObject, extends = Structure)]
    pub type StructureRoad;

    /// The number of ticks until the road will decay, losing
    /// [`ROAD_DECAY_AMOUNT`] hits.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureRoad.ticksToDecay)
    ///
    /// [`ROAD_DECAY_AMOUNT`]: crate::constants::ROAD_DECAY_AMOUNT
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &StructureRoad) -> u32;
}

impl Attackable for StructureRoad {
    fn hits(&self) -> u32 {
        Structure::hits(self.as_ref())
    }

    fn hits_max(&self) -> u32 {
        Structure::hits_max(self.as_ref())
    }
}
impl CanDecay for StructureRoad {
    fn ticks_to_decay(&self) -> u32 {
        Self::ticks_to_decay(self)
    }
}
impl HasId for StructureRoad {
    fn id(&self) -> Option<JsString> {
        Some(Structure::id(self.as_ref()))
    }
}
impl HasPosition for StructureRoad {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl RoomObjectProperties for StructureRoad {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
impl StructureProperties for StructureRoad {}
