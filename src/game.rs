//! The main interface to objects in the Screeps game world.
//!
//! This contains all functionality from the [`Game`] object in Screeps. That
//! generally means all state which is true this tick throughout the world.
//!
//! [Screeps documentation](http://docs.screeps.com/api/#Game)

use std::{convert::{TryFrom, TryInto}, marker::PhantomData};

use js_sys::{JsString, Object, Array};

use wasm_bindgen::{
    JsCast,
    prelude::*
};

use crate::{RoomName, local::{JsObjectId, ObjectId, RawObjectId}};

pub mod cpu;
pub mod gcl;
pub mod gpl;
pub mod map;
pub mod market;

use self::{cpu::CpuInfo, gcl::GclInfo, gpl::GplInfo, market::MarketInfo};
use crate::Room;
use crate::objects::RoomObject;


pub struct JsHashMap<K, V> {
    map: Object,
    _phantom: PhantomData<(K, V)>
}

impl<K, V> JsHashMap<K, V> where K: From<JsValue> {
    pub fn keys(&self) -> impl Iterator<Item = K> {
        let array = Object::keys(self.map.unchecked_ref());

        OwnedArrayIter::new(array)
    }  
}

impl<K, V> JsHashMap<K, V> where V: From<JsValue> {
    pub fn values(&self) -> impl Iterator<Item = V> {
        let array = Object::values(self.map.unchecked_ref());

        OwnedArrayIter::new(array)
    }  
}

impl<K, V> JsHashMap<K, V> where K: Into<JsValue>, V: From<JsValue> {
    pub fn get<'a>(&self, key: &'a K) -> Option<V> where &'a K: Into<JsValue> {
        let key = key.into();
        let val = js_sys::Reflect::get(&self.map, &key).ok()?;

        Some(val.into())
    }    
}

impl<K, V> JsHashMap<K, V> where K: Into<JsValue>, V: TryFrom<JsValue> {
    pub fn try_get<'a>(&self, key: &'a K) -> Option<V> where &'a K: Into<JsValue> {
        let key = key.into();
        let val = js_sys::Reflect::get(&self.map, &key).ok()?;
        let val = val.try_into().ok()?;

        Some(val)
    }    
}

impl<K, V> From<Object> for JsHashMap<K, V> {
    fn from(map: Object) -> Self {
        Self {
            map,
            _phantom: Default::default()
        }
    }
}

impl<K, V> From<JsValue> for JsHashMap<K, V> {
    fn from(val: JsValue) -> Self {
        Self {
            map: val.into(),
            _phantom: Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct OwnedArrayIter<T> {
    range: std::ops::Range<u32>,
    array: Array,
    _phantom: PhantomData<T>
}

impl<T> OwnedArrayIter<T> {
    pub fn new(array: Array) -> Self {
        OwnedArrayIter {
            range: 0..array.length(),
            array: array,
            _phantom: Default::default()
        }
    }
}

impl<T> std::iter::Iterator for OwnedArrayIter<T> where T: From<JsValue> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.range.next()?;
        Some(self.array.get(index).into())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl<T> std::iter::DoubleEndedIterator for OwnedArrayIter<T> where T: From<JsValue> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = self.range.next_back()?;
        Some(self.array.get(index).into())
    }
}

impl<T> std::iter::FusedIterator for OwnedArrayIter<T> where T: From<JsValue> {}

impl<T> std::iter::ExactSizeIterator for OwnedArrayIter<T> where T: From<JsValue> {}

#[wasm_bindgen]
extern "C" {
    pub type Game;

    /// Get an [`Object`] with all of your construction sites, which contains
    /// object ids in [`JsString`] form as keys and [`ConstructionSite`] values.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.constructionSites)
    ///
    /// [`ConstructionSite`]: crate::objects::ConstructionSite
    #[wasm_bindgen(static_method_of = Game, getter = constructionSites)]
    pub fn construction_sites() -> Object;

    /// Get a [`CpuInfo`] object, which contains properties and methods to get
    /// information about and manage your CPU and memory resource usage.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.cpu)
    #[wasm_bindgen(static_method_of = Game, getter)]
    pub fn cpu() -> CpuInfo;

    /// Get an [`Object`] with all of your creeps, which contains creep names in
    /// [`JsString`] form as keys and [`Creep`] objects as values. Note that
    /// newly spawned creeps are immediately added to the hash, but will not
    /// have an id until the following tick.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.creeps)
    ///
    /// [`Creep`]: crate::objects::Creep
    #[wasm_bindgen(static_method_of = Game, getter)]
    pub fn creeps() -> Object;

    /// Get an [`Object`] with all of your flags, which contains flag names in
    /// [`JsString`] form as keys and [`Flag`] objects as values.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.flags)
    ///
    /// [`Flag`]: crate::objects::Flag
    #[wasm_bindgen(static_method_of = Game, getter)]
    pub fn flags() -> Object;

    /// Get a [`GclInfo`] object, which contains properties about your global
    /// control level (GCL).
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.gcl)
    #[wasm_bindgen(static_method_of = Game, getter)]
    pub fn gcl() -> GclInfo;

