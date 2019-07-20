//! [`JsVec`]
use crate::{
    macros::*,
    traits::{FromExpectedType, IntoExpectedType, TryFrom, TryInto},
    ConversionError,
};
use std::marker::PhantomData;
use stdweb::{Array, InstanceOf, JsSerialize, Reference, ReferenceType, Value};

//
//   - InstanceOf
//   - AsRef<Reference>
//   - ReferenceType
//   - Into<Reference>
//   - TryInto<Reference>
//   - TryFrom<Reference>
//   - TryFrom<&Reference>
//   - TryFrom<Value>
//   - TryFrom<&Value>

/// Reference to a JavaScript array which is expected to contain a specific type
/// of item.
///
/// All `TryFrom` / `TryInto` conversions use type checking, but
/// `FromExpectedType` / `IntoExpectedType` possibly don't.
///
/// The implementation for `Into<Vec<T>>` uses `IntoExpectedType` internally.
///
/// Trait notes: implements `TryInto<Vec<T>>`, but `Vec<T>` does not implement
/// `TryFrom<JsVec<T>>` due to coherence issues.
pub struct JsVec<T> {
    inner: Array,
    phantom: PhantomData<Vec<T>>,
}

impl<T> JsVec<T> {
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> JsVec<T>
where
    T: FromExpectedType<Value>,
{
    /// Gets an item, panicking if the types don't match and `check-all-casts`
    /// is enabled.
    pub fn get(&self, idx: usize) -> Option<T> {
        // this assumes u32::max_value() == usize::max_value()
        // (otherwise cast below could overflow).
        if idx >= self.len() {
            None
        } else {
            Some(js_unwrap_ref!(@{self.inner.as_ref()}[@{idx as u32}]))
        }
    }

    /// Gets an item, returning an error if the types don't match and
    /// `check-all-casts` is enabled.
    pub fn try_get(&self, idx: usize) -> Result<Option<T>, ConversionError> {
        // this assumes u32::max_value() == usize::max_value()
        // (otherwise cast below could overflow).
        if idx >= self.len() {
            Ok(None)
        } else {
            (js! {
                return @{self.inner.as_ref()}[@{idx as u32}];
            })
            .into_expected_type()
            .map(Some)
        }
    }

    /// Iterates over elements, panicking if any are not the expected type and
    /// `check-all-casts` is enabled.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self)
    }

    /// Iterates over elements, returning an error if any are not the expected
    /// type and `check-all-casts` is enabled.
    ///
    /// Use [`IntoIterator::into_iter`] to iterate expecting all types match.
    pub fn try_iter(&self) -> TryIter<'_, T> {
        TryIter::new(self)
    }

    /// Iterates over elements, consuming self and returning a result if any are
    /// not the expected type.
    ///
    /// Use [`IntoIterator::into_iter`] to iterate expecting all types match.
    pub fn try_into_iter(self) -> TryIntoIter<T> {
        TryIntoIter::new(self)
    }

    /// Turns this remote JS array into a local `Vec`, returning an error if any
    /// elements are not the expected type and `check-all-casts` is enabled.
    pub fn try_local(&self) -> Result<Vec<T>, ConversionError> {
        self.try_iter().collect()
    }

    /// Turns this remote JS array into a local `Vec`, panicking if any elements
    /// are not the expected type and `check-all-casts` is enabled.
    pub fn local(&self) -> Vec<T> {
        self.iter().collect()
    }
}

pub struct IntoIter<T> {
    index: u32,
    inner: JsVec<T>,
}

impl<T> IntoIter<T> {
    pub fn new(vec: JsVec<T>) -> Self {
        IntoIter {
            inner: vec,
            index: 0,
        }
    }
}

pub struct TryIntoIter<T> {
    index: u32,
    inner: JsVec<T>,
}

impl<T> TryIntoIter<T> {
    pub fn new(vec: JsVec<T>) -> Self {
        TryIntoIter {
            inner: vec,
            index: 0,
        }
    }
}

pub struct Iter<'a, T> {
    index: u32,
    inner: &'a JsVec<T>,
}

impl<'a, T> Iter<'a, T> {
    pub fn new(vec: &'a JsVec<T>) -> Self {
        Iter {
            inner: vec,
            index: 0,
        }
    }
}

pub struct TryIter<'a, T> {
    index: u32,
    inner: &'a JsVec<T>,
}

impl<'a, T> TryIter<'a, T> {
    pub fn new(vec: &'a JsVec<T>) -> Self {
        TryIter {
            inner: vec,
            index: 0,
        }
    }
}

