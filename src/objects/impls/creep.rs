use crate::{
    constants::{Direction, Part, ResourceType},
    objects::{
        ConstructionSite, Owner, Resource, Room, RoomObject, RoomPosition, Store, Structure,
        StructureController,
    },
    prelude::*,
};
use js_sys::{Array, JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// A [`Creep`] unit in the game world.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep)
    #[wasm_bindgen(extends = RoomObject)]
    pub type Creep;

    /// Retrieve an [`Array`] containing details about the creep's body parts
    /// and boosts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.body)
    #[wasm_bindgen(final, method, getter)]
    pub fn body(this: &Creep) -> Array;

    /// The amount of fatigue the creep has. If greater than 0, it cannot move
    /// this tick without being pulled.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.fatigue)
    #[wasm_bindgen(final, method, getter)]
    pub fn fatigue(this: &Creep) -> u32;

    /// Retrieve the current hits of this creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.hits)
    #[wasm_bindgen(final, method, getter)]
    pub fn hits(this: &Creep) -> u32;

    /// Retrieve the maximum hits of this creep, which generally equals 50 per
    /// body part.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.hitsMax)
    #[wasm_bindgen(final, method, getter = hitsMax)]
    pub fn hits_max(this: &Creep) -> u32;

    /// Object ID of the creep, which can be used to efficiently fetch a fresh
    /// reference to the object on subsequent ticks.  `None` if the creep began
    /// spawning this tick.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.id)
    #[wasm_bindgen(final, method, getter)]
    pub fn id(this: &Creep) -> Option<JsString>;

    /// A shortcut to `Memory.creeps[creep.name]`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.memory)
    #[wasm_bindgen(final, method, getter)]
    pub fn memory(this: &Creep) -> JsValue;

    /// Sets a new value to `Memory.creeps[creep.name]`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.memory)
    #[wasm_bindgen(final, method, setter)]
    pub fn set_memory(this: &Creep, val: &JsValue);

    /// Whether this creep is owned by the player.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.my)
    #[wasm_bindgen(final, method, getter)]
    pub fn my(this: &Creep) -> bool;

    /// The creep's name as an owned reference to a [`JsString`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.name)
    #[wasm_bindgen(final, method, getter)]
    pub fn name(this: &Creep) -> JsString;

    /// The [`Owner`] of this creep that contains the owner's username.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.owner)
    #[wasm_bindgen(final, method, getter)]
    pub fn owner(this: &Creep) -> Owner;

    /// What the creep said last tick.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.saying)
    #[wasm_bindgen(final, method, getter)]
    pub fn saying(this: &Creep) -> Option<JsString>;

    /// Whether the creep is still spawning.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.spawning)
    #[wasm_bindgen(final, method, getter)]
    pub fn spawning(this: &Creep) -> bool;

    /// The [`Store`] of the creep, which contains information about what
    /// resources it is it carrying.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &Creep) -> Store;

    /// The number of ticks the creep has left to live
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.ticksToLive)
    #[wasm_bindgen(final, method, getter = ticksToLive)]
    pub fn ticks_to_live(this: &Creep) -> u32;

    /// Attack a target in melee range using a creep's attack parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.attack)
    #[wasm_bindgen(final, method)]
    pub fn attack(this: &Creep, target: &RoomObject) -> i8;

    /// Attack a [`StructureController`] in melee range using a creep's claim
    /// parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.attackController)
    #[wasm_bindgen(final, method, js_name = attackController)]
    pub fn attack_controller(this: &Creep, target: &StructureController) -> i8;

    /// Use a creep's work parts to consume carried energy, putting it toward
    /// progress in a [`ConstructionSite`] in range 3.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.build)
    #[wasm_bindgen(final, method)]
    pub fn build(this: &Creep, target: &ConstructionSite) -> i8;

    /// Cancel an a successfully called creep function from earlier in the tick,
    /// with a [`JsString`] that must contain the JS version of the function
    /// name.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.cancelOrder)
    #[wasm_bindgen(final, method, js_name = cancelOrder)]
    pub fn cancel_order(this: &Creep, target: &JsString) -> i8;

    /// Claim an unowned [`StructureController`] in melee range as your own
    /// using a creep's claim parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.claimController)
    #[wasm_bindgen(final, method, js_name = claimController)]
    pub fn claim_controller(this: &Creep, target: &StructureController) -> i8;

    // todo constant links - REPAIR_POWER, DISMANTLE_POWER, and buildable types
    // which I think we have
    /// Dismantle a [`Structure`] in melee range, giving the creep energy
    /// equivalent to half of the cost to repair the same hits. Must be a type
    /// of structure that can be constructed.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.dismantle)
    #[wasm_bindgen(final, method)]
    pub fn dismantle(this: &Creep, target: &Structure) -> i8;

    /// Drop a resource on the ground from the creep's [`Store`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.drop)
    #[wasm_bindgen(final, method)]
    pub fn drop(this: &Creep, ty: ResourceType, amount: Option<u32>) -> i8;

    // todo constant link SAFE_MODE_COST
    /// Consume [`ResourceType::Ghodium`] from the creep's [`Store`] to add a
    /// safe mode activation to a [`StructureController`] in melee range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.generateSafeMode)
    #[wasm_bindgen(final, method, js_name = generateSafeMode)]
    pub fn generate_safe_mode(this: &Creep, target: &StructureController) -> i8;

    /// Get the number of parts of the given type the creep has in its body,
    /// excluding fully damaged parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.getActiveBodyparts)
    #[wasm_bindgen(final, method, js_name = getActiveBodyparts)]
    pub fn get_active_bodyparts(this: &Creep, ty: Part) -> u8;

    /// Harvest from a [`Source`], [`Mineral`], or [`Deposit`] in melee range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.harvest)
    #[wasm_bindgen(final, method)]
    pub fn harvest(this: &Creep, target: &RoomObject) -> i8;

    /// Heal a [`Creep`] or [`PowerCreep`] in melee range, including itself.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.heal)
    #[wasm_bindgen(final, method)]
    pub fn heal(this: &Creep, target: &RoomObject) -> i8;

    /// Move one square in the specified direction.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.move)
    #[wasm_bindgen(final, method, js_name = move)]
    pub fn move_direction(this: &Creep, direction: Direction) -> i8;

    /// Accept an attempt by another creep to pull this one.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.move)
    #[wasm_bindgen(final, method, js_name = move)]
    pub fn move_pulled_by(this: &Creep, target: &Creep) -> i8;

    /// Move the creep along a previously determined path returned from a
    /// pathfinding function, in array or serialized string form.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.moveByPath)
    #[wasm_bindgen(final, method, js_name = moveByPath)]
    pub fn move_by_path(this: &Creep, path: &JsValue) -> i8;

    /// Move the creep toward the specified goal, either a [`RoomPosition`] or
    /// [`RoomObject`]. Note that using this function will store data in
    /// `Memory.creeps[creep_name]` and enable the default serialization
    /// behavior of the `Memory` object, which may hamper attempts to directly
    /// use `RawMemory`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.moveByPath)
    #[wasm_bindgen(final, method, js_name = moveTo)]
    pub fn move_to(this: &Creep, target: &JsValue, options: Option<Object>) -> i8;

    /// Whether to send an email notification when this creep is attacked.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.notifyWhenAttacked)
    #[wasm_bindgen(final, method, js_name = notifyWhenAttacked)]
    pub fn notify_when_attacked(this: &Creep, enabled: bool) -> i8;

    /// Pick up a [`Resource`] in melee range (or at the same position as the
    /// creep).
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.pickup)
    #[wasm_bindgen(final, method)]
    pub fn pickup(this: &Creep, target: &Resource) -> i8;

    /// Help another creep to move by pulling, if the second creep accepts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.pull)
    #[wasm_bindgen(final, method)]
    pub fn pull(this: &Creep, target: &Creep) -> i8;

    /// Attack a target in range 3 using a creep's ranged attack parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.rangedAttack)
    #[wasm_bindgen(final, method, js_name = rangedAttack)]
    pub fn ranged_attack(this: &Creep, target: &RoomObject) -> i8;

    /// Heal a target in range 3 using a creep's heal parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.rangedHeal)
    #[wasm_bindgen(final, method, js_name = rangedHeal)]
    pub fn ranged_heal(this: &Creep, target: &RoomObject) -> i8;

    /// Attack all enemy targets in range using a creep's ranged attack parts,
    /// with lower damage depending on range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.rangedMassAttack)
    #[wasm_bindgen(final, method, js_name = rangedMassAttack)]
    pub fn ranged_mass_attack(this: &Creep) -> i8;

    /// Repair a target in range 3 using carried energy and the creep's work
    /// parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.repair)
    #[wasm_bindgen(final, method)]
    pub fn repair(this: &Creep, target: &RoomObject) -> i8;

    /// Reserve an unowned [`StructureController`] in melee range using a
    /// creep's claim parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.reserveController)
    #[wasm_bindgen(final, method, js_name = reserveController)]
    pub fn reserve_controller(this: &Creep, target: &StructureController) -> i8;

    /// Display a string in a bubble above the creep next tick. 10 character
    /// limit.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.say)
    #[wasm_bindgen(final, method)]
    pub fn say(this: &Creep, message: &JsString, public: bool) -> i8;

    /// Add (or remove, using an empty string) a sign to a
    /// [`StructureController`] in melee range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.signController)
    #[wasm_bindgen(final, method, js_name = signController)]
    pub fn sign_controller(this: &Creep, target: &StructureController) -> i8;

    /// Immediately kill the creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.suicide)
    #[wasm_bindgen(final, method)]
    pub fn suicide(this: &Creep) -> i8;

    /// Transfer a resource from the creep's store to [`Structure`],
    /// [`PowerCreep`], or another [`Creep`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.transfer)
    #[wasm_bindgen(final, method)]
    pub fn transfer(this: &Creep, target: &RoomObject, ty: ResourceType, amount: Option<u32>)
        -> i8;

    /// Upgrade a [`StructureController`] in range 3 using carried energy and
    /// the creep's work parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.upgradeController)
    #[wasm_bindgen(final, method, js_name = upgradeController)]
    pub fn upgrade_controller(this: &Creep, target: &StructureController) -> i8;

    /// Withdraw a resource from a [`Structure`], [`Tombstone`], or [`Ruin`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.withdraw)
    #[wasm_bindgen(final, method)]
    pub fn withdraw(this: &Creep, target: &RoomObject, ty: ResourceType, amount: Option<u32>)
        -> i8;
}

