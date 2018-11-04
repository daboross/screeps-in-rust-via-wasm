use std::{f64, marker::PhantomData, mem};

use stdweb::{web::TypedArray, Array, Object, Reference, UnsafeTypedArray};

use {
    objects::{HasPosition, RoomPosition},
    positions::LocalRoomPosition,
    traits::TryInto,
};

#[derive(Clone, Debug)]
pub struct LocalCostMatrix {
    /// Length should be 2500.
    bits: Vec<u8>,
}

#[inline]
fn pos_as_idx(x: u8, y: u8) -> usize {
    (x as usize) * 50 + (y as usize)
}

impl Default for LocalCostMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalCostMatrix {
    pub fn new() -> Self {
        LocalCostMatrix {
            bits: vec![0; 2500],
        }
    }

    pub fn set(&mut self, x: u8, y: u8, val: u8) {
        self.bits[pos_as_idx(x, y)] = val;
    }

    pub fn get(&self, x: u8, y: u8) -> u8 {
        self.bits[pos_as_idx(x, y)]
    }

    /// Copies all data into an JavaScript CostMatrix for use.
    ///
    /// This is slower than [`as_uploaded`], but much safer.
    ///
    /// [`as_uploaded`]: #method.as_uploaded
    pub fn upload(&self) -> CostMatrix<'static> {
        let bits: TypedArray<u8> = self.bits[..].into();

        CostMatrix {
            inner: (js! {
                var matrix = Object.create(CostMatrix.prototype);
                matrix._bits = @{bits};
                return matrix;
            })
            .try_into()
            .expect("expected function returning CostMatrix to return a Reference"),
            lifetime: PhantomData,
        }
    }

    /// Temporarily exposes the bits of this matrix as a cost matrix.
    ///
    /// # Unsafety
    ///
    /// There are two main invariants you must uphold after using this function:
    ///
    /// 1. The `CostMatrix` can only be used in JS code as long as this `LocalCostMatrix` is alive.
    ///    Doing otherwise will result in undefined behavior, mainly JS being allowed to read/
    ///    manipulate uninitialized rust memory or rust memory that's been repurposed.
    ///
    /// 2. The `set` method of the cost matrix must not be used - it must be read only. This takes
    ///    &self, but technically allows mutation of the inner Vec via JavaScript access. You
    ///    should not use this method, or you will invoke Rust undefined behavior.
    ///
    /// The CostMatrix returned will _reference the internal data of this `LocalCostMatrix`_.
    pub unsafe fn as_uploaded<'a>(&'a self) -> CostMatrix<'a> {
        let bits: UnsafeTypedArray<u8> = UnsafeTypedArray::new(&self.bits);

        CostMatrix {
            inner: (js! {
                // using this first is necessary in order to uphold the invariant of
                // `UnsafeTypedArray`.
                var bits = @{bits};

                var matrix = Object.create(CostMatrix.prototype);
                matrix._bits = bits;

                return matrix;
            })
            .try_into()
            .expect("expected function returning CostMatrix to return a Reference"),
            lifetime: PhantomData,
        }
    }
}

impl Into<Vec<u8>> for LocalCostMatrix {
    /// Returns a vector of bits length 2500, where each position is
    /// `idx = ((x * 50) + y)`.
    fn into(self) -> Vec<u8> {
        self.bits
    }
}

/// A `CostMatrix` that's valid to pass as a result from a `PathFinder.search` room callback.
///
/// Lives as long as `'a` lifetime. It's unsound to leak to JS past this lifetime if this matrix
/// was created by [`LocalCostMatrix::as_uploaded`].
///
/// [`LocalCostMatrix::as_uploaded`]: struct.LocalCostMatrix.html#method.as_uploaded
pub struct CostMatrix<'a> {
    pub(crate) inner: Reference,
    pub(crate) lifetime: PhantomData<&'a ()>,
}

impl Default for CostMatrix<'static> {
    fn default() -> Self {
        CostMatrix {
            inner: js_unwrap!(new PathFinder.CostMatrix()),
            lifetime: PhantomData,
        }
    }
}

