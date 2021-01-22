use std::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::game::Game;

// use crate::{
//     objects::{HasId, SizedRoomObject},
//     traits::{TryFrom, TryInto},
//     ConversionError,
// };

mod errors;
mod raw;

pub use errors::*;
pub use raw::*;

/// Represents an Object ID and a type that the ID points to.
///
/// Each object id in screeps is represented by a Mongo GUID, which,
/// while not guaranteed, is unlikely to change. This takes advantage of that by
/// storing a packed representation of 12 bytes.
///
/// This object ID is typed, but not strictly. It's completely safe to create an
/// ObjectId with an incorrect type, and all operations which use the type will
/// double-check at runtime.
///
/// With that said, using this can provide nice type inference, and should have
/// few disadvantages to the lower-level alternative, [`RawObjectId`].
///
/// # Conversion
///
/// Use `into` to convert between `ObjectId<T>` and [`RawObjectId`], and
/// [`ObjectId::into_type`] to change the type this `ObjectId` points to freely.
///
/// # Ordering
///
/// To facilitate use as a key in a [`BTreeMap`] or other similar data
/// structures, `ObjectId` implements [`PartialOrd`] and [`Ord`].
///
/// `ObjectId`'s are ordered by the corresponding order of their underlying
/// byte values. This agrees with:
///
/// - lexicographical ordering of the object id strings
/// - JavaScript's ordering of object id strings
/// - ordering of [`RawObjectId`]s
///
/// **Note:** when running on the official screeps server, or on a private
/// server backed by a MongoDB database, this ordering roughly corresponds to
/// creation order. The first four bytes of a MongoDB-created `ObjectId` [are
/// seconds since the epoch when the id was created][1], so up to a second
/// accuracy, these ids will be sorted by object creation time.
///
/// [`BTreeMap`]: std::collections::BTreeMap
/// [1]: https://docs.mongodb.com/manual/reference/method/ObjectId/
// Copy, Clone, Debug, PartialEq, Eq, Hash, PartialEq, Eq implemented manually below
#[derive(Serialize, Deserialize)]
#[serde(transparent, bound = "")]
pub struct ObjectId<T> {
    raw: RawObjectId,
    #[serde(skip)]
    phantom: PhantomData<T>,
}

// traits implemented manually so they don't depend on `T` implementing them.
impl<T> Copy for ObjectId<T> {}
impl<T> Clone for ObjectId<T> {
    fn clone(&self) -> ObjectId<T> {
        ObjectId {
            raw: self.raw.clone(),
            phantom: PhantomData,
        }
    }
}
impl<T> fmt::Debug for ObjectId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.raw.fmt(f)
    }
}
impl<T> PartialEq for ObjectId<T> {
    fn eq(&self, o: &ObjectId<T>) -> bool {
        self.raw.eq(&o.raw)
    }
}
impl<T> Eq for ObjectId<T> {}
impl<T> Hash for ObjectId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state)
    }
}
impl<T> PartialOrd<ObjectId<T>> for ObjectId<T> {
    #[inline]
    fn partial_cmp(&self, other: &ObjectId<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for ObjectId<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.raw.cmp(&other.raw)
    }
}

impl<T> FromStr for ObjectId<T> {
    type Err = RawObjectIdParseError;

    fn from_str(s: &str) -> Result<Self, RawObjectIdParseError> {
        let raw: RawObjectId = s.parse()?;

        Ok(raw.into())
    }
}

// impl<T> TryFrom<u128> for ObjectId<T> {
//     type Error = RawObjectIdParseError;

//     fn try_from(val: u128) -> Result<Self, RawObjectIdParseError> {
//         let raw: RawObjectId = val.try_into()?;

//         Ok(raw.into())
//     }
// }

impl<T> fmt::Display for ObjectId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.raw.fmt(f)
    }
}

impl<T> ObjectId<T> {
    /// Changes the type this [`ObjectId`] points to, unchecked.
    ///
    /// This will allow changing to any type - `ObjectId` makes no guarantees
    /// about its ID matching the type of any object in the game that it
    /// actually points to.
    pub fn into_type<U>(self) -> ObjectId<U> {
        RawObjectId::from(self).into()
    }

