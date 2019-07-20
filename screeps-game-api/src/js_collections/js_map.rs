//! [`JsMap`]
use std::{borrow::Borrow, collections::HashMap, marker::PhantomData};

use stdweb::{JsSerialize, Object, Value};

use {
    traits::{FromExpectedType, IntoExpectedType},
    ConversionError,
};

pub struct JsMap<K, V> {
    inner: Object,
    phantom: PhantomData<HashMap<K, V>>,
}

impl<K, V> JsMap<K, V> {
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V> JsMap<K, V>
where
    Option<V>: FromExpectedType<Value>,
{
    /// Gets an item, panicking if the types don't match and `check-all-casts` is enabled.
    pub fn get<'a, Q: ?Sized>(&self, key: &'a Q) -> Option<V>
    where
        K: Borrow<Q>,
        &'a Q: Into<Value>,
    {
        js_unwrap_ref!(@{self.inner.as_ref()}[@{key.into()}])
    }

    /// Gets an item, returning an error if the types don't match and `check-all-casts` is enabled.
    pub fn try_get<'a, Q: ?Sized>(&self, key: &'a Q) -> Option<Result<V, ConversionError>>
    where
        K: Borrow<Q>,
        &'a Q: Into<Value>,
    {
        (js! {
            return @{self.inner.as_ref()}[@{key.into()}].into_expected_type();
        })
        .into_expected_type()
        .transpose()
    }
}

// impl Index<Nucleotide> for NucleotideCount {
//     type Output = usize;

//     fn index(&self, nucleotide: Nucleotide) -> &usize {
//         match nucleotide {
//             Nucleotide::A => &self.a,
//             Nucleotide::C => &self.c,
//             Nucleotide::G => &self.g,
//             Nucleotide::T => &self.t,
//         }
//     }
// }
