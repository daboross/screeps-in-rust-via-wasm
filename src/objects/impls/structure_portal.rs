use crate::{
    objects::{Room, RoomObject, RoomPosition, Structure},
    prelude::*,
};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructurePortal`], which allows movement
    /// between remote locations or other shards.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePortal)
    #[wasm_bindgen(extends = RoomObject, extends = Structure)]
    pub type StructurePortal;

    // todo: destination

    /// The number of ticks until the portal will decay, if it's unstable, or 0
    /// if it's stable.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePortal.ticksToDecay)
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &StructurePortal) -> u32;
}

impl CanDecay for StructurePortal {
    fn ticks_to_decay(&self) -> u32 {
        Self::ticks_to_decay(self)
    }
}

impl HasId for StructurePortal {
    fn id(&self) -> Option<JsString> {
        Some(Structure::id(self.as_ref()))
    }
}
impl HasPosition for StructurePortal {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl RoomObjectProperties for StructurePortal {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
impl StructureProperties for StructurePortal {}

// use serde::Deserialize;
// use stdweb::Value;

// use crate::{
//     local::{Position, RoomName},
//     objects::StructurePortal,
//     traits::TryInto,
// };

// #[derive(Deserialize, Debug)]
// pub struct InterShardPortalDestination {
//     shard: String,
//     room: RoomName,
// }
// js_deserializable!(InterShardPortalDestination);

// pub enum PortalDestination {
//     InterRoom(Position),
//     InterShard(InterShardPortalDestination),
// }

// impl StructurePortal {
//     pub fn destination(&self) -> PortalDestination {
//         let v = js! {
//             let destination = @{self.as_ref()}.destination;
//             if (destination instanceof Position) {
//                 return destination.__packedPos;
//             } else {
//                 return destination;
//             }
//         };

//         match v {
//             Value::Number(_) => PortalDestination::InterRoom(
//                 v.try_into()
//                     .expect("expected Position::try_from(pos.__packedPos) to
// succeed"),             ),
//             _ => PortalDestination::InterShard(
//                 v.try_into()
//                     .expect("Value couldn't be converted into an
// InterShardPortalDestination"),             ),
//         }
//     }
// }
