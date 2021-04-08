//! Get global Screeps resources.
//!
//! This contains all functionality from the [`Game`] object in Screeps. That
//! generally means all state which is true this tick throughout the world.
//!
//! [`Game`]: http://docs.screeps.com/api/#Game
use std::collections::HashMap;

use crate::{
    local::{ObjectId, RawObjectId},
    objects::{HasId, RoomObject, SizedRoomObject},
    traits::TryInto,
    ConversionError, ResourceType,
};

pub mod cpu;
pub mod gcl;
pub mod gpl;
pub mod map;
pub mod market;
pub mod shards;

/// See [http://docs.screeps.com/api/#Game.constructionSites]
///
/// [http://docs.screeps.com/api/#Game.constructionSites]: http://docs.screeps.com/api/#Game.constructionSites
pub mod construction_sites {
    game_map_access!(objects::ConstructionSite, Game.constructionSites);
}

/// See [http://docs.screeps.com/api/#Game.creeps]
///
/// [http://docs.screeps.com/api/#Game.creeps]: http://docs.screeps.com/api/#Game.creeps
pub mod creeps {
    game_map_access!(objects::Creep, Game.creeps);
}

/// See [http://docs.screeps.com/api/#Game.flags]
///
/// [http://docs.screeps.com/api/#Game.flags]: http://docs.screeps.com/api/#Game.flags
pub mod flags {
    game_map_access!(objects::Flag, Game.flags);
}

/// See [http://docs.screeps.com/api/#Game.powerCreeps]
///
/// [http://docs.screeps.com/api/#Game.powerCreeps]: http://docs.screeps.com/api/#Game.powerCreeps
pub mod power_creeps {
    game_map_access!(objects::AccountPowerCreep, Game.powerCreeps);
}

/// See [http://docs.screeps.com/api/#Game.resources]
///
/// [http://docs.screeps.com/api/#Game.resources]: http://docs.screeps.com/api/#Game.resources
pub mod resources {
    use std::collections::HashMap;

    use crate::constants::IntershardResourceType;

    /// Retrieve the full `HashMap<IntershardResourceType, u32>`.
    pub fn hashmap() -> HashMap<IntershardResourceType, u32> {
        // `TryFrom<Value>` is only implemented for `HashMap<String, V>`.
        //
        // See https://github.com/koute/stdweb/issues/359.
        let map: HashMap<String, u32> = js_unwrap!(Game.resources);
        map.into_iter()
            .map(|(key, val)| {
                (
                    key.parse()
                        .expect("expected resource key in Game.resources to be a known intershard resource type"),
                    val,
                )
            })
            .collect()
    }

    /// Retrieve the string keys of this object.
    pub fn keys() -> Vec<IntershardResourceType> {
        js_unwrap!(Object.keys(Game.resources).map(__resource_type_str_to_num))
    }

    /// Retrieve a specific value by key.
    pub fn get(key: IntershardResourceType) -> Option<u32> {
        js_unwrap!(Game.resources[__resource_type_num_to_str(@{key as u32})])
    }
}

/// See [http://docs.screeps.com/api/#Game.rooms]
///
/// [http://docs.screeps.com/api/#Game.rooms]: http://docs.screeps.com/api/#Game.rooms
pub mod rooms {
    use std::collections::HashMap;

    use crate::{local::RoomName, objects::Room};

    /// Retrieve the full `HashMap<RoomName, Room>`.
    pub fn hashmap() -> HashMap<RoomName, Room> {
        // `TryFrom<Value>` is only implemented for `HashMap<String, V>`.
        //
        // See https://github.com/koute/stdweb/issues/359.
        let map: HashMap<String, Room> = js_unwrap!(Game.rooms);
        map.into_iter()
            .map(|(key, val)| {
                (
                    key.parse()
                        .expect("expected room name in Game.rooms to be valid"),
                    val,
                )
            })
            .collect()
    }

    /// Retrieve the string keys of this object.
    pub fn keys() -> Vec<RoomName> {
        js_unwrap!(Object.keys(Game.rooms))
    }

    /// Retrieve all values in this object.
    pub fn values() -> Vec<Room> {
        js_unwrap_ref!(Object.values(Game.rooms))
    }

    /// Retrieve a specific value by key.
    pub fn get(name: RoomName) -> Option<Room> {
        js_unwrap_ref!(Game.rooms[@{name}])
    }
}

/// See [http://docs.screeps.com/api/#Game.spawns]
///
/// [http://docs.screeps.com/api/#Game.spawns]: http://docs.screeps.com/api/#Game.spawns
pub mod spawns {
    game_map_access!(objects::StructureSpawn, Game.spawns);
}