    /// Get a [`GplInfo`] object, which contains properties about your global
    /// power level (GPL).
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.gpl)
    #[wasm_bindgen(static_method_of = Game, getter)]
    pub fn gpl() -> GplInfo;

    /// Get a [`MarketInfo`] object, which contains methods for getting
    /// information about the market and trading with other players.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.market)
    #[wasm_bindgen(static_method_of = Game, getter)]
    pub fn market() -> MarketInfo;

    /// Get an [`Object`] with all of your power creeps, which contains creep
    /// names in [`JsString`] form as keys and [`PowerCreep`] objects as values.
    /// Note that these power creeps may not be spawned on the current shard,
    /// and will not have a position or id if they are not.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.powerCreeps)
    ///
    /// [`PowerCreep`]: crate::objects::PowerCreep
    #[wasm_bindgen(static_method_of = Game, getter = powerCreeps)]
    pub fn power_creeps() -> Object;

    /// Get an [`Object`] with all of your account resources, with
    /// [`IntershardResourceType`] keys and integer values.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.resources)
    #[wasm_bindgen(static_method_of = Game, getter)]
    pub fn resources() -> Object;

    /// Get an [`Object`] with the rooms visible for the current tick, which
    /// contains room names in [`JsString`] form as keys and [`Room`] objects as
    /// values.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.rooms)
    ///
    /// [`Room`]: crate::objects::Room
    #[wasm_bindgen(static_method_of = Game, getter)]
    fn rooms_internal() -> Object;

    /// Get a [`JsString`] with the name of the shard being run on.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.shard)
    #[wasm_bindgen(static_method_of = Game, getter)]
    pub fn shard() -> JsString;

    /// Get an [`Object`] with all of your spawns, which contains spawn names in
    /// [`JsString`] form as keys and [`StructureSpawn`] objects as values.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.spawns)
    ///
    /// [`StructureSpawn`]: crate::objects::StructureSpawn
    #[wasm_bindgen(static_method_of = Game, getter)]
    pub fn spawns() -> Object;

    /// Get an [`Object`] with all of your owned structures, which contains
    /// object IDs in [`JsString`] form as keys and [`Structure`] objects as
    /// values.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.spawns)
    ///
    /// [`Structure`]: crate::objects::Structure
    #[wasm_bindgen(static_method_of = Game, getter)]
    pub fn structures() -> Object;

    /// Get the current time, the number of ticks the game has been running.
    ///
    /// [Screeps documentation](http://docs.screeps.com/api/#Game.time)
    #[wasm_bindgen(static_method_of = Game, getter)]
    pub fn time() -> u32;

    /// Your current score, as determined by the symbols you have decoded.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#Game.score)
    #[cfg(feature = "enable-symbols")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-symbols")))]
    #[wasm_bindgen(static_method_of = Game, getter)]
    pub fn score() -> u32;

    /// The symbols you've decoded after multiplier adjustments, used to
    /// determine your score.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#Game.symbols)
    #[cfg(feature = "enable-symbols")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-symbols")))]
    #[wasm_bindgen(static_method_of = Game, getter)]
    pub fn symbols() -> HashMap<ResourceType, u32>;

    /// Get the [`RoomObject`] represented by a given object ID, if it is still
    /// alive and visible.
    ///
    /// [Screeps documentation](http://docs.screeps.com/api/#Game.getObjectById)
    #[wasm_bindgen(static_method_of = Game, js_name = getObjectById)]
    pub fn get_object_by_id(id: &JsString) -> Option<RoomObject>;

    /// Send an email message to yourself with a given message. Set a group
    /// interval to only send messages every `group_interval` minutes.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.notify)
    #[wasm_bindgen(static_method_of = Game)]
    pub fn notify(message: &JsString, group_interval: Option<u32>);
}