// need custom implementation in order to ensure length of 'bits' is always 2500
mod serde_impls {
    use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};

    use super::LocalCostMatrix;

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
            let bits: Vec<u8> = Vec::deserialize(deserializer)?;

            if bits.len() != 2500 {
                return Err(DeError::invalid_length(bits.len(), &"a vec of length 2500"));
            }

            Ok(LocalCostMatrix { bits })
        }
    }
}

pub struct SearchOptions<'a, F>
where
    F: Fn(String) -> CostMatrix<'a>,
{
    room_callback: F,
    plain_cost: u8,
    swamp_cost: u8,
    flee: bool,
    max_ops: u32,
    max_rooms: u32,
    max_cost: f64,
    heuristic_weight: f64,
}

impl Default for SearchOptions<'static, fn(String) -> CostMatrix<'static>> {
    fn default() -> Self {
        fn cost_matrix(_: String) -> CostMatrix<'static> {
            CostMatrix::default()
        }

        // TODO: should we fall back onto the game's default values, or is
        // it alright to copy them here?
        SearchOptions {
            room_callback: cost_matrix,
            plain_cost: 1,
            swamp_cost: 5,
            flee: false,
            max_ops: 2000,
            max_rooms: 16,
            max_cost: f64::INFINITY,
            heuristic_weight: 1.2,
        }
    }
}

impl SearchOptions<'static, fn(String) -> CostMatrix<'static>> {
    /// Creates default SearchOptions
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a, F> SearchOptions<'a, F>
where
    F: Fn(String) -> CostMatrix<'a>,
{
    /// Sets room callback - default `|_| { CostMatrix::default() }`.
    pub fn room_callback<'b, F2>(self, room_callback: F2) -> SearchOptions<'b, F2>
    where
        F2: Fn(String) -> CostMatrix<'b>,
    {
        let SearchOptions {
            room_callback: _,
            plain_cost,
            swamp_cost,
            flee,
            max_ops,
            max_rooms,
            max_cost,
            heuristic_weight,
        } = self;
        SearchOptions {
            room_callback,
            plain_cost,
            swamp_cost,
            flee,
            max_ops,
            max_rooms,
            max_cost,
            heuristic_weight,
        }
    }

    /// Sets plain cost - default `1`.
    pub fn plain_cost(mut self, cost: u8) -> Self {
        self.plain_cost = cost;
        self
    }

    /// Sets swamp cost - default `5`.
    pub fn swamp_cost(mut self, cost: u8) -> Self {
        self.swamp_cost = cost;
        self
    }

    /// Sets whether this is a flee search - default `false`.
    pub fn flee(mut self, flee: bool) -> Self {
        self.flee = flee;
        self
    }

    /// Sets maximum ops - default `2000`.
    pub fn max_ops(mut self, ops: u32) -> Self {
        self.max_ops = ops;
        self
    }

    /// Sets maximum rooms - default `16`, max `16`.
    pub fn max_rooms(mut self, rooms: u32) -> Self {
        self.max_rooms = rooms;
        self
    }

    /// Sets maximum path cost - default `f64::Infinity`.
    pub fn max_cost(mut self, cost: f64) -> Self {
        self.max_cost = cost;
        self
    }

    /// Sets heuristic weight - default `1.2`.
    pub fn heuristic_weight(mut self, weight: f64) -> Self {
        self.heuristic_weight = weight;
        self
    }
}

pub struct SearchResults {
    path: Array,
    pub ops: u32,
    pub cost: u32,
    pub incomplete: bool,
}

impl SearchResults {
    pub fn opaque_path(&self) -> &Array {
        &self.path
    }
    pub fn load_local_path(&self) -> Vec<LocalRoomPosition> {
        self.path
            .clone()
            .try_into()
            .expect("expected PathFinder.search path result to be an array of RoomPositions")
    }
    pub fn load_semi_local_path(&self) -> Vec<RoomPosition> {
        self.path
            .clone()
            .try_into()
            .expect("expected PathFinder.search path result to be an array of RoomPositions")
    }
}

/// Searches between a single origin and single goal.
pub fn search<'a, O, G, F>(
    origin: &O,
    goal: &G,
    range: u32,
    opts: SearchOptions<'a, F>,
) -> SearchResults
where
    O: HasPosition,
    G: HasPosition,
    F: Fn(String) -> CostMatrix<'a> + 'a,
{
    let pos = goal.pos();
    search_real(
        &origin.pos(),
        &js_unwrap!({pos: @{pos.as_ref()}, range: @{range}}),
        opts,
    )
}

