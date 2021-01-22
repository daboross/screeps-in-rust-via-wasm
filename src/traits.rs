use enum_dispatch::enum_dispatch;
use js_sys::{Array, JsString, Object};
use wasm_bindgen::prelude::*;

use crate::{constants::*, enums::*, objects::*};

#[enum_dispatch]
pub trait Attackable {
    /// Retrieve the current hits of this object.
    fn hits(&self) -> u32;

    /// Retrieve the maximum hits of this object.
    fn hits_max(&self) -> u32;
}

#[enum_dispatch]
pub trait CanDecay {
    /// The number of ticks until the object will decay, losing hits.
    fn ticks_to_decay(&self) -> u32;
}

#[enum_dispatch]
pub trait HasCooldown {
    /// The number of ticks until the object can be used again.
    fn cooldown(&self) -> u32;
}

#[enum_dispatch]
pub trait HasId {
    /// Object ID of the object, which can be used to efficiently fetch a
    /// fresh reference to the object on subsequent ticks, or `None` if the
    /// object doesn't currently have an id.
    fn id(&self) -> Option<JsString>;
}

#[enum_dispatch]
pub trait HasPosition {
    /// Position of the object, or `None` if the object is a power creep not
    /// spawned on the current shard.
    fn pos(&self) -> Option<RoomPosition>;
}

#[enum_dispatch]
pub trait HasStore {
    /// The store of the object, containing information about the resources it
    /// is holding.
    fn store(&self) -> Store;
}

#[enum_dispatch]
pub trait OwnedStructureProperties {
    /// Whether this structure is owned by the player.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#OwnedStructure.my)
    fn my(&self) -> bool;

    /// The [`Owner`] of this structure that contains the owner's username, or
    /// `None` if it's an ownable structure currently not under a player's
    /// control.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#OwnedStructure.owner)
    fn owner(&self) -> Option<Owner>;
}

#[enum_dispatch]
pub trait RoomObjectProperties {
    /// Effects applied to the object.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomObject.effects)
    fn effects(&self) -> Array;

    /// A link to the room that the object is currently in, or `None` if the
    /// object is a power creep not spawned on the current shard, or a flag or
    /// construction site not in a visible room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomObject.room)
    fn room(&self) -> Option<Room>;
}

#[enum_dispatch]
pub trait SharedCreepProperties {
    /// A shortcut to the part of the `Memory` tree used for this creep by
    /// default
    fn memory(&self) -> JsValue;

    /// Sets a new value to the memory object shortcut for this creep.
    fn set_memory(&self, val: &JsValue);

    /// Whether this creep is owned by the player.
    fn my(&self) -> bool;

    /// The creep's name as an owned reference to a [`JsString`].
    fn name(&self) -> JsString;

    /// The [`Owner`] of this creep that contains the owner's username.
    fn owner(&self) -> Owner;

    /// What the creep said last tick.
    fn saying(&self) -> Option<JsString>;

    /// The number of ticks the creep has left to live.
    fn ticks_to_live(&self) -> u32;

    /// Cancel an a successfully called creep function from earlier in the tick,
    /// with a [`JsString`] that must contain the JS version of the function
    /// name.
    fn cancel_order(&self, target: &JsString) -> i8;

    /// Drop a resource on the ground from the creep's [`Store`].
    fn drop(&self, ty: ResourceType, amount: Option<u32>) -> i8;

    /// Move one square in the specified direction.
    fn move_direction(&self, direction: Direction) -> i8;

    /// Move the creep along a previously determined path returned from a
    /// pathfinding function, in array or serialized string form.
    fn move_by_path(&self, path: &JsValue) -> i8;

    /// Move the creep toward the specified goal, either a [`RoomPosition`] or
    /// [`RoomObject`]. Note that using this function will store data in
    /// `Memory.creeps[creep_name]` and enable the default serialization
    /// behavior of the `Memory` object, which may hamper attempts to directly
    /// use `RawMemory`.
    fn move_to(&self, target: &JsValue, options: Option<Object>) -> i8;

    /// Whether to send an email notification when this creep is attacked.
    fn notify_when_attacked(&self, enabled: bool) -> i8;

    /// Pick up a [`Resource`] in melee range (or at the same position as the
    /// creep).
    fn pickup(&self, target: &Resource) -> i8;

    /// Display a string in a bubble above the creep next tick. 10 character
    /// limit.
    fn say(&self, message: &JsString, public: bool) -> i8;

    /// Immediately kill the creep.
    fn suicide(&self) -> i8;

    /// Transfer a resource from the creep's store to [`Structure`],
    /// [`PowerCreep`], or another [`Creep`].
    fn transfer(&self, target: &RoomObject, ty: ResourceType, amount: Option<u32>) -> i8;

    /// Withdraw a resource from a [`Structure`], [`Tombstone`], or [`Ruin`].
    fn withdraw(&self, target: &RoomObject, ty: ResourceType, amount: Option<u32>) -> i8;
}

#[enum_dispatch]
pub trait StructureProperties {}
