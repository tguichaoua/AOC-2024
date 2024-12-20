pub mod template;

// Use this file to add helper functions and additional modules.

pub mod array2d;
pub mod points;

/* -------------------------------------------------------------------------- */

use std::cmp::Ordering;

use array2d::Array2D;
pub use glam::{ivec2 as pos, IVec2 as Pos};
use glam::{uvec2, UVec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MapSize(glam::IVec2);

impl MapSize {
    pub fn width(&self) -> i32 {
        self.0.x
    }

    pub fn height(&self) -> i32 {
        self.0.y
    }

    pub fn contains(&self, pos: Pos) -> bool {
        pos.x >= 0 && pos.x < self.0.x && pos.y >= 0 && pos.y < self.0.y
    }
}

#[inline]
pub fn ascii_map_size(input: &str) -> MapSize {
    debug_assert!(input.is_ascii());

    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    debug_assert!(
        input.lines().all(|line| line.len() == width),
        "some lines of `input` haven't the same length"
    );

    MapSize(glam::ivec2(
        width.try_into().unwrap(),
        height.try_into().unwrap(),
    ))
}

#[inline]
pub fn ascii_map_size_uvec2(input: &str) -> UVec2 {
    debug_assert!(input.is_ascii());

    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    debug_assert!(
        input.lines().all(|line| line.len() == width),
        "some lines of `input` haven't the same length"
    );

    uvec2(width.try_into().unwrap(), height.try_into().unwrap())
}

// TODO: replace all occurance of this by `parse_ascii_map`
#[inline]
pub fn parse_ascii_map_ivec(input: &str) -> impl Iterator<Item = (Pos, u8)> + Clone + '_ {
    debug_assert!(input.is_ascii());
    input.lines().enumerate().flat_map(|(y, line)| {
        let y: i32 = y.try_into().unwrap();
        line.bytes()
            .enumerate()
            .filter(|&(_, b)| (b != b'.'))
            .map(move |(x, b)| {
                let x: i32 = x.try_into().unwrap();
                (pos(x, y), b)
            })
    })
}

#[inline]
pub fn parse_ascii_map(input: &str) -> impl Iterator<Item = (UVec2, u8)> + Clone + '_ {
    debug_assert!(input.is_ascii());
    input.lines().enumerate().flat_map(|(y, line)| {
        let y: u32 = y.try_into().unwrap();
        line.bytes().enumerate().map(move |(x, b)| {
            let x: u32 = x.try_into().unwrap();
            (uvec2(x, y), b)
        })
    })
}

/* -------------------------------------------------------------------------- */

/// Repeat a macro invocation passing 1 to 12 ident.
macro_rules! for_tuple {
    ( $macro:ident ) => {
        $macro!(A);
        $macro!(A B);
        $macro!(A B C);
        $macro!(A B C D);
        $macro!(A B C D E);
        $macro!(A B C D E F);
        $macro!(A B C D E F G);
        $macro!(A B C D E F G H);
        $macro!(A B C D E F G H I);
        $macro!(A B C D E F G H I J);
        $macro!(A B C D E F G H I J K);
        $macro!(A B C D E F G H I J K L);
    };
}

/* -------------------------------------------------------------------------- */

/// Parse a string into a tuple by splitting it by whitespaces.
pub fn parse_tuple<T: ParseTuple>(input: &str) -> anyhow::Result<T> {
    <T as ParseTuple>::parse(input)
}

pub trait ParseTuple: Sized {
    fn parse(input: &str) -> anyhow::Result<Self>;
}

macro_rules! impl_tuple_parse {
    ($($T:ident)*) => {
        impl<$($T,)*> ParseTuple for ( $($T,)* )
        where
            $( $T: ::core::str::FromStr, )*
            $( ::anyhow::Error: From<<$T as ::core::str::FromStr>::Err>, )*
        {
            fn parse(input: &str) -> anyhow::Result<Self> {
                use itertools::Itertools;
                use anyhow::Context;
                #[allow(non_snake_case)]
                let ($($T,)*) = input.split_whitespace().collect_tuple().context("invalid number of items")?;
                Ok((
                    $( $T.parse::<$T>()?, )*
                ))
            }
        }
    };
}

for_tuple!(impl_tuple_parse);

/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[allow(non_upper_case_globals)]
impl Dir {
    pub const North: Dir = Dir::Up;
    pub const East: Dir = Dir::Right;
    pub const South: Dir = Dir::Down;
    pub const West: Dir = Dir::Left;
}

impl Dir {
    #[must_use]
    #[inline]
    pub fn rotated_clockwise(self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    #[must_use]
    #[inline]
    pub fn rotated_anti_clockwise(self) -> Dir {
        match self {
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
        }
    }

    #[must_use]
    #[inline]
    pub fn opposite(self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
        }
    }

