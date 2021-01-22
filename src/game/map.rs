//! Game map related functionality.
//!
//! [Screeps documentation](https://docs.screeps.com/api/#Game-map)

use js_sys::{Array, JsString, Object};

use wasm_bindgen::prelude::*;

use crate::{constants::ExitDirection, objects::RoomTerrain};

#[wasm_bindgen]
extern "C" {
    /// Object with info about the game map from [`Game::map`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game-map)
    ///
    /// [`Game::map`]: crate::game::Game::map
    #[wasm_bindgen]
    pub type MapInfo;

    /// Get an object with information about the exits from a given room, with
    /// [`JsString`] versions of direction integers as keys and [`JsString`]
    /// room names as values.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.map.describeExits)
    #[wasm_bindgen(method, js_name = describeExits)]
    pub fn describe_exits(this: &MapInfo, room_name: &JsString) -> Object;

    /// Get the exit direction from a given room leading toward a destination
    /// room, with an optional [`FindRouteOptions`] parameter allowing control
    /// over the costs to enter rooms.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.map.findExit)
    #[wasm_bindgen(method, js_name = findExit)]
    pub fn find_exit(
        this: &MapInfo,
        from_room: &JsString,
        to_room: &JsString,
        options: Option<&FindRouteOptions>,
    ) -> ExitDirection;

    /// Get the route from a given room leading toward a destination room, with
    /// an optional [`FindRouteOptions`] parameter allowing control over the
    /// costs to enter rooms.
    ///
    /// Returns an [`Array`] with an object per room in the route, with keys
    /// `exit` containing an [`ExitDirection`] and `room` containing room name
    /// as a [`JsString`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.map.findRoute)
    #[wasm_bindgen(method, js_name = findRoute)]
    pub fn find_route(
        this: &MapInfo,
        from_room: &JsString,
        to_room: &JsString,
        options: Option<&FindRouteOptions>,
    ) -> Array;

    /// Get the distance used for range calculations between two rooms,
    /// optionally setting `continuous` to true to consider the world borders to
    /// wrap around, which is used for terminal calculations.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.map.getRoomLinearDistance)
    #[wasm_bindgen(method, js_name = getRoomLinearDistance)]
    pub fn get_room_linear_distance(
        this: &MapInfo,
        room_1: &JsString,
        room_2: &JsString,
        continuous: bool,
    ) -> Array;

    /// Get the [`RoomTerrain`] object for any room, even one you don't have
    /// vision in.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.map.getRoomTerrain)
    #[wasm_bindgen(method, js_name = getRoomTerrain)]
    pub fn get_room_terrain(this: &MapInfo, room_name: &JsString) -> RoomTerrain;

    /// Get the size of the world map.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.map.getWorldSize)
    #[wasm_bindgen(method, js_name = getWorldSize)]
    pub fn get_world_size(this: &MapInfo) -> u32;

    // todo MapRoomStatus return val
    /// Get the status of a given room, determining whether it's in a special
    /// area or currently inaccessible.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.map.getRoomStatus)
    #[wasm_bindgen(method, js_name = getRoomStatus)]
    pub fn get_room_status(this: &MapInfo, room_name: &JsString) -> Object;

    // todo
    // /// Get a [`MapVisual`] object, allowing rendering visual indicators on the
    // game map. ///
    // /// [Screeps documentation](https://docs.screeps.com/api/#Game.map.visual)
    // #[wasm_bindgen(method, getter)]
    // pub fn visual(this: &MapInfo) -> MapVisual;
}

#[wasm_bindgen]
extern "C" {
    /// Object that represents a set of options for a call to
    /// [`MapInfo::find_exit`] or [`MapInfo::find_route`].
    #[wasm_bindgen]
    pub type FindRouteOptions;

    /// Route callback, which determines the cost of entering a given room (the
    /// first parameter) from a given neighbor room (the second parameter), or
    /// [`f64::INFINITY`] to block entry into the room.
    #[wasm_bindgen(method, setter = routeCallback)]
    pub fn route_callback(
        this: &FindRouteOptions,
        callback: &Closure<dyn FnMut(JsString, JsString) -> f64>,
    );
}

