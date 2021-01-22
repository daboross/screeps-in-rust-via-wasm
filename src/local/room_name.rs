use std::{
    cmp::{Ord, Ordering, PartialOrd},
    error,
    fmt::{self, Write},
    ops,
    str::FromStr,
};

use arrayvec::ArrayString;

use super::{HALF_WORLD_SIZE, VALID_ROOM_NAME_COORDINATES};

/// A structure representing a room name.
///
/// # Ordering
///
/// To facilitate use as a key in a [`BTreeMap`] or other similar data
/// structures, `RoomName` implements [`PartialOrd`] and [`Ord`].
///
/// `RoomName`s are ordered first by y position, then by x position. North is
/// considered less than south, and west less than east.
///
/// The total ordering is `N127W127`, `N127W126`, `N127W125`, ..., `N127W0`,
/// `N127E0`, ..., `N127E127`, `N126W127`, ..., `S127E126`, `S127E127`.
///
/// This follows left-to-right reading order when looking at the Screeps map
/// from above.
///
/// [`BTreeMap`]: std::collections::BTreeMap
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RoomName {
    /// A bit-packed integer, containing, from highest-order to lowest:
    ///
    /// - 1 byte: (room_x) + 128
    /// - 1 byte: (room_y) + 128
    ///
    /// For `Wxx` rooms, `room_x = -xx - 1`. For `Exx` rooms, `room_x = xx`.
    ///
    /// For `Nyy` rooms, `room_y = -yy - 1`. For `Syy` rooms, `room_y = yy`.
    ///
    /// This is the same representation of the upper 16 bits of [`Position`]'s
    /// packed representation.
    ///
    /// [`Position`]: crate::local::Position
    packed: u16,
}

impl fmt::Display for RoomName {
    /// Formats this room name into the format the game expects.
    ///
    /// Resulting string will be `(E|W)[0-9]+(N|S)[0-9]+`, and will result
    /// in the same RoomName if passed into [`RoomName::new`].
    ///
    /// [`RoomName::new`]: struct.RoomName.html#method.new
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x_coord = self.x_coord();
        let y_coord = self.y_coord();

        if self.packed == 0 {
            write!(f, "sim")?;
        } else {
            if x_coord >= 0 {
                write!(f, "E{}", x_coord)?;
            } else {
                write!(f, "W{}", -x_coord - 1)?;
            }

            if y_coord >= 0 {
                write!(f, "S{}", y_coord)?;
            } else {
                write!(f, "N{}", -y_coord - 1)?;
            }
        }

        Ok(())
    }
}

impl RoomName {
    /// Parses a room name from a string.
    ///
    /// This will parse the input string, returning an error if it is in an
    /// invalid room name.
    ///
    /// The expected format can be represented by the regex
    /// `[ewEW][0-9]+[nsNS][0-9]+`.
    #[inline]
    pub fn new<T>(x: &T) -> Result<Self, RoomNameParseError>
    where
        T: AsRef<str> + ?Sized,
    {
        x.as_ref().parse()
    }

    #[inline]
    pub(crate) fn from_packed(packed: u16) -> Self {
        RoomName { packed }
    }

    /// Creates a new room name from room coords with direction implicit in
    /// sign.
    ///
    /// For `Wxx` rooms, `room_x = -xx - 1`. For `Exx` rooms, `room_x = xx`.
    ///
    /// For `Nyy` rooms, `room_y = -yy - 1`. For `Syy` rooms, `room_y = yy`.
    ///
    /// # Errors
    ///
    /// Returns an error if the coordinates are outside of the valid room name
    /// bounds.
    pub(super) fn from_coords(x_coord: i32, y_coord: i32) -> Result<Self, RoomNameParseError> {
        if !VALID_ROOM_NAME_COORDINATES.contains(&x_coord)
            || !VALID_ROOM_NAME_COORDINATES.contains(&y_coord)
        {
            return Err(RoomNameParseError::PositionOutOfBounds { x_coord, y_coord });
        }

        let room_x = (x_coord + HALF_WORLD_SIZE) as u16;
        let room_y = (y_coord + HALF_WORLD_SIZE) as u16;

        Ok(Self::from_packed((room_x << 8) | room_y))
    }

    /// Gets the x coordinate.
    ///
    /// For `Wxx` rooms, returns `-xx - 1`. For `Exx` rooms, returns `xx`.
    #[inline]
    pub(super) fn x_coord(&self) -> i32 {
        ((self.packed >> 8) & 0xFF) as i32 - HALF_WORLD_SIZE
    }