/// See [http://docs.screeps.com/api/#Game.structures]
///
/// [http://docs.screeps.com/api/#Game.structures]: http://docs.screeps.com/api/#Game.structures
pub mod structures {
    game_map_access!(objects::Structure, Game.structures);
}

/// See [http://docs.screeps.com/api/#Game.time]
///
/// [http://docs.screeps.com/api/#Game.time]: http://docs.screeps.com/api/#Game.time
pub fn time() -> u32 {
    js_unwrap!(Game.time)
}

/// Your current score, as determined by the symbols you have decoded.
///
/// See [https://docs-season.screeps.com/api/#Game.score]
///
/// [https://docs-season.screeps.com/api/#Game.score]: https://docs-season.screeps.com/api/#Game.score
#[cfg(feature = "enable-symbols")]
pub fn score() -> u32 {
    js_unwrap!(Game.score)
}

/// The symbols you've decoded after multiplier adjustments, used to determine
/// your score.
///
/// See [https://docs-season.screeps.com/api/#Game.symbols]
///
/// [https://docs-season.screeps.com/api/#Game.symbols]: https://docs-season.screeps.com/api/#Game.symbols
#[cfg(feature = "enable-symbols")]
pub fn symbols() -> HashMap<ResourceType, u32> {
    // `TryFrom<Value>` is only implemented for `HashMap<String, V>`.
    //
    // See https://github.com/koute/stdweb/issues/359.
    let map: HashMap<String, u32> = js_unwrap!(Game.symbols);
    map.into_iter()
        .map(|(key, val)| {
            (
                key.parse()
                    .expect("expected resource key in Game.symbols to be a known resource type"),
                val,
            )
        })
        .collect()
}

/// See [http://docs.screeps.com/api/#Game.getObjectById]
///
/// This gets an object expecting a specific type and will return a
/// `ConversionError` if the type does not match.
///
/// If all you want to assume is that something has an ID, use
/// [`get_object_erased`].
///
/// This uses the typed id type, [`ObjectId`]. Note that if you'd rather store
/// an untyped ID, it's free to convert from [`RawObjectId`] to [`ObjectId`].
///
/// # Example
///
/// ```no_run
/// use screeps::{game, prelude::*, Creep, ObjectId};
///
/// // get your id however
/// let id: ObjectId<Creep> = "aaaa".parse().unwrap();
///
/// let creep = game::get_object_typed(id).unwrap();
/// match creep {
///     Some(creep) => println!("creep with id aaaa has name {}", creep.name()),
///     None => println!("no creep with id aaaa! such a surprise!"),
/// }
/// ```
///
/// Or, using `RawObjectId`,
///
/// ```no_run
/// use screeps::{game, prelude::*, Creep, RawObjectId};
///
/// let id: RawObjectId = "bbbb".parse().unwrap();
///
/// let creep = game::get_object_typed::<Creep>(id.into()).unwrap();
/// if let Some(creep) = creep {
///     println!("creep with id bbbb exists, and has name {}", creep.name());
/// }
/// ```
///
/// [http://docs.screeps.com/api/#Game.getObjectById]: http://docs.screeps.com/api/#Game.getObjectById
pub fn get_object_typed<T>(id: ObjectId<T>) -> Result<Option<T>, ConversionError>
where
    T: HasId + SizedRoomObject,
{
    let array_view = unsafe { id.unsafe_as_uploaded() };
    (js! {
        return Game.getObjectById(object_id_from_packed(@{array_view}));
    })
    .try_into()
}

/// See [http://docs.screeps.com/api/#Game.getObjectById]
///
/// This gets the object in 'erased' form - all that is known about it is that
/// it's a RoomObject.
///
/// If a more specific type is expected, [`get_object_typed`] can be used.
///
/// The ID passed in must be either an [`ObjectId`], or a [`RawObjectId`]. Both
/// work, and the type of [`ObjectId`] if passed will be ignored.
///
/// [http://docs.screeps.com/api/#Game.getObjectById]: http://docs.screeps.com/api/#Game.getObjectById
pub fn get_object_erased(id: impl Into<RawObjectId>) -> Option<RoomObject> {
    let id = id.into();
    let array_view = unsafe { id.unsafe_as_uploaded() };
    js_unwrap_ref!(Game.getObjectById(object_id_from_packed(@{array_view})))
}

pub fn notify(message: &str, group_interval: Option<u32>) {
    js! { @(no_return)
        Game.notify(@{message}, @{group_interval.unwrap_or(0)});
    }
}
