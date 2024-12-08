pub mod template;

// Use this file to add helper functions and additional modules.

/* -------------------------------------------------------------------------- */

#[inline]
pub fn contains_point(size: glam::IVec2, point: glam::IVec2) -> bool {
    (0..size.x).contains(&point.x) && (0..size.y).contains(&point.y)
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
