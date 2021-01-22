use crate::{
    objects::{OwnedStructure, Owner, Room, RoomObject, RoomPosition, Structure},
    prelude::*,
};
use js_sys::{Array, Date, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureController`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureController;

    /// Whether power is enabled in the room, allowing power creeps to use
    /// powers.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.isPowerEnabled)
    #[wasm_bindgen(method, getter = isPowerEnabled)]
    pub fn is_power_enabled(this: &StructureController) -> bool;

    /// The current room control level (RCL) of the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.level)
    #[wasm_bindgen(method, getter)]
    pub fn level(this: &StructureController) -> u8;

    /// The progress toward upgrading the controller to the next level
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.progress)
    #[wasm_bindgen(method, getter)]
    pub fn progress(this: &StructureController) -> u32;

    /// The total [`StructureController::progress`] needed to upgrade the
    /// controller to the next level.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.progressTotal)
    #[wasm_bindgen(method, getter = progressTotal)]
    pub fn progress_total(this: &StructureController) -> u32;

    /// Information about the reservation of this controller, if it is currently
    /// reserved.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.reservation)
    #[wasm_bindgen(method, getter)]
    pub fn reservation(this: &StructureController) -> Option<Reservation>;

    /// The number of ticks remaining in safe mode, or 0 if safe mode isn't
    /// currently active.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.safeMode)
    #[wasm_bindgen(method, getter = safeMode)]
    pub fn safe_mode(this: &StructureController) -> u32;

    /// The number of of available safe mode activations, which can be increased
    /// by using [`Creep::generate_safe_mode`]
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.safeModeAvailable)
    ///
    /// [`Creep::generate_safe_mode`]: crate::objects::Creep::generate_safe_mode
    #[wasm_bindgen(method, getter = safeModeAvailable)]
    pub fn safe_mode_available(this: &StructureController) -> u32;

    /// The cooldown remaining until safe mode can be activated again.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.safeModeCooldown)
    #[wasm_bindgen(method, getter = safeModeCooldown)]
    pub fn safe_mode_cooldown(this: &StructureController) -> u32;

    /// Information about the sign on this controller, if it has been signed by
    /// [`Creep::sign_controller`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.sign)
    ///
    /// [`Creep::sign_controller`]: crate::objects::Creep::sign_controller
    #[wasm_bindgen(method, getter)]
    pub fn sign(this: &StructureController) -> Option<Sign>;

    /// The number of ticks until the level of the controller will be
    /// decremented due to a lack of [`Creep::upgrade_controller`] activity.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.ticksToDowngrade)
    #[wasm_bindgen(method, getter = ticksToDowngrade)]
    pub fn ticks_to_downgrade(this: &StructureController) -> u32;

    /// The number of ticks until the controller can be upgraded, or have safe
    /// mode activated, due to [`Creep::attack_controller`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.upgradeBlocked)
    ///
    /// [`Creep::attack_controller`]: crate::objects::Creep::attack_controller
    #[wasm_bindgen(method, getter = upgradeBlocked)]
    pub fn upgrade_blocked(this: &StructureController) -> u32;

    /// Activate safe mode for the room, preventing hostile creep actions in the
    /// room for 20,000 ticks
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.activateSafeMode)
    #[wasm_bindgen(method, js_name = activateSafeMode)]
    pub fn activate_safe_mode(this: &StructureController) -> i8;

    /// Relinquish ownership of the controller and its room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.unclaim)
    #[wasm_bindgen(method)]
    pub fn unclaim(this: &StructureController) -> i8;
}

impl HasId for StructureController {
    fn id(&self) -> Option<JsString> {
        Self::id(self.as_ref())
    }
}
impl HasPosition for StructureController {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}
impl OwnedStructureProperties for StructureController {
    fn my(&self) -> bool {
        OwnedStructure::my(self.as_ref())
    }

    fn owner(&self) -> Option<Owner> {
        OwnedStructure::owner(self.as_ref())
    }
}
impl RoomObjectProperties for StructureController {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
impl StructureProperties for StructureController {}

#[wasm_bindgen]
extern "C" {
    /// Object with info on who has reserved this [`StructureController`]
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.reservation)
    #[wasm_bindgen]
    pub type Reservation;

    /// The name of the player that has reserved this controller as a
    /// [`JsString`].
    #[wasm_bindgen(method, getter)]
    pub fn username(this: &Reservation) -> JsString;

    /// The number of ticks until the reservation expires.
    #[wasm_bindgen(method, getter = ticksToEnd)]
    pub fn ticks_to_end(this: &Reservation) -> u32;
}

#[wasm_bindgen]
extern "C" {
    /// Object with info on the sign on a [`StructureController`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureController.sign)
    #[wasm_bindgen]
    pub type Sign;

    /// The name of the player that has reserved this controller as a
    /// [`JsString`].
    #[wasm_bindgen(method, getter)]
    pub fn username(this: &Sign) -> JsString;

    /// The text of the sign on this controller as a [`JsString`].
    #[wasm_bindgen(method, getter)]
    pub fn text(this: &Sign) -> JsString;

    /// The tick when this sign was written.
    #[wasm_bindgen(method, getter)]
    pub fn time(this: &Sign) -> u32;

    /// The timestamp of when this sign was written.
    #[wasm_bindgen(method, getter)]
    pub fn datetime(this: &Sign) -> Date;
}