    /// Creates an object ID from its packed representation.
    ///
    /// The input to this function is the bytes representing the up-to-24 hex
    /// digits in the object id.
    ///
    /// See also [`RawObjectId::from_packed`].
    pub fn from_packed(packed: u128) -> Self {
        RawObjectId::from_packed(packed).into()
    }

    // /// Creates an object ID from a packed representation stored in JavaScript.
    // ///
    // /// The input must be a reference to a length-3 array of integers.
    // ///
    // /// Recommended to be used with the `object_id_to_packed` JavaScript utility
    // /// function, which takes in a string and returns the array of three
    // /// integers that this function expects.
    // ///
    // /// # Example
    // ///
    // /// ```no_run
    // /// use screeps::{prelude::*, traits::TryInto, Creep, ObjectId};
    // /// use stdweb::js;
    // ///
    // /// let packed_obj_id = (js! {
    // ///     let creep = _.sample(Game.creeps);
    // ///     return object_id_to_packed(creep.id);
    // /// })
    // /// .try_into()
    // /// .unwrap();
    // ///
    // /// let parsed: ObjectId<Creep> = ObjectId::from_packed_js_val(packed_obj_id).unwrap();
    // /// println!("found creep with id {}", parsed);
    // /// ```
    // ///
    // /// See also [`RawObjectId::from_packed_js_val`].
    // pub fn from_packed_js_val(packed_val: Reference) -> Result<Self, ConversionError> {
    //     RawObjectId::from_packed_js_val(packed_val).map(Into::into)
    // }

    /// Converts this object ID to a `u128` number.
    ///
    /// The returned number, when formatted as hex, will produce a string
    /// parseable into this object id.
    ///
    /// The returned number will be less than or equal to `2^96 - 1`, as that's
    /// the maximum value that `RawObjectId` can hold.
    pub fn to_u128(self) -> u128 {
        self.raw.into()
    }

    // /// Formats this object ID as a string on the stack.
    // ///
    // /// This is equivalent to [`ToString::to_string`], but involves no
    // /// allocation.
    // ///
    // /// To use the produced string in stdweb, use `&*` to convert it to a string
    // /// slice.
    // ///
    // /// This is less efficient than [`ObjectId::unsafe_as_uploaded`], but
    // /// easier to get right.
    // ///
    // /// # Example
    // ///
    // /// ```no_run
    // /// use screeps::{prelude::*, Creep, ObjectId};
    // /// use stdweb::js;
    // ///
    // /// let object_id = screeps::game::creeps::values()[0].id();
    // ///
    // /// let str_repr = object_id.to_array_string();
    // ///
    // /// js! {
    // ///     let id = @{&*str_repr};
    // ///     console.log("we have a creep with the id " + id);
    // /// }
    // /// ```
    // ///
    // /// See also [`RawObjectId::to_array_string`].
    // pub fn to_array_string(&self) -> ArrayString<[u8; 24]> {
    //     self.raw.to_array_string()
    // }

    // /// Creates an array accessible from JavaScript which represents part of
    // /// this object id's packed representation.
    // ///
    // /// Specifically, the resulting array will contain the first non-zero number
    // /// in this object id, and all following numbers. This allows for a more
    // /// efficient `object_id_from_packed` implementation.
    // ///
    // /// # Safety
    // ///
    // /// This is highly unsafe.
    // ///
    // /// This creates an `UnsafeTypedArray` and does not use it in JS, so the
    // /// restrictions from [`UnsafeTypedArray`] apply. When you call into
    // /// JavaScript using it, you must "use" it immediately before calling into
    // /// any Rust code whatsoever.
    // ///
    // /// There are other safety concerns as well, but all deriving from
    // /// [`UnsafeTypedArray`]. See [`UnsafeTypedArray`].
    // ///
    // /// # Example
    // ///
    // /// ```no_run
    // /// use screeps::{prelude::*, Creep, ObjectId};
    // /// use stdweb::js;
    // ///
    // /// let object_id = screeps::game::creeps::values()[0].id();
    // ///
    // /// let array_view = unsafe { object_id.unsafe_as_uploaded() };
    // ///
    // /// js! {
    // ///     let id = object_id_from_packed(@{array_view});
    // ///     console.log("we have a creep with the id " + id);
    // /// }
    // /// ```
    // ///
    // /// See also [`RawObjectId::unsafe_as_uploaded`].
    // pub unsafe fn unsafe_as_uploaded(&self) -> UnsafeTypedArray<'_, u32> {
    //     self.raw.unsafe_as_uploaded()
    // }

