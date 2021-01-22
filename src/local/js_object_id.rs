use std::{
    cmp::{Eq, PartialEq},
    fmt,
    marker::PhantomData,
};

use js_sys::JsString;
use wasm_bindgen::prelude::*;

use crate::game::Game;

/// Represents a reference to an Object ID string held on the javascript heap
/// and a type that the ID points to.
///
/// This representation is less useful on the Rust side due to lack of
/// visibility on the underlying string and lack of most trait implementations,
/// and consumes more memory, but is faster to resolve and may be useful with
/// objects you plan to resolve frequently.
///
/// This object ID is typed, but not strictly, and can be converted into
/// referring into another type of object with [`JsObjectId::into_type`].
// Copy, Clone, Debug, PartialEq, Eq, Hash, PartialEq, Eq implemented manually
// below
pub struct JsObjectId<T> {
    pub raw: JsString,
    phantom: PhantomData<T>,
}

// traits implemented manually so they don't depend on `T` implementing them.
impl<T> Clone for JsObjectId<T> {
    fn clone(&self) -> JsObjectId<T> {
        JsObjectId {
            raw: self.raw.clone(),
            phantom: PhantomData,
        }
    }
}
impl<T> fmt::Debug for JsObjectId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.raw.fmt(f)
    }
}
impl<T> PartialEq for JsObjectId<T> {
    fn eq(&self, o: &JsObjectId<T>) -> bool {
        self.raw.eq(&o.raw)
    }
}
impl<T> Eq for JsObjectId<T> {}

// impl<T> FromStr for JsObjectId<T> {
//     type Err = RawObjectIdParseError;

//     fn from_str(s: &str) -> Result<Self, RawObjectIdParseError> {
//         let raw: RawObjectId = s.parse()?;

//         Ok(raw.into())
//     }
// }

impl<T> fmt::Display for JsObjectId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        String::from(&self.raw).fmt(f)
    }
}

impl<T> JsObjectId<T> {
    /// Changes the type this [`JsObjectId`] points to, unchecked.
    ///
    /// This will allow changing to any type - `JsObjectId` makes no guarantees
    /// about its ID matching the type of any object in the game that it
    /// actually points to.
    pub fn into_type<U>(self) -> JsObjectId<U> {
        JsObjectId {
            raw: self.raw,
            phantom: PhantomData,
        }
    }

    // /// Resolves this object ID into an object.
    // ///
    // /// This is a shortcut for [`game::get_object_typed(id)`][1]
    // ///
    // /// # Errors
    // ///
    // /// Will return an error if this ID's type does not match the object it
    // /// points to.
    // ///
    // /// Will return `Ok(None)` if the object no longer exists, or is in a room
    // /// we don't have vision for.
    // ///
    // /// [1]: crate::game::get_object_typed
    // pub fn try_resolve(self) -> Result<Option<T>, ConversionError>
    // where
    //     T: HasId + SizedRoomObject,
    // {
    //     crate::game::get_object_typed(self)
    // }

    /// Resolves this ID into an object, assuming the type `T` is the correct
    /// type of object that this ID refers to. If the ID has been converted to
    /// an invalid type, using the returned object in a way not valid for its
    /// type will cause a panic.
    ///
    /// Will return `None` if this object no longer exists, or is in a room we
    /// don't have vision for.
    pub fn resolve(&self) -> Option<T>
    where
        T: From<JsValue>,
    {
        Game::get_object_by_js_id_typed(self)
    }
}

impl<T> From<JsString> for JsObjectId<T> {
    fn from(raw: JsString) -> Self {
        JsObjectId {
            raw,
            phantom: PhantomData,
        }
    }
}

impl<T> From<JsObjectId<T>> for JsString {
    fn from(id: JsObjectId<T>) -> Self {
        id.raw
    }
}

impl<T> From<JsObjectId<T>> for String {
    fn from(id: JsObjectId<T>) -> Self {
        id.to_string()
    }
}