    /// Returns a unit vector pointing in the direction.
    ///
    /// The positive directions are down for Y axis and right for X axis.
    #[must_use]
    #[inline]
    pub fn as_vec_down_right(self) -> glam::IVec2 {
        match self {
            Dir::Up => glam::IVec2::new(0, -1),
            Dir::Right => glam::IVec2::new(1, 0),
            Dir::Down => glam::IVec2::new(0, 1),
            Dir::Left => glam::IVec2::new(-1, 0),
        }
    }

    #[inline]
    pub fn is_horizontal(self) -> bool {
        matches!(self, Dir::Left | Dir::Right)
    }

    #[inline]
    pub fn is_vertical(self) -> bool {
        matches!(self, Dir::Up | Dir::Down)
    }
}

/* -------------------------------------------------------------------------- */

#[inline]
pub fn ascii_array_2d(input: &str) -> Array2D<u8> {
    ascii_array_2d_with(input, |x| x)
}

#[inline]
pub fn ascii_array_2d_with<T>(input: &str, f: impl Fn(u8) -> T) -> Array2D<T> {
    debug_assert!(input.is_ascii());

    let rows_count = input.lines().count();
    let columns_count = input.lines().next().unwrap().len();
    let mut items = input.lines().flat_map(|line| line.bytes()).map(f);

    Array2D::from_elem(
        array2d::Size::from_rows_columns(rows_count, columns_count),
        |_, _| items.next().unwrap(),
    )
}

/* -------------------------------------------------------------------------- */

/// Computes `num / denom` but returns [`None`] if `denom` is `0`
/// or the result if not an integer.
#[inline]
pub fn int_div<T: num::Integer + Copy>(num: T, denom: T) -> Option<T> {
    if denom == T::zero() {
        return None;
    }
    if num % denom != T::zero() {
        return None;
    }
    Some(num / denom)
}

/// Solves the following system where all values are integers.
///
/// Returns the solution in the form `(x1, x2)`,
/// or [`None`] if there is no integer solution.
///
/// ```txt
/// ┌    ┐   ┌    ┐ ┌      ┐
/// │ x1 │   │ y1 │ │ a  b │
/// │    │ = │    │ │      │
/// │ x2 │   │ y2 │ │ c  d │
/// └    ┘   └    ┘ └      ┘
/// ```
#[inline]
pub fn int_linear_solve2<T: num::Integer + Copy>(
    y1: T,
    y2: T,
    a: T,
    b: T,
    c: T,
    d: T,
) -> Option<(T, T)> {
    let denom = a * d - b * c;

    let x1 = y1 * d - y2 * b;
    let x2 = y2 * a - y1 * c;

    let x1 = int_div(x1, denom)?;
    let x2 = int_div(x2, denom)?;

    Some((x1, x2))
}

/* -------------------------------------------------------------------------- */

#[inline]
pub fn cmp_uvec2(a: &UVec2, b: &UVec2) -> Ordering {
    a.x.cmp(&b.x).then(a.y.cmp(&b.y))
}

/* -------------------------------------------------------------------------- */

#[inline]
pub fn four_directions_bounded(
    pos: glam::UVec2,
    bounds: glam::UVec2,
) -> impl Iterator<Item = glam::UVec2> {
    let mut directions = [None; 4];

    // left
    if let Some(x) = pos.x.checked_sub(1) {
        directions[0] = Some(uvec2(x, pos.y));
    }

    // top
    if let Some(y) = pos.y.checked_sub(1) {
        directions[1] = Some(uvec2(pos.x, y));
    }

    // right
    {
        let x = pos.x + 1;
        if x < bounds.x {
            directions[2] = Some(uvec2(x, pos.y));
        }
    }

    // down
    {
        let y = pos.y + 1;
        if y < bounds.y {
            directions[3] = Some(uvec2(pos.x, y));
        }
    }

    directions.into_iter().flatten()
}

#[inline]
pub fn four_directions(pos: glam::UVec2) -> impl Iterator<Item = glam::UVec2> + Clone {
    [pos.left(), pos.up(), Some(pos.right()), Some(pos.down())]
        .into_iter()
        .flatten()
}

/* -------------------------------------------------------------------------- */

pub trait VecExt {
    fn up(&self) -> Option<UVec2>;
    fn down(&self) -> UVec2;
    fn left(&self) -> Option<UVec2>;
    fn right(&self) -> UVec2;
}

impl VecExt for UVec2 {
    #[inline]
    fn up(&self) -> Option<UVec2> {
        self.y.checked_sub(1).map(|y| uvec2(self.x, y))
    }

    #[inline]
    fn down(&self) -> UVec2 {
        uvec2(self.x, self.y + 1)
    }

    #[inline]
    fn left(&self) -> Option<UVec2> {
        self.x.checked_sub(1).map(|x| uvec2(x, self.y))
    }

    #[inline]
    fn right(&self) -> UVec2 {
        uvec2(self.x + 1, self.y)
    }
}

/* -------------------------------------------------------------------------- */
