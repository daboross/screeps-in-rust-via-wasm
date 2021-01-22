//! Utilities for doing math on [`Position`]s which are present in the
//! JavaScript API.
use crate::constants::Direction;

use super::Position;

impl Position {
    /// Gets linear direction to the specified position.
    ///
    /// Note that this chooses between `Top`/`Bottom`/`Left`/`Right` and
    /// `TopLeft`/`TopRight`/`BottomLeft`/`BottomRight` by the magnitude in both
    /// directions. For instance, [`Direction::Top`] can be returned even
    /// if the target has a slightly different `x` coordinate.
    pub fn get_direction_to(self, target: Position) -> Option<Direction> {
        // Logic copied from https://github.com/screeps/engine/blob/
        // 020ba168a1fde9a8072f9f1c329d5c0be8b440d7/src/utils.js#L73-L107
        let (dx, dy) = target - self;
        if dx.abs() > dy.abs() * 2 {
            if dx > 0 {
                Some(Direction::Right)
            } else {
                Some(Direction::Left)
            }
        } else if dy.abs() > dx.abs() * 2 {
            if dy > 0 {
                Some(Direction::Bottom)
            } else {
                Some(Direction::Top)
            }
        } else {
            if dx > 0 && dy > 0 {
                Some(Direction::BottomRight)
            } else if dx > 0 && dy < 0 {
                Some(Direction::TopRight)
            } else if dx < 0 && dy > 0 {
                Some(Direction::BottomLeft)
            } else if dx < 0 && dy < 0 {
                Some(Direction::TopLeft)
            } else {
                None
            }
        }
    }

    /// Gets linear range to the specified position.
    ///
    /// This operates on positions as "world positions", and will return an
    /// accurate range for positions in different rooms. Note that the
    /// corresponding JavaScript method, `RoomPosition.getRangeTo` returns
    /// `Infinity` if given positions in different rooms.
    #[inline]
    pub fn get_range_to(self, target: Position) -> u32 {
        let (dx, dy) = self - target;
        dx.abs().max(dy.abs()) as u32
    }

    /// Checks whether this position is in the given range of another position.
    ///
    /// This operates on positions as "world positions", and may return true for
    /// positions in different rooms which are still within the given range.
    /// Note that the corresponding JavaScript method, `RoomPosition.inRangeTo`,
    /// will always return `false` for positions from different rooms.
    #[inline]
    pub fn in_range_to(self, target: Position, range: u32) -> bool {
        self.get_range_to(target) <= range
    }

    /// Checks whether this position is the same as the specified position.
    ///
    /// Note that this is equivalent to `this_pos == target.pos()`.
    #[inline]
    pub fn is_equal_to(self, target: Position) -> bool {
        self == target
    }

    /// True if this position is in the same room as the target, and the range
    /// is at most 1.
    #[inline]
    pub fn is_near_to(self, target: Position) -> bool {
        self.room_name() == target.room_name()
            && (self.x() as i32 - target.x() as i32).abs() <= 1
            && (self.y() as i32 - target.y() as i32).abs() <= 1
    }
}

#[cfg(test)]
mod test {
    use crate::{Direction, Position, RoomName};

    #[test]
    fn test_direction_to() {
        let a = Position::new(1, 1, RoomName::from_coords(1, 1).unwrap());
        let b = Position::new(2, 2, RoomName::from_coords(1, 1).unwrap());
        assert_eq!(a.get_direction_to(b), Some(Direction::BottomRight));
    }
}