impl Game {
    /// Get the typed object represented by a given [`JsObjectId`], if it's
    /// still alive and visible.
    ///
    /// [Screeps documentation](http://docs.screeps.com/api/#Game.getObjectById)
    pub fn get_object_by_js_id_typed<T>(id: &JsObjectId<T>) -> Option<T>
    where
        T: From<JsValue>,
    {
        Game::get_object_by_id(&id.raw)
            .map(JsValue::from)
            .map(Into::into)
    }

    /// Get the typed object represented by a given [`ObjectId`], if it's still
    /// alive and visible.
    ///
    /// [Screeps documentation](http://docs.screeps.com/api/#Game.getObjectById)
    pub fn get_object_by_id_typed<T>(id: &ObjectId<T>) -> Option<T>
    where
        T: From<JsValue>,
    {
        // construct a reference to a javascript string using the id data
        let js_str = JsString::from(id.to_string());

        Game::get_object_by_id(&js_str)
            .map(JsValue::from)
            .map(Into::into)
    }

    /// Get the [`RoomObject`] represented by a given [`RawObjectId`], if it's
    /// still alive and visible.
    ///
    /// [Screeps documentation](http://docs.screeps.com/api/#Game.getObjectById)
    pub fn get_object_by_id_erased(id: &RawObjectId) -> Option<RoomObject> {
        // construct a reference to a javascript string using the id data
        let js_str = JsString::from(id.to_string());

        Game::get_object_by_id(&js_str)
    }

    /// Get an [`JsHashMap<RoomName, Room>`] with the rooms visible for the current tick, which
    /// contains room names in [`RoomName`] form as keys and [`Room`] objects as
    /// values.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.rooms)
    ///
    /// [`Room`]: crate::objects::Room
    pub fn rooms() -> JsHashMap<RoomName, Room> {
        Game::rooms_internal().into()
    }
}

// pub fn get_object_typed<T>(id: ObjectId<T>) -> Result<Option<T>,
// ConversionError> where
//     T: HasId + SizedRoomObject,
// {
//     let array_view = unsafe { id.unsafe_as_uploaded() };
//     (js! {
//         return Game.getObjectById(object_id_from_packed(@{array_view}));
//     })
//     .try_into()
// }

// /// See [http://docs.screeps.com/api/#Game.constructionSites]
// ///
// /// [http://docs.screeps.com/api/#Game.constructionSites]: http://docs.screeps.com/api/#Game.constructionSites
// pub mod construction_sites {
//     game_map_access!(objects::ConstructionSite, Game.constructionSites);
// }

// /// See [http://docs.screeps.com/api/#Game.creeps]
// ///
// /// [http://docs.screeps.com/api/#Game.creeps]: http://docs.screeps.com/api/#Game.creeps
// pub mod creeps {
//     game_map_access!(objects::Creep, Game.creeps);
// }

// /// See [http://docs.screeps.com/api/#Game.flags]
// ///
// /// [http://docs.screeps.com/api/#Game.flags]: http://docs.screeps.com/api/#Game.flags
// pub mod flags {
//     game_map_access!(objects::Flag, Game.flags);
// }

// /// See [http://docs.screeps.com/api/#Game.powerCreeps]
// ///
// /// [http://docs.screeps.com/api/#Game.powerCreeps]: http://docs.screeps.com/api/#Game.powerCreeps
// pub mod power_creeps {
//     game_map_access!(objects::AccountPowerCreep, Game.powerCreeps);
// }

// /// See [http://docs.screeps.com/api/#Game.resources]
// ///
// /// [http://docs.screeps.com/api/#Game.resources]: http://docs.screeps.com/api/#Game.resources
// pub mod resources {
//     use std::collections::HashMap;

//     use crate::constants::IntershardResourceType;

//     /// Retrieve the full `HashMap<IntershardResourceType, u32>`.
//     pub fn hashmap() -> HashMap<IntershardResourceType, u32> {
//         // `TryFrom<Value>` is only implemented for `HashMap<String, V>`.
//         //
//         // See https://github.com/koute/stdweb/issues/359.
//         let map: HashMap<String, u32> = js_unwrap!(Game.resources);
//         map.into_iter()
//             .map(|(key, val)| {
//                 (
//                     key.parse()
//                         .expect("expected resource key in Game.resources to
// be a known intershard resource type"),                     val,
//                 )
//             })
//             .collect()
//     }

