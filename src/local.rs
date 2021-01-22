//! Pure-data structures relating to Screeps.
use std::ops::Range;

mod cost_matrix;
mod js_object_id;
mod object_id;
mod room_name;
mod room_position;

/// Represents two constants related to room names.
///
/// First, this is the constant added to room coordinates before they're stored
/// in the packed representation.
///
/// Second, `-HALF_WORLD_SIZE` is the minimum representable room name
/// coordinate, and `HALF_WORLD_SIZE - 1` is the maximum representable room name
/// coordinate.
const HALF_WORLD_SIZE: i32 = 128;

/// Valid room name coordinates.
const VALID_ROOM_NAME_COORDINATES: Range<i32> = -HALF_WORLD_SIZE..HALF_WORLD_SIZE;

pub use self::{cost_matrix::*, js_object_id::*, object_id::*, room_name::*, room_position::*};
