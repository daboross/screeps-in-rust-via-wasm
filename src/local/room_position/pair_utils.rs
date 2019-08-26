//! Utilities for working with Position and coordinate pairs
use super::Position;

impl Position {
    /// Returns this position's in-room coordinates as a pair of unsigned
    /// integers.
    #[inline]
    pub fn coords(&self) -> (u32, u32) {
        (self.x(), self.y())
    }

    /// Returns this position's in-room coordinates as a pair of signed
    /// integers.
    #[inline]
    pub fn coords_signed(&self) -> (i32, i32) {
        (self.x() as i32, self.y() as i32)
    }
}

// Note: we would usually implement `From<Position> for (u8, u8)`, but this
// implementation is not allowed as it'd be implementing it on a nested type,
// and both the outer type (`(T, T)`), and the inner ones (`u8`) are from an
// external crate.

impl Into<(u8, u8)> for Position {
    /// Turns this positions's in-room coordinates into a pair of `u8` values.
    #[inline]
    fn into(self) -> (u8, u8) {
        (self.x() as u8, self.y() as u8)
    }
}

impl Into<(u16, u16)> for Position {
    /// Turns this positions's in-room coordinates into a pair of `u16` values.
    #[inline]
    fn into(self) -> (u16, u16) {
        (self.x() as u16, self.y() as u16)
    }
}

impl Into<(u32, u32)> for Position {
    /// Turns this positions's in-room coordinates into a pair of `u32` values.
    #[inline]
    fn into(self) -> (u32, u32) {
        (self.x(), self.y())
    }
}

impl Into<(u64, u64)> for Position {
    /// Turns this positions's in-room coordinates into a pair of `u64` values.
    #[inline]
    fn into(self) -> (u64, u64) {
        (self.x() as u64, self.y() as u64)
    }
}

impl Into<(i8, i8)> for Position {
    /// Turns this positions's in-room coordinates into a pair of `i8` values.
    #[inline]
    fn into(self) -> (i8, i8) {
        (self.x() as i8, self.y() as i8)
    }
}

impl Into<(i16, i16)> for Position {
    /// Turns this positions's in-room coordinates into a pair of `i16` values.
    #[inline]
    fn into(self) -> (i16, i16) {
        (self.x() as i16, self.y() as i16)
    }
}

impl Into<(i32, i32)> for Position {
    /// Turns this positions's in-room coordinates into a pair of `i32` values.
    #[inline]
    fn into(self) -> (i32, i32) {
        (self.x() as i32, self.y() as i32)
    }
}

impl Into<(i64, i64)> for Position {
    /// Turns this positions's in-room coordinates into a pair of `i64` values.
    #[inline]
    fn into(self) -> (i64, i64) {
        (self.x() as i64, self.y() as i64)
    }
}