//     /// Retrieve the string keys of this object.
//     pub fn keys() -> Vec<IntershardResourceType> {
//         js_unwrap!(Object.keys(Game.resources).
// map(__resource_type_str_to_num))     }

//     /// Retrieve a specific value by key.
//     pub fn get(key: IntershardResourceType) -> Option<u32> {
//         js_unwrap!(Game.resources[__resource_type_num_to_str(@{key as u32})])
//     }
// }

// /// See [http://docs.screeps.com/api/#Game.spawns]
// ///
// /// [http://docs.screeps.com/api/#Game.spawns]: http://docs.screeps.com/api/#Game.spawns
// pub mod spawns {
//     game_map_access!(objects::StructureSpawn, Game.spawns);
// }

// /// See [http://docs.screeps.com/api/#Game.structures]
// ///
// /// [http://docs.screeps.com/api/#Game.structures]: http://docs.screeps.com/api/#Game.structures
// pub mod structures {
//     game_map_access!(objects::Structure, Game.structures);
// }

// /// See [http://docs.screeps.com/api/#Game.getObjectById]
// ///
// /// This gets an object expecting a specific type and will return a
// /// `ConversionError` if the type does not match.
// ///
// /// If all you want to assume is that something has an ID, use
// /// [`get_object_erased`].
// ///
// /// This uses the typed id type, [`ObjectId`]. Note that if you'd rather
// store /// an untyped ID, it's free to convert from [`RawObjectId`] to
// [`ObjectId`]. ///
// /// # Example
// ///
// /// ```no_run
// /// use screeps::{game, prelude::*, Creep, ObjectId};
// ///
// /// // get your id however
// /// let id: ObjectId<Creep> = "aaaa".parse().unwrap();
// ///
// /// let creep = game::get_object_typed(id).unwrap();
// /// match creep {
// ///     Some(creep) => println!("creep with id aaaa has name {}",
// creep.name()), ///     None => println!("no creep with id aaaa! such a
// surprise!"), /// }
// /// ```
// ///
// /// Or, using `RawObjectId`,
// ///
// /// ```no_run
// /// use screeps::{game, prelude::*, Creep, RawObjectId};
// ///
// /// let id: RawObjectId = "bbbb".parse().unwrap();
// ///
// /// let creep = game::get_object_typed::<Creep>(id.into()).unwrap();
// /// if let Some(creep) = creep {
// ///     println!("creep with id bbbb exists, and has name {}", creep.name());
// /// }
// /// ```
// ///
// /// [http://docs.screeps.com/api/#Game.getObjectById]: http://docs.screeps.com/api/#Game.getObjectById
// pub fn get_object_typed<T>(id: ObjectId<T>) -> Result<Option<T>,
// ConversionError> where
//     T: HasId + SizedRoomObject,
// {
//     let array_view = unsafe { id.unsafe_as_uploaded() };
//     (js! {
//         return Game.getObjectById(object_id_from_packed(@{array_view}));
//     })
//     .try_into()
// }

// /// See [http://docs.screeps.com/api/#Game.getObjectById]
// ///
// /// This gets the object in 'erased' form - all that is known about it is
// that /// it's a RoomObject.
// ///
// /// If a more specific type is expected, [`get_object_typed`] can be used.
// ///
// /// The ID passed in must be either an [`ObjectId`], or a [`RawObjectId`].
// Both /// work, and the type of [`ObjectId`] if passed will be ignored.
// ///
// /// [http://docs.screeps.com/api/#Game.getObjectById]: http://docs.screeps.com/api/#Game.getObjectById
// pub fn get_object_erased(id: impl Into<RawObjectId>) -> Option<RoomObject> {
//     let id = id.into();
//     let array_view = unsafe { id.unsafe_as_uploaded() };
//     js_unwrap_ref!(Game.getObjectById(object_id_from_packed(@{array_view})))
// }

// pub fn notify(message: &str, group_interval: Option<u32>) {
//     js! { @(no_return)
//         Game.notify(@{message}, @{group_interval.unwrap_or(0)});
//     }
// }