    /// Resolves this ID into an object, assuming the type `T` is the correct
    /// type of object that this ID refers to. If the ID has been converted to
    /// an invalid type, using the returned object in a way not valid for its
    /// type will cause a panic.
    ///
    /// Will return `None` if this object no longer exists, or is in a room we
    /// don't have vision for.
    pub fn resolve(self) -> Option<T>
    where
        T: From<JsValue>,
    {
        Game::get_object_by_id_typed(&self)
    }

    // /// Resolves this ID into an object, panicking on type mismatch.
    // ///
    // /// This is a shortcut for [`id.try_resolve().expect(...)`][1]
    // ///
    // /// # Panics
    // ///
    // /// Will panic if this ID points to an object which is not of type `T`.
    // ///
    // /// Will return `None` if this object no longer exists, or is in a room we
    // /// don't have vision for.
    // ///
    // /// [1]: ObjectId::try_resolve
    // pub fn resolve(self) -> Option<T>
    // where
    //     T: HasId + SizedRoomObject,
    // {
    //     match self.try_resolve() {
    //         Ok(v) => v,
    //         Err(e) => panic!("error resolving id {}: {}", self, e),
    //     }
    // }
}

impl<T> PartialEq<RawObjectId> for ObjectId<T> {
    #[inline]
    fn eq(&self, other: &RawObjectId) -> bool {
        self.raw == *other
    }
}

impl<T> PartialEq<ObjectId<T>> for RawObjectId {
    #[inline]
    fn eq(&self, other: &ObjectId<T>) -> bool {
        *self == other.raw
    }
}

impl<T> PartialOrd<RawObjectId> for ObjectId<T> {
    #[inline]
    fn partial_cmp(&self, other: &RawObjectId) -> Option<Ordering> {
        Some(self.raw.cmp(other))
    }
}

impl<T> PartialOrd<ObjectId<T>> for RawObjectId {
    #[inline]
    fn partial_cmp(&self, other: &ObjectId<T>) -> Option<Ordering> {
        Some(self.cmp(&other.raw))
    }
}

impl<T> From<RawObjectId> for ObjectId<T> {
    fn from(raw: RawObjectId) -> Self {
        ObjectId {
            raw,
            phantom: PhantomData,
        }
    }
}

impl<T> From<ObjectId<T>> for RawObjectId {
    fn from(id: ObjectId<T>) -> Self {
        id.raw
    }
}

// impl<T> From<ObjectId<T>> for ArrayString<[u8; 24]> {
//     fn from(id: ObjectId<T>) -> Self {
//         id.to_array_string()
//     }
// }

impl<T> From<ObjectId<T>> for String {
    fn from(id: ObjectId<T>) -> Self {
        id.to_string()
    }
}

impl<T> From<ObjectId<T>> for u128 {
    fn from(id: ObjectId<T>) -> Self {
        id.raw.into()
    }
}

impl<T> From<u128> for ObjectId<T> {
    fn from(packed: u128) -> Self {
        Self::from_packed(packed)
    }
}

// impl<T> From<[u32; 3]> for ObjectId<T> {
//     fn from(packed: [u32; 3]) -> Self {
//         Self::from_packed(packed)
//     }
// }

// impl<T> From<ObjectId<T>> for [u32; 3] {
//     fn from(id: ObjectId<T>) -> Self {
//         id.raw.into()
//     }
// }