impl Attackable for Creep {
    fn hits(&self) -> u32 {
        Self::hits(self)
    }

    fn hits_max(&self) -> u32 {
        Self::hits_max(self)
    }
}
impl HasId for Creep {
    fn id(&self) -> Option<JsString> {
        Self::id(self)
    }
}
impl HasPosition for Creep {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl HasStore for Creep {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
impl RoomObjectProperties for Creep {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}

impl SharedCreepProperties for Creep {
    fn memory(&self) -> JsValue {
        Self::memory(self)
    }

    fn set_memory(&self, val: &JsValue) {
        Self::set_memory(self, val)
    }

    fn my(&self) -> bool {
        Self::my(self)
    }

    fn name(&self) -> JsString {
        Self::name(self)
    }

    fn owner(&self) -> Owner {
        Self::owner(self)
    }

    fn saying(&self) -> Option<JsString> {
        Self::saying(self)
    }

    fn ticks_to_live(&self) -> u32 {
        Self::ticks_to_live(self)
    }

    fn cancel_order(&self, target: &JsString) -> i8 {
        Self::cancel_order(self, target)
    }

    fn drop(&self, ty: ResourceType, amount: Option<u32>) -> i8 {
        Self::drop(self, ty, amount)
    }

    fn move_direction(&self, direction: Direction) -> i8 {
        Self::move_direction(self, direction)
    }

    fn move_by_path(&self, path: &JsValue) -> i8 {
        Self::move_by_path(self, path)
    }

    fn move_to(&self, target: &JsValue, options: Option<Object>) -> i8 {
        Self::move_to(self, target, options)
    }

    fn notify_when_attacked(&self, enabled: bool) -> i8 {
        Self::notify_when_attacked(self, enabled)
    }

    fn pickup(&self, target: &Resource) -> i8 {
        Self::pickup(self, target)
    }

    fn say(&self, message: &JsString, public: bool) -> i8 {
        Self::say(self, message, public)
    }

    fn suicide(&self) -> i8 {
        Self::suicide(self)
    }

    fn transfer(&self, target: &RoomObject, ty: ResourceType, amount: Option<u32>) -> i8 {
        Self::transfer(self, target, ty, amount)
    }

    fn withdraw(&self, target: &RoomObject, ty: ResourceType, amount: Option<u32>) -> i8 {
        Self::withdraw(self, target, ty, amount)
    }
}
