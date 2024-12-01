pub mod template;

// Use this file to add helper functions and additional modules.

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

impl_tuple_parse!(A);
impl_tuple_parse!(A B);
impl_tuple_parse!(A B C);
impl_tuple_parse!(A B C D);
impl_tuple_parse!(A B C D E);

/* -------------------------------------------------------------------------- */
