use std::collections::HashMap;
use std::convert::TryInto;
use std::iter::IntoIterator;
use std::ops::{Index, IndexMut};

use crate::objects::CostMatrix;

use super::Position;

#[derive(Clone, Debug)]
pub struct LocalCostMatrix {
    bits: [u8; 2500],
}

#[inline]
fn pos_as_idx(x: u8, y: u8) -> usize {
    (x as usize) * 50 + (y as usize)
}

#[inline]
fn idx_as_pos(idx: usize) -> (u8, u8) {
    ((idx / 50) as u8, (idx % 50) as u8)
}

impl Default for LocalCostMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalCostMatrix {
    #[inline]
    pub fn new() -> Self {
        LocalCostMatrix {
            bits: [0; 2500],
        }
    }

    #[inline]
    pub fn set(&mut self, x: u8, y: u8, val: u8) {
        self[(x, y)] = val;
    }

    #[inline]
    pub fn get(&self, x: u8, y: u8) -> u8 {
        self[(x, y)]
    }

    // # Safety
    // Calling this method with x >= 50 or y >= 50 is undefined behaviour.
    #[inline]
    pub unsafe fn get_unchecked(&self, x: u8, y: u8) -> u8 {
        debug_assert!(x < 50, "out of bounds x: {}", x);
        debug_assert!(y < 50, "out of bounds y: {}", y);
        *self.bits.get_unchecked(pos_as_idx(x,y))
    }

    // # Safety
    // Calling this method with x >= 50 or y >= 50 is undefined behaviour.
    #[inline]
    pub unsafe fn set_unchecked(&mut self, x: u8, y: u8, val: u8) {
        debug_assert!(x < 50, "out of bounds x: {}", x);
        debug_assert!(y < 50, "out of bounds y: {}", y);
        *self.bits.get_unchecked_mut(pos_as_idx(x, y)) = val;
    }

    pub fn get_bits(&self) -> &[u8; 2500] {
        &self.bits
    }

    pub fn iter(&self) -> impl Iterator<Item = ((u8, u8), &u8)> {
        self.bits.iter().enumerate().map(|(idx, val)| { (idx_as_pos(idx), val) })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = ((u8, u8), &mut u8)> {
        self.bits.iter_mut().enumerate().map(|(idx, val)| { (idx_as_pos(idx), val) })
    }

    // Takes all non-zero entries in `src`, and inserts them into `self`.
    //
    // If an entry for that position exists already, overwrites it with the new
    // value.
    pub fn merge_from_dense(&mut self, src: &LocalCostMatrix) {
        for i in 0..2500 {
            let val = unsafe { *src.bits.get_unchecked(i) };
            if val > 0 {
                unsafe { *self.bits.get_unchecked_mut(i) = val; }
            }
        }
    }

    // Takes all entries in `src` and merges them into `self`.
    //
    // If an entry for that position exists already, overwrites it with the new
    // value.
    pub fn merge_from_sparse(&mut self, src: &SparseCostMatrix) {
        for (pos, val) in src.iter() {
            unsafe { *self.bits.get_unchecked_mut(pos_as_idx(pos.0, pos.1)) = *val; }
        }
    }
}

impl From<LocalCostMatrix> for Vec<u8> {
    /// Returns a vector of bits length 2500, where each position is
    /// `idx = ((x * 50) + y)`.
    #[inline]
    fn from(lcm: LocalCostMatrix) -> Vec<u8> {
        lcm.bits.into()
    }
}

impl From<CostMatrix> for LocalCostMatrix {
    fn from(js_matrix: CostMatrix) -> Self {
        let array = js_matrix.get_bits();

        // SAFETY: CostMatrix is always 2500 long.
        LocalCostMatrix {
            bits: array.to_vec().try_into().expect("JS CostMatrix was not length 2500."),
        }
    }
}

impl Index<(u8, u8)> for LocalCostMatrix {
    type Output = u8;

    fn index(&self, idx: (u8, u8)) -> &Self::Output {
        assert!(idx.0 < 50, "out of bounds x: {}", idx.0);
        assert!(idx.1 < 50, "out of bounds y: {}", idx.1);
        // SAFETY: Just did bounds checking above.
        unsafe { self.bits.get_unchecked(pos_as_idx(idx.0, idx.1)) }
    }
}

impl IndexMut<(u8, u8)> for LocalCostMatrix {
    fn index_mut(&mut self, idx: (u8, u8)) -> &mut Self::Output {
        assert!(idx.0 < 50, "out of bounds x: {}", idx.0);
        assert!(idx.1 < 50, "out of bounds y: {}", idx.1);
        // SAFETY: Just did bounds checking above.
        unsafe { self.bits.get_unchecked_mut(pos_as_idx(idx.0, idx.1)) }
    }
}

impl Index<Position> for LocalCostMatrix {
    type Output = u8;

    fn index(&self,  idx: Position) -> &Self::Output {
        // SAFETY: Position always gives a valid in-room coordinate.
        unsafe { self.bits.get_unchecked(pos_as_idx(idx.x(), idx.y())) }
    }
}

impl IndexMut<Position> for LocalCostMatrix {
    fn index_mut(&mut self, idx: Position) -> &mut Self::Output {
        // SAFETY: Position always gives a valid in-room coordinate.
        unsafe { self.bits.get_unchecked_mut(pos_as_idx(idx.x(), idx.y())) }
    }
}

#[derive(Clone, Debug)]
pub struct SparseCostMatrix {
    inner: HashMap<(u8, u8), u8>
}

impl Default for SparseCostMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl SparseCostMatrix {
    pub fn new() -> Self {
        SparseCostMatrix { inner: HashMap::new() }
    }

