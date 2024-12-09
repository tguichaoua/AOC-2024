pub mod template;

// Use this file to add helper functions and additional modules.

pub mod array2d;

/* -------------------------------------------------------------------------- */

use array2d::Array2D;
pub use glam::{ivec2 as pos, IVec2 as Pos};

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
pub fn parse_ascii_map(input: &str) -> impl Iterator<Item = (Pos, u8)> + Clone + '_ {
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