    /// Gets the y coordinate.
    ///
    /// For `Nyy` rooms, returns `-yy - 1`. For `Syy` rooms, returns `yy`.
    #[inline]
    pub(super) fn y_coord(&self) -> i32 {
        (self.packed & 0xFF) as i32 - HALF_WORLD_SIZE
    }

    #[inline]
    pub(super) fn packed_repr(&self) -> u16 {
        self.packed
    }

    /// Converts this RoomName into an efficient, stack-based string.
    ///
    /// This is equivalent to [`ToString::to_string`], but involves no
    /// allocation.
    pub fn to_array_string(&self) -> ArrayString<[u8; 8]> {
        let mut res = ArrayString::new();
        write!(res, "{}", self).expect("expected ArrayString write to be infallible");
        res
    }
}

impl ops::Add<(i32, i32)> for RoomName {
    type Output = Self;

    /// Offsets this room name by a given horizontal and vertical (x, y) pair.
    ///
    /// The first number offsets to the west when negative and to the east when
    /// positive. The first number offsets to the north when negative and to
    /// the south when positive.
    ///
    /// # Panics
    ///
    /// Will panic if the addition overflows the boundaries of RoomName.
    #[inline]
    fn add(self, (x, y): (i32, i32)) -> Self {
        RoomName::from_coords(self.x_coord() + x, self.y_coord() + y)
            .expect("expected addition to keep RoomName in-bounds")
    }
}

impl ops::Sub<(i32, i32)> for RoomName {
    type Output = Self;

    /// Offsets this room name in the opposite direction from the coordinates.
    ///
    /// See the implementation for `Add<(i32, i32)>`.
    ///
    /// # Panics
    ///
    /// Will panic if the subtraction overflows the boundaries of RoomName.
    #[inline]
    fn sub(self, (x, y): (i32, i32)) -> Self {
        RoomName::from_coords(self.x_coord() - x, self.y_coord() - y)
            .expect("expected addition to keep RoomName in-bounds")
    }
}

impl ops::Sub<RoomName> for RoomName {
    type Output = (i32, i32);

    /// Subtracts one room name from the other, extracting the difference.
    ///
    /// The first return value represents east/west offset, with 'more east'
    /// being positive and 'more west' being negative.
    ///
    /// The second return value represents north/south offset, with 'more south'
    /// being positive and 'more north' being negative.
    ///
    /// This coordinate system agrees with the implementations `Add<(i32, i32)>
    /// for RoomName` and `Sub<(i32, i32)> for RoomName`.
    #[inline]
    fn sub(self, other: RoomName) -> (i32, i32) {
        (
            self.x_coord() - other.x_coord(),
            self.y_coord() - other.y_coord(),
        )
    }
}

impl FromStr for RoomName {
    type Err = RoomNameParseError;

    fn from_str(s: &str) -> Result<Self, RoomNameParseError> {
        parse_to_coords(s)
            .map_err(|()| RoomNameParseError::new(s))
            .and_then(|(x, y)| RoomName::from_coords(x, y))
    }
}

fn parse_to_coords(s: &str) -> Result<(i32, i32), ()> {
    if s == "sim" {
        return Ok((-HALF_WORLD_SIZE, -HALF_WORLD_SIZE));
    }

    let mut chars = s.char_indices();

    let east = match chars.next() {
        Some((_, 'E')) | Some((_, 'e')) => true,
        Some((_, 'W')) | Some((_, 'w')) => false,
        _ => return Err(()),
    };

    let (x_coord, south): (i32, bool) = {
        // we assume there's at least one number character. If there isn't,
        // we'll catch it when we try to parse this substr.
        let (start_index, _) = chars.next().ok_or(())?;
        let end_index;
        let south;
        loop {
            match chars.next().ok_or(())? {
                (i, 'N') | (i, 'n') => {
                    end_index = i;
                    south = false;
                    break;
                }
                (i, 'S') | (i, 's') => {
                    end_index = i;
                    south = true;
                    break;
                }
                _ => continue,
            }
        }

        let x_coord = s[start_index..end_index].parse().map_err(|_| ())?;

        (x_coord, south)
    };

    let y_coord: i32 = {
        let (start_index, _) = chars.next().ok_or(())?;

        s[start_index..s.len()].parse().map_err(|_| ())?
    };

    let room_x = if east { x_coord } else { -x_coord - 1 };
    let room_y = if south { y_coord } else { -y_coord - 1 };

    Ok((room_x, room_y))
}

