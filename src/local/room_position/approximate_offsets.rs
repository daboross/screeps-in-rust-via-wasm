//! Methods related to approximating positions between other positions.
use super::Position;

impl Position {
    /// Calculates an approximate midpoint between this point and the target.
    ///
    /// In case of a tie, rounds towards this point.
    ///
    /// If `distance_towards_target` is bigger than the distance to the target,
    /// the target is returned.
    pub fn towards(self, target: Position, distance_towards_target: i32) -> Position {
        let (offset_x, offset_y) = target - self;
        let total_distance = offset_x.abs().max(offset_y.abs());
        if distance_towards_target > total_distance {
            return target;
        }

        let new_offset_x = (offset_x * distance_towards_target) / total_distance;
        let new_offset_y = (offset_y * distance_towards_target) / total_distance;

        self + (new_offset_x, new_offset_y)
    }

    /// Calculates an approximate midpoint between this point and the target.
    ///
    /// In case of a tie, rounds towards the target.
    ///
    /// If `distance_from_target` is bigger than the distance to the target,
    /// this position is returned.
    pub fn between(self, target: Position, distance_from_target: i32) -> Position {
        target.towards(self, distance_from_target)
    }

    /// Calculates an approximate midpoint between this point and the target.
    ///
    /// In case of a tie, rounds towards the target.
    pub fn midpoint_between(self, target: Position) -> Position {
        let (offset_x, offset_y) = self - target;

        let new_offset_x = offset_x / 2;
        let new_offset_y = offset_y / 2;

        target + (new_offset_x, new_offset_y)
    }
}

#[cfg(test)]
mod test {
    use super::Position;
    use crate::RoomName;

    fn test_rooms() -> impl Iterator<Item = RoomName> {
        ["E0N0", "E20N20", "W20N0", "E20S20", "W20S20"]
            .iter()
            .map(|s| s.parse().unwrap())
    }

    fn pos(room: RoomName, x: u32, y: u32) -> Position {
        Position::new(x, y, room)
    }

    #[test]
    fn towards_accurate() {
        for room in test_rooms() {
            let start = pos(room, 10, 10);
            assert_eq!(start.towards(pos(room, 10, 15), 1), pos(room, 10, 11));
            assert_eq!(start.towards(pos(room, 10, 15), 4), pos(room, 10, 14));
            assert_eq!(start.towards(pos(room, 10, 15), 10), pos(room, 10, 15));
            assert_eq!(start.towards(pos(room, 15, 15), 1), pos(room, 11, 11));
            assert_eq!(start.towards(pos(room, 15, 15), 3), pos(room, 13, 13));
            assert_eq!(start.towards(pos(room, 15, 20), 2), pos(room, 11, 12));
            assert_eq!(start.towards(pos(room, 0, 5), 2), pos(room, 8, 9));
        }
    }
    #[test]
    fn towards_approximate() {
        for room in test_rooms() {
            let start = pos(room, 10, 10);
            assert_eq!(start.towards(pos(room, 15, 20), 1), pos(room, 10, 11));
            assert_eq!(start.towards(pos(room, 15, 20), 9), pos(room, 14, 19));
            assert_eq!(start.towards(pos(room, 0, 5), 1), pos(room, 9, 10));
        }
    }
    #[test]
    fn midpoint_accurate() {
        for room in test_rooms() {
            let start = pos(room, 10, 10);
            assert_eq!(start.midpoint_between(pos(room, 10, 16)), pos(room, 10, 13));
            assert_eq!(start.midpoint_between(pos(room, 20, 10)), pos(room, 15, 10));
            assert_eq!(start.midpoint_between(pos(room, 12, 12)), pos(room, 11, 11));
            assert_eq!(start.midpoint_between(pos(room, 4, 4)), pos(room, 7, 7));
        }
    }
    #[test]
    fn midpoint_approximate() {
        for room in test_rooms() {
            let start = pos(room, 10, 10);
            assert_eq!(start.midpoint_between(pos(room, 10, 15)), pos(room, 10, 13));
            assert_eq!(start.midpoint_between(pos(room, 19, 10)), pos(room, 15, 10));
            assert_eq!(start.midpoint_between(pos(room, 11, 11)), pos(room, 11, 11));
            assert_eq!(start.midpoint_between(pos(room, 15, 15)), pos(room, 13, 13));
            assert_eq!(start.midpoint_between(pos(room, 15, 25)), pos(room, 13, 18));
            assert_eq!(start.midpoint_between(pos(room, 9, 10)), pos(room, 9, 10));
            assert_eq!(start.midpoint_between(pos(room, 7, 10)), pos(room, 8, 10));
            assert_eq!(start.midpoint_between(pos(room, 1, 3)), pos(room, 5, 6));
        }
    }
}
