pub mod template;

// Use this file to add helper functions and additional modules.

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