impl_js_vec_iterators_from_expected_type_panic!(Iter<'a>, IntoIter);
impl_js_vec_iterators_from_expected_type_with_result!(TryIter<'a>, TryIntoIter);

impl<T> IntoIterator for JsVec<T>
where
    T: FromExpectedType<Value>,
{
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<'a, T> IntoIterator for &'a JsVec<T>
where
    T: FromExpectedType<Value>,
{
    type Item = T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

impl<T> AsRef<Array> for JsVec<T> {
    fn as_ref(&self) -> &Array {
        &self.inner
    }
}

impl<T> From<JsVec<T>> for Array {
    fn from(jsv: JsVec<T>) -> Array {
        jsv.inner
    }
}

impl<T> TryFrom<JsVec<T>> for Array {
    type Error = ConversionError;

    fn try_from(jsv: JsVec<T>) -> Result<Array, Self::Error> {
        Ok(jsv.into())
    }
}

impl<T> From<JsVec<T>> for Reference {
    fn from(jsv: JsVec<T>) -> Reference {
        jsv.inner.into()
    }
}

impl<T> TryFrom<JsVec<T>> for Reference {
    type Error = ConversionError;

    fn try_from(jsv: JsVec<T>) -> Result<Reference, Self::Error> {
        Ok(jsv.into())
    }
}

impl<T> InstanceOf for JsVec<T>
where
    T: InstanceOf,
{
    fn instance_of(reference: &Reference) -> bool {
        (js! {
            let arr = @{reference};
            if (!(arr instanceof Array)) {
                return false;
            }
            for (let item of arr) {
                if (!(@{|r: Reference| T::instance_of(&r)}(item))) {
                    return false;
                }
            }
            return true;
        })
        .try_into()
        .expect("expected JsVec instance_of js code returning a bool to return a bool")
    }
}

impl<T> TryFrom<Array> for JsVec<T>
where
    T: InstanceOf,
{
    type Error = ConversionError;

    fn try_from(arr: Array) -> Result<JsVec<T>, Self::Error> {
        if arr.len() == 0 {
            return Ok(JsVec {
                inner: arr,
                phantom: PhantomData,
            });
        }

        // Type check array elements
        if !Self::instance_of(arr.as_ref()) {
            return Err(ConversionError::Custom(
                "reference is of a different type".into(),
            ));
        }
        Ok(JsVec {
            inner: arr,
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<Reference> for JsVec<T>
where
    T: InstanceOf,
{
    type Error = ConversionError;

    fn try_from(r: Reference) -> Result<JsVec<T>, Self::Error> {
        let arr: Array = r.try_into()?;
        arr.try_into()
    }
}

impl<T> TryFrom<Value> for JsVec<T>
where
    T: InstanceOf,
{
    type Error = ConversionError;

    fn try_from(r: Value) -> Result<JsVec<T>, Self::Error> {
        let arr: Array = r.try_into()?;
        arr.try_into()
    }
}

impl<T> TryInto<Vec<T>> for JsVec<T>
where
    T: TryFrom<Value, Error = ConversionError>,
{
    type Error = ConversionError;

    fn try_into(self) -> Result<Vec<T>, Self::Error> {
        self.inner.try_into()
    }
}

impl<T> AsRef<Reference> for JsVec<T> {
    fn as_ref(&self) -> &Reference {
        self.inner.as_ref()
    }
}

impl<T> FromExpectedType<Array> for JsVec<T>
where
    T: InstanceOf,
{
    fn from_expected_type(arr: Array) -> Result<Self, ConversionError> {
        #[cfg(feature = "check-all-casts")]
        {
            arr.try_into()
        }
        #[cfg(not(feature = "check-all-casts"))]
        {
            Ok(JsVec {
                inner: arr,
                phantom: PhantomData,
            })
        }
    }
}

impl<T> ReferenceType for JsVec<T>
where
    T: InstanceOf,
{
    unsafe fn from_reference_unchecked(reference: Reference) -> Self {
        JsVec {
            inner: Array::from_reference_unchecked(reference),
            phantom: PhantomData,
        }
    }
}

impl<T> FromExpectedType<Reference> for JsVec<T>
where
    T: InstanceOf,
{
    fn from_expected_type(r: Reference) -> Result<Self, ConversionError> {
        #[cfg(feature = "check-all-casts")]
        {
            r.try_into()
        }
        #[cfg(not(feature = "check-all-casts"))]
        {
            Ok(unsafe { Self::from_reference_unchecked(r) })
        }
    }
}

impl<T> From<&'_ [T]> for JsVec<T>
where
    T: JsSerialize,
{
    fn from(v: &[T]) -> Self {
        JsVec {
            inner: v.into(),
            phantom: PhantomData,
        }
    }
}

impl<T> From<&'_ mut [T]> for JsVec<T>
where
    T: JsSerialize,
{
    fn from(v: &mut [T]) -> Self {
        (&*v).into()
    }
}

impl<T> From<&'_ Vec<T>> for JsVec<T>
where
    T: JsSerialize,
{
    fn from(v: &Vec<T>) -> Self {
        (&**v).into()
    }
}

impl<T> From<&'_ mut Vec<T>> for JsVec<T>
where
    T: JsSerialize,
{
    fn from(v: &mut Vec<T>) -> Self {
        (&**v).into()
    }
}

impl<T> From<Vec<T>> for JsVec<T>
where
    T: JsSerialize,
{
    fn from(v: Vec<T>) -> Self {
        (&*v).into()
    }
}