// //! See [http://docs.screeps.com/api/#Game.map]
// //!
// //! [http://docs.screeps.com/api/#Game.map]: http://docs.screeps.com/api/#Game.map
// use std::{borrow::Cow, collections, mem, str::FromStr};

// use num_traits::FromPrimitive;
// use parse_display::FromStr;
// use serde::{
//     de::{Deserializer, Error as _, Unexpected},
//     Deserialize,
// };
// use stdweb::Value;

// use crate::{
//     constants::{Direction, ExitDirection, ReturnCode},
//     local::RoomName,
//     objects::RoomTerrain,
//     traits::{TryFrom, TryInto},
// };

// /// See [http://docs.screeps.com/api/#Game.map.describeExits]
// ///
// /// [http://docs.screeps.com/api/#Game.map.describeExits]: http://docs.screeps.com/api/#Game.map.describeExits
// pub fn describe_exits(room_name: RoomName) -> collections::HashMap<Direction, RoomName> {
//     let orig: collections::HashMap<String, RoomName> =
//         js_unwrap!(Game.map.describeExits(@{room_name}) || {});

//     orig.into_iter()
//         .map(|(key, value)| {
//             let key: u32 = key.parse().expect(
//                 "expected all directions returned from Game.map.describeExits to be integers",
//             );
//             (
//                 Direction::from_u32(key).expect(
//                     "expected all directions returned from Game.map.describeExits to be directions",
//                 ),
//                 value,
//             )
//         })
//         .collect()
// }

// /// See [http://docs.screeps.com/api/#Game.map.getRoomLinearDistance]
// ///
// /// [http://docs.screeps.com/api/#Game.map.getRoomLinearDistance]: http://docs.screeps.com/api/#Game.map.getRoomLinearDistance
// pub fn get_room_linear_distance(room1: RoomName, room2: RoomName, continuous: bool) -> u32 {
//     js_unwrap!(Game.map.getRoomLinearDistance(@{room1}, @{room2}, @{continuous}))
// }

// pub fn get_room_terrain(room_name: RoomName) -> RoomTerrain {
//     js_unwrap!(Game.map.getRoomTerrain(@{room_name}))
// }

// /// See [http://docs.screeps.com/api/#Game.map.getWorldSize]
// ///
// /// [http://docs.screeps.com/api/#Game.map.getWorldSize]: http://docs.screeps.com/api/#Game.map.getWorldSize
// pub fn get_world_size() -> u32 {
//     js_unwrap!(Game.map.getWorldSize())
// }

// /// See [http://docs.screeps.com/api/#Game.map.getRoomStatus]
// ///
// /// [http://docs.screeps.com/api/#Game.map.getRoomStatus]: http://docs.screeps.com/api/#Game.map.getRoomStatus
// pub fn get_room_status(room_name: RoomName) -> MapRoomStatus {
//     js_unwrap!(Game.map.getRoomStatus(@{room_name}))
// }

// /// Represents the availability and respawn/novice state of a room on the map
// #[derive(Clone, Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct MapRoomStatus {
//     pub status: RoomStatus,
//     pub timestamp: Option<u64>,
// }
// js_deserializable!(MapRoomStatus);

// #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, FromStr)]
// #[display(style = "camelCase")]
// pub enum RoomStatus {
//     Normal,
//     Closed,
//     Novice,
//     Respawn,
// }

// impl<'de> Deserialize<'de> for RoomStatus {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s: Cow<'de, str> = Cow::deserialize(deserializer)?;
//         Self::from_str(&s).map_err(|_| {
//             D::Error::invalid_value(Unexpected::Str(&s), &"a known getRoomStatus status string")
//         })
//     }
// }

// /// Implements `Game.map.findExit`.
// pub fn find_exit(from_room: RoomName, to_room: RoomName) -> Result<ExitDirection, ReturnCode> {
//     let code: i32 = js_unwrap! {Game.map.findExit(@{from_room}, @{to_room})};
//     ExitDirection::from_i32(code)
//         .ok_or_else(|| ReturnCode::from_i32(code).expect("find_exit: Error code not recognized."))
// }

