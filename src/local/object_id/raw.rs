use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

use super::errors::RawObjectIdParseError;

const MAX_PACKED_VAL: u128 = (1 << (32 * 3)) - 1;

/// Represents an Object ID using a packed 12-byte representation
///
/// Each object id in screeps is represented by a Mongo GUID, which,
/// while not guaranteed, is unlikely to change. This takes advantage of that by
/// storing a packed representation.
///
/// To convert to a String in JavaScript, either use
/// [`RawObjectId::to_array_string`], or [`RawObjectId::unsafe_as_uploaded`].
/// See method documentation for more information.
///
/// # Ordering
///
/// To facilitate use as a key in a [`BTreeMap`] or other similar data
/// structures, `ObjectId` implements [`PartialOrd`] and [`Ord`].
///
/// `RawObjectId`'s are ordered by the corresponding order of their underlying
/// byte values. See [`ObjectId`] documentation for more information.
///
/// [`BTreeMap`]: std::collections::BTreeMap
/// [`Ord`]: std::cmp::Ord
/// [`PartialOrd`]: std::cmp::PartialOrd
/// [`ObjectId`]: super::ObjectId
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(transparent)]
pub struct RawObjectId {
    packed: u128,
}

impl fmt::Debug for RawObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawObjectId")
            .field("packed", &self.packed)
            .field("real", &self.to_string())
            .finish()
    }
}

impl fmt::Display for RawObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0width$x}",
            self.packed >> 32,
            width = (self.packed as u32) as usize
        )
    }
}

impl FromStr for RawObjectId {
    type Err = RawObjectIdParseError;

    fn from_str(s: &str) -> Result<Self, RawObjectIdParseError> {
        // get the actual integer value of the id, which we'll store in the most
        // significant 96 bits of the u128
        let u128_id = u128::from_str_radix(s, 16)?;
        // and the length, which we know can't be greater than 24 without going over
        // MAX_PACKED_VAL
        let pad_length = s.len() as u128;

        if u128_id > MAX_PACKED_VAL {
            return Err(RawObjectIdParseError::value_too_large(u128_id));
        }

        Ok(Self::from(u128_id << 32 | pad_length))
    }
}

// impl TryFrom<u128> for RawObjectId {
//     type Error = RawObjectIdParseError;

//     /// Creates an object ID from its binary representation as a `u128` number.
//     ///
//     /// # Errors
//     ///
//     /// This will error if the given value is greater than `2^96 - 1`, the
//     /// maximum number storable in a 96-bit integer.
//     fn try_from(val: u128) -> Result<Self, RawObjectIdParseError> {
//         if val > MAX_PACKED_VAL {
//             return Err(RawObjectIdParseError::value_too_large(val));
//         }

//         // if the endianness is right, then I think this should optimize down to a
//         // transmute. If it isn't, then it should be pretty efficient anyways and will
//         // still be _correct_.
//         let as_array = [
//             ((val >> 64) & 0xFFFF_FFFF) as u32,
//             ((val >> 32) & 0xFFFF_FFFF) as u32,
//             (val & 0xFFFF_FFFF) as u32,
//         ];
//         Ok(Self::from_packed(as_array))
//     }
// }