    pub fn get(&self, x: u8, y: u8) -> u8 {
        assert!(x < 50, "out of bounds x: {}", x);
        assert!(y < 50, "out of bounds y: {}", y);
        if let Some(ref_val) = self.inner.get(&(x, y)) {
            *ref_val
        } else {
            0
        }
    }

    pub fn set(&mut self, x: u8, y: u8, val: u8) {
        assert!(x < 50, "out of bounds x: {}", x);
        assert!(y < 50, "out of bounds y: {}", y);
        self.inner.insert((x, y), val);
    }

    pub fn iter(&self) -> impl Iterator<Item = ((u8, u8), &u8)> {
        self.inner.iter().map(|(pos, val)| { (*pos, val) })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = ((u8, u8), &mut u8)> {
        self.inner.iter_mut().map(|(pos, val)| { (*pos, val) })
    }

    // Takes all non-zero entries in `src`, and inserts them into `self`.
    //
    // If an entry for that position exists already, overwrites it with the new
    // value.
    pub fn merge_from_dense(&mut self, src: &LocalCostMatrix) {
        self.inner.extend(src.iter().filter_map(|(xy, val)| {
            if *val > 0 {
                Some((xy, *val))
            } else {
                None
            }
        }))
    }

    // Takes all entries in `src` and merges them into `self`.
    //
    // If an entry for that position exists already, overwrites it with the new
    // value.
    pub fn merge_from_sparse(&mut self, src: &SparseCostMatrix) {
        self.inner.extend(src.inner.iter());
    }
}

impl From<HashMap<(u8, u8), u8>> for SparseCostMatrix {
    fn from(mut map: HashMap<(u8, u8), u8>) -> Self {
        map.retain(|pos, _| { pos.0 < 50 && pos.1 < 50 });
        SparseCostMatrix { inner: map }
    }
}

impl From<HashMap<Position, u8>> for SparseCostMatrix {
    fn from(mut map: HashMap<Position, u8>) -> Self {
        SparseCostMatrix { inner: map.drain().map(|(pos, val)| { (pos.into(), val) }).collect() }
    }
}

impl From<CostMatrix> for SparseCostMatrix {
    fn from(js_matrix: CostMatrix) -> Self {
        let vals: Vec<u8> = js_matrix.get_bits().to_vec();
        assert!(vals.len() == 2500, "JS CostMatrix had length {} instead of 2500.", vals.len());

        SparseCostMatrix {
            inner: vals.into_iter().enumerate().filter_map(|(idx, val)| {
                    // 0 is the same as unset, so filtering it out
                    if val > 0 {
                        Some((idx_as_pos(idx), val))
                    } else {
                        None
                    }
                }).collect()
        }
    }
}

impl From<LocalCostMatrix> for SparseCostMatrix {
    fn from(lcm: LocalCostMatrix) -> Self {
        SparseCostMatrix {
            inner: lcm.iter().filter_map(|(xy, val)| { 
                if *val > 0 { 
                    Some((xy, *val)) 
                } else { 
                    None 
                }
            }).collect() 
        }
    }
}

impl From<SparseCostMatrix> for LocalCostMatrix {
    fn from(mut scm: SparseCostMatrix) -> Self {
        let mut lcm = LocalCostMatrix::new();
        for (pos, val) in scm.inner.drain() {
            lcm[pos] = val;
        }
        lcm
    }
}

// need custom implementation in order to ensure length of 'bits' is always 2500
mod serde_impls {
    use serde::{de::Error, de::Unexpected, Deserialize, Deserializer, Serialize, Serializer};
    use std::convert::TryInto;
    use std::collections::HashMap;

    use super::{LocalCostMatrix, SparseCostMatrix};

    impl Serialize for LocalCostMatrix {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.bits.serialize(s)
        }
    }

    impl<'de> Deserialize<'de> for LocalCostMatrix {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let vec_bits: Vec<u8> = Vec::deserialize(deserializer)?;

            if vec_bits.len() != 2500 {
                return Err(D::Error::invalid_length(
                    vec_bits.len(),
                    &"a vec of length 2500",
                ));
            }

            // SAFETY: If the length wasn't right, we would have hit the check above
            Ok(LocalCostMatrix { bits: vec_bits.try_into().unwrap() })
        }
    }

    impl Serialize for SparseCostMatrix {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.inner.serialize(s)
        }
    }

    impl<'de> Deserialize<'de> for SparseCostMatrix {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let map: HashMap<(u8, u8), u8> = HashMap::deserialize(deserializer)?;

            if map.keys().any(|pos| { pos.0 >= 50 || pos.1 >= 50 }) {
                return Err(D::Error::invalid_value(
                    Unexpected::Map,
                    &"a map whose keys are (u8, u8) with both values in 0..50",
                ));
            }

            Ok(SparseCostMatrix { inner: map })
        }
    }
}