// pub fn find_exit_with_callback(
//     from_room: RoomName,
//     to_room: RoomName,
//     route_callback: impl FnMut(RoomName, RoomName) -> f64,
// ) -> Result<ExitDirection, ReturnCode> {
//     let mut raw_callback = route_callback;

//     let mut callback_boxed = move |to_name: RoomName, from_name: RoomName| -> f64 {
//         raw_callback(to_name, from_name).into()
//     };

//     // Type erased and boxed callback: no longer a type specific to the closure
//     // passed in, now unified as &Fn
//     let callback_type_erased: &mut (dyn FnMut(RoomName, RoomName) -> f64) =
//         &mut callback_boxed;

//     // Overwrite lifetime of reference so it can be passed to javascript. 
//     // It's now pretending to be static data. This should be entirely safe
//     // because we control the only use of it and it remains valid during the
//     // pathfinder callback. This transmute is necessary because "some lifetime 
//     // above the current scope but otherwise unknown" is not a valid lifetime.
//     let callback_lifetime_erased: &'static mut dyn FnMut(RoomName, RoomName) -> f64 =
//         unsafe { mem::transmute(callback_type_erased) };

//     let code: i32 = js!(
//         let cb = @{callback_lifetime_erased};
//         let res = Game.map.findExit(@{from_room}, @{to_room}, cb);
//         cb.drop();
//         return res;
//     )
//     .try_into()
//     .expect("expected int from findExit");

//     ExitDirection::from_i32(code)
//         .map(Ok)
//         .or_else(|| ReturnCode::from_i32(code).map(Err))
//         .unwrap_or_else(|| {
//             panic!(
//                 "find_exit: return value {:?} not recognized as either Exit nor ReturnCode",
//                 code
//             )
//         })
// }

// pub fn find_route(from_room: &str, to_room: &str) -> Result<Vec<RoomRouteStep>, ReturnCode> {
//     let v = js!(return Game.map.findRoute(@{from_room}, @{to_room}););
//     parse_find_route_returned_value(v)
// }

// pub fn find_route_with_callback(
//     from_room: RoomName,
//     to_room: RoomName,
//     route_callback: impl FnMut(RoomName, RoomName) -> f64,
// ) -> Result<Vec<RoomRouteStep>, ReturnCode> {
//     let mut raw_callback = route_callback;

//     let mut callback_boxed = move |to_name: RoomName, from_name: RoomName| -> f64 {
//         raw_callback(to_name, from_name).into()
//     };

//     // Type erased and boxed callback: no longer a type specific to the closure
//     // passed in, now unified as &Fn
//     let callback_type_erased: &mut (dyn FnMut(RoomName, RoomName) -> f64) =
//         &mut callback_boxed;

//     // Overwrite lifetime of reference so it can be passed to javascript. 
//     // It's now pretending to be static data. This should be entirely safe
//     // because we control the only use of it and it remains valid during the
//     // pathfinder callback. This transmute is necessary because "some lifetime 
//     // above the current scope but otherwise unknown" is not a valid lifetime.
//     let callback_lifetime_erased: &'static mut dyn FnMut(RoomName, RoomName) -> f64 =
//         unsafe { mem::transmute(callback_type_erased) };

//     let v = js!(
//         let cb = @{callback_lifetime_erased};
//         let res = Game.map.findRoute(@{from_room}, @{to_room}, { routeCallback: cb });
//         cb.drop();
//         return res;
//     );

//     parse_find_route_returned_value(v)
// }

// fn parse_find_route_returned_value(v: Value) -> Result<Vec<RoomRouteStep>, ReturnCode> {
//     match v {
//         Value::Number(x) => Err(ReturnCode::try_from(Value::Number(x)).unwrap_or_else(|e| {
//             panic!(
//                 "parse_find_route_returned_value: unknown return value: {:?} (err: {})",
//                 x, e
//             )
//         })),
//         Value::Reference(_) => Ok(v.try_into().expect("Error on parsing exit directions.")),
//         _ => panic!(
//             "Game.map.findRoute expected Number or Reference, found {:?}.",
//             v
//         ),
//     }
// }

// #[derive(Clone, Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct RoomRouteStep {
//     pub exit: ExitDirection,
//     pub room: RoomName,
// }
// js_deserializable!(RoomRouteStep);