impl RawObjectId {
    /// Creates an object ID from its packed representation.
    ///
    /// The input to this function is the bytes representing the up-to-24 hex
    /// digits in the object id.
    pub fn from_packed(packed: u128) -> Self {
        RawObjectId { packed }
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
    // /// use screeps::{prelude::*, traits::TryInto, RawObjectId};
    // /// use stdweb::js;
    // ///
    // /// let packed_obj_id = (js! {
    // ///     let creep = _.sample(Game.creeps);
    // ///     return object_id_to_packed(creep.id);
    // /// })
    // /// .try_into()
    // /// .unwrap();
    // ///
    // /// let parsed = RawObjectId::from_packed_js_val(packed_obj_id).unwrap();
    // /// println!("found creep with id {}", parsed);
    // /// ```
    // pub fn from_packed_js_val(packed_val: Reference) -> Result<Self, ConversionError> {
    //     let mut packed = [0u32; 3];
    //     // TODO: make this more efficient, once we get mutable UnsafeTypedArrays.
    //     // See https://github.com/koute/stdweb/issues/360.
    //     packed[0] = js! {return @{&packed_val}[0]}.try_into()?;
    //     packed[1] = js! {return @{&packed_val}[1]}.try_into()?;
    //     packed[2] = js! {return @{&packed_val}[2]}.try_into()?;

    //     Ok(Self::from_packed(packed))
    // }

    // /// Converts this object ID to a `u128` number.
    // ///
    // /// The returned number, when formatted as hex, will produce a string
    // /// parseable into this object id.
    // ///
    // /// The returned number will be less than or equal to `2^96 - 1`, as that's
    // /// the maximum value that `RawObjectId` can hold.
    // pub fn to_u128(self) -> u128 {
    //     ((self.packed[0] as u128) << 64)
    //         | ((self.packed[1] as u128) << 32)
    //         | (self.packed[2] as u128)
    // }

    // /// Internal function which trims off leading zero integers.
    // fn non_zero_packed_ints(&self) -> &[u32] {
    //     for i in 0..3 {
    //         if self.packed[i] != 0 {
    //             return &self.packed[i..3];
    //         }
    //     }
    //     // fallback to static zero-sized slice if we have no non-zero integers...
    //     &[]
    // }

    // /// Formats this object ID as a string on the stack.
    // ///
    // /// This is equivalent to [`ToString::to_string`], but involves no
    // /// allocation.
    // ///
    // /// To use the produced string in stdweb, use `&*` to convert it to a string
    // /// slice.
    // ///
    // /// This is less efficient than [`RawObjectId::unsafe_as_uploaded`], but
    // /// easier to get right.
    // ///
    // /// # Example
    // ///
    // /// ```no_run
    // /// use screeps::{prelude::*, RawObjectId};
    // /// use stdweb::js;
    // ///
    // /// let object_id: RawObjectId = screeps::game::creeps::values()[0].untyped_id();
    // ///
    // /// let str_repr = object_id.to_array_string();
    // ///
    // /// js! {
    // ///     let id = @{&*str_repr};
    // ///     console.log("we have a creep with the id " + id);
    // /// }
    // /// ```
    // pub fn to_array_string(&self) -> ArrayString<[u8; 24]> {
    //     let mut res = ArrayString::new();
    //     write!(res, "{}", self).expect("expected formatting into a fixed-sized buffer to succeed");
    //     res
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
    // /// use screeps::{prelude::*, RawObjectId};
    // /// use stdweb::js;
    // ///
    // /// let object_id: RawObjectId = screeps::game::creeps::values()[0].untyped_id();
    // ///
    // /// let array_view = unsafe { object_id.unsafe_as_uploaded() };
    // ///
    // /// js! {
    // ///     let id = object_id_from_packed(@{array_view});
    // ///     console.log("we have a creep with the id " + id);
    // /// }
    // /// ```
    // pub unsafe fn unsafe_as_uploaded(&self) -> UnsafeTypedArray<'_, u32> {
    //     UnsafeTypedArray::new(self.non_zero_packed_ints())
    // }
}

// impl From<RawObjectId> for ArrayString<[u8; 24]> {
//     fn from(id: RawObjectId) -> Self {
//         id.to_array_string()
//     }
// }

impl From<RawObjectId> for String {
    fn from(id: RawObjectId) -> Self {
        id.to_string()
    }
}

impl From<RawObjectId> for u128 {
    fn from(id: RawObjectId) -> Self {
        id.packed
    }
}

// impl From<RawObjectId> for [u32; 3] {
//     fn from(id: RawObjectId) -> Self {
//         id.packed
//     }
// }

impl From<u128> for RawObjectId {
    fn from(packed: u128) -> Self {
        Self::from_packed(packed)
    }
}

#[cfg(test)]
mod test {
    use super::RawObjectId;

    // #[cfg(target_arch = "wasm32")]
    // use crate::macros::*;
    // use crate::traits::TryInto;

    const TEST_IDS: &[&str] = &[
        "0",
        "1",
        // max allowed ID
        "ffffffffffffffffffffffff",
        // 24 char, leading 0 (#278)
        "06aebab343040c9baaa22322",
        "000000000000000000000001",
        "100000000000000000000000",
        "10000000000000000",
        "1000000000000000",
        "bc03381d32f6790",
        // 15 char, leading 0 (#244)
        "0df4aea318bd552",
        "000000000000f00",
        "100000000",
        "10000000",
    ];

    #[test]
    fn rust_display_rust_fromstr_roundtrip() {
        for id in TEST_IDS {
            let parsed: RawObjectId = id.parse().unwrap();
            assert_eq!(&*parsed.to_string(), *id);
        }
    }

    // #[test]
    // fn rust_to_array_string_rust_fromstr_roundtrip() {
    //     for id in TEST_IDS {
    //         let parsed: RawObjectId = id.parse().unwrap();
    //         assert_eq!(&*parsed.to_array_string(), *id);
    //     }
    // }

    #[test]
    fn rust_to_u128_from_u128_roundtrip() {
        for id in TEST_IDS {
            let parsed: RawObjectId = id.parse().unwrap();
            let int = u128::from(parsed);
            let reparsed: RawObjectId = int.into();
            assert_eq!(parsed, reparsed);
            assert_eq!(reparsed.to_string(), *id);
        }
    }

    #[test]
    fn large_values_do_not_parse() {
        let large_ids = &[
            "1000000000000000000000000".to_owned(),
            format!("{:x}", u128::max_value()),
        ];
        for id in large_ids {
            let res: Result<RawObjectId, _> = id.parse();
            assert!(res.is_err());
        }
    }

    // #[test]
    // #[cfg(target_arch = "wasm32")]
    // fn js_format_rust_fromstr_roundtrip() {
    //     for id in TEST_IDS {
    //         let parsed: RawObjectId = id.parse().unwrap();
    //         let array_view = unsafe { parsed.unsafe_as_uploaded() };
    //         let js_produced_string: String = js_unwrap!(object_id_from_packed(@{array_view}));
    //         let reparsed: RawObjectId = js_produced_string
    //             .parse()
    //             .expect("expected to successfully reparse object id");
    //         assert_eq!(parsed, reparsed);
    //     }
    // }

    // #[test]
    // #[cfg(target_arch = "wasm32")]
    // fn rust_display_js_parse_roundtrip() {
    //     for id in TEST_IDS {
    //         let parsed: RawObjectId = id.parse().unwrap();
    //         let string = parsed.to_array_string();
    //         let js_produced_vals = js_unwrap!(object_id_to_packed(@{&*string}));
    //         let recreated = RawObjectId::from_packed_js_val(js_produced_vals).unwrap();
    //         assert_eq!(parsed, recreated);
    //     }
    // }
}