/// An error representing when a string can't be parsed into a
/// [`RoomName`].
///
/// [`RoomName`]: struct.RoomName.html
#[derive(Clone, Debug)]
pub enum RoomNameParseError {
    TooLarge { length: usize },
    InvalidString { string: ArrayString<[u8; 8]> },
    PositionOutOfBounds { x_coord: i32, y_coord: i32 },
}

impl RoomNameParseError {
    /// Private method to construct a `RoomNameParseError`.
    fn new(failed_room_name: &str) -> Self {
        match ArrayString::from(failed_room_name) {
            Ok(string) => RoomNameParseError::InvalidString { string },
            Err(_) => RoomNameParseError::TooLarge {
                length: failed_room_name.len(),
            },
        }
    }
}

impl error::Error for RoomNameParseError {}

impl fmt::Display for RoomNameParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RoomNameParseError::TooLarge { length } => write!(
                f,
                "got invalid room name, too large to stick in error. \
                 expected length 8 or less, got length {}",
                length
            ),
            RoomNameParseError::InvalidString { string } => write!(
                f,
                "expected room name formatted `[ewEW][0-9]+[nsNS][0-9]+`, found `{}`",
                string
            ),
            RoomNameParseError::PositionOutOfBounds { x_coord, y_coord } => write!(
                f,
                "expected room name with coords within -128..+128, found {}, {}",
                x_coord, y_coord,
            ),
        }
    }
}

impl PartialEq<str> for RoomName {
    fn eq(&self, other: &str) -> bool {
        let s = self.to_array_string();
        s.eq_ignore_ascii_case(other)
    }
}
impl PartialEq<RoomName> for str {
    #[inline]
    fn eq(&self, other: &RoomName) -> bool {
        // Explicitly call the impl for `PartialEq<str>` so that we don't end up
        // accidentally calling one of the other implementations and ending up in an
        // infinite loop.
        //
        // This one in particular would probably be OK, but I've written it this way to
        // be consistent with the others, and to ensure that if this code changes in
        // this future it'll stay working.
        <RoomName as PartialEq<str>>::eq(other, self)
    }
}

impl PartialEq<&str> for RoomName {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        <RoomName as PartialEq<str>>::eq(self, other)
    }
}

impl PartialEq<RoomName> for &str {
    #[inline]
    fn eq(&self, other: &RoomName) -> bool {
        <RoomName as PartialEq<str>>::eq(other, self)
    }
}

impl PartialEq<String> for RoomName {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        <RoomName as PartialEq<str>>::eq(self, &other)
    }
}

impl PartialEq<RoomName> for String {
    #[inline]
    fn eq(&self, other: &RoomName) -> bool {
        <RoomName as PartialEq<str>>::eq(other, self)
    }
}

impl PartialEq<&String> for RoomName {
    #[inline]
    fn eq(&self, other: &&String) -> bool {
        <RoomName as PartialEq<str>>::eq(self, other)
    }
}

impl PartialEq<RoomName> for &String {
    #[inline]
    fn eq(&self, other: &RoomName) -> bool {
        <RoomName as PartialEq<str>>::eq(other, self)
    }
}

impl PartialOrd for RoomName {
    #[inline]
    fn partial_cmp(&self, other: &RoomName) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RoomName {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y_coord()
            .cmp(&other.y_coord())
            .then_with(|| self.x_coord().cmp(&other.x_coord()))
    }
}

mod serde {
    use std::fmt;

    use serde::{
        de::{Error, Unexpected, Visitor},
        Deserialize, Deserializer, Serialize, Serializer,
    };

    use super::RoomName;

    impl Serialize for RoomName {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&self.to_array_string())
        }
    }

    struct RoomNameVisitor;

    impl<'de> Visitor<'de> for RoomNameVisitor {
        type Value = RoomName;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str(
                "room name formatted `(E|W)[0-9]+(N|S)[0-9]+` with both numbers within -128..128",
            )
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            v.parse()
                .map_err(|_| E::invalid_value(Unexpected::Str(v), &self))
        }
    }

    impl<'de> Deserialize<'de> for RoomName {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(RoomNameVisitor)
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_string_equality() {
        use super::RoomName;
        let room_names = vec!["E21N4", "w6S42", "W17s5", "e2n5", "sim"];
        for room_name in room_names {
            assert_eq!(room_name, RoomName::new(room_name).unwrap());
            assert_eq!(RoomName::new(room_name).unwrap(), room_name);
            assert_eq!(RoomName::new(room_name).unwrap(), &room_name.to_string());
            assert_eq!(&room_name.to_string(), RoomName::new(room_name).unwrap());
        }
    }
}