/// Searches between a single origin and multiple goals.
pub fn search_many<'a, O, G, I, F>(origin: &O, goal: G, opts: SearchOptions<'a, F>) -> SearchResults
where
    O: HasPosition,
    G: IntoIterator<Item = (I, u32)>,
    I: HasPosition,
    F: Fn(String) -> CostMatrix<'a> + 'a,
{
    let goals: Vec<Object> = goal
        .into_iter()
        .map(|(target, range)| {
            let pos = target.pos();
            js_unwrap!({pos: @{pos.as_ref()}, range: @{range}})
        })
        .collect();
    let goals_js: Reference = js_unwrap!(@{goals});
    search_real(&origin.pos(), &goals_js, opts)
}

scoped_thread_local!(static PF_CALLBACK: Box<(Fn(String) -> Reference)>);

fn search_real<'a, F>(
    origin: &RoomPosition,
    goal: &Reference,
    opts: SearchOptions<'a, F>,
) -> SearchResults
where
    F: Fn(String) -> CostMatrix<'a> + 'a,
{
    // TODO: should we just accept `fn()` and force the user
    // to do this? it would... greatly simplify all of this.

    // This callback is the one actually passed to JavaScript.
    fn callback(input: String) -> Reference {
        PF_CALLBACK.with(|callback| callback(input))
    }

    // User provided callback: rust String -> CostMatrix
    let raw_callback = opts.room_callback;

    // Wrapped user callback: rust String -> Reference
    let callback_unboxed = move |input| raw_callback(input).inner;

    // Type erased and boxed callback: no longer a type specific to the closure passed in,
    // now unified as Box<Fn>
    let callback_type_erased: Box<Fn(String) -> Reference + 'a> = Box::new(callback_unboxed);

    // Overwrite lifetime of box inside closure so it can be stuck in scoped_thread_local storage:
    // now pretending to be static data so that it can be stuck in scoped_thread_local. This should
    // be entirely safe because we're only sticking it in scoped storage and we control the only use
    // of it, but it's still necessary because "some lifetime above the current scope but otherwise
    // unknown" is not a valid lifetime to have PF_CALLBACK have.
    let callback_lifetime_erased: Box<Fn(String) -> Reference + 'static> =
        unsafe { mem::transmute(callback_type_erased) };

    let SearchOptions {
        plain_cost,
        swamp_cost,
        flee,
        max_ops,
        max_rooms,
        heuristic_weight,
        ..
    } = opts;

    // Store callback_lifetime_erased in PF_CALLBACK for the duration of the PathFinder call and
    // make the call to PathFinder.
    //
    // See https://docs.rs/scoped-tls/0.1/scoped_tls/
    PF_CALLBACK.set(&callback_lifetime_erased, || {
        let res: ::stdweb::Reference = js_unwrap!{
            PathFinder.search(@{origin.as_ref()}, @{goal}, {
                roomCallback: @{callback},
                plainCost: @{plain_cost},
                swampCost: @{swamp_cost},
                flee: @{flee},
                maxOps: @{max_ops},
                maxRooms: @{max_rooms},
                heuristicWeight: @{heuristic_weight}
            })
        };

        SearchResults {
            path: js_unwrap!(@{&res}.path),
            ops: js_unwrap!(@{&res}.ops),
            cost: js_unwrap!(@{&res}.cost),
            incomplete: js_unwrap!(@{&res}.incomplete),
        }
    })
}
