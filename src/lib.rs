#![allow(dead_code)]
#![feature(portable_simd)]

macro_rules! solutions {
    ($($day:literal $id:ident,)*) => {
        paste::paste! {
            $(
                mod [<d $day $id>];
                pub use [<d $day $id>]::run as [<d $day $id>];
            )*

            pub fn select(id: &str) -> Option<fn(&str) -> String> {
                match id {
                    $(concat!("d", stringify!($day), stringify!($id)) => Some(|s| [<d $day $id>](StrOrSlice::str_or_slice(s)).to_string()),)*
                    _ => None,
                }
            }

            pub const OPTIONS: &[&'static str] = &[];

            #[macro_export]
            macro_rules! idents {
                ($macro:ident) => {
                    $macro!($($day [<d $day $id>],)*);
                };
            }
        }
    };
}

solutions! {
    1 p1,
    1 p1_euclid,
    1 p2,
    2 p1,
    2 p2,
    3 p1,
    3 p2,
    5 p1_zip,
    5 p1_bsearch,
    5 p2,
    6 p1,
    6 p2,
    7 p1_copy,
    7 p1_fill,
    7 p2_fill,
}

pub trait StrOrSlice {
    fn str_or_slice(s: &str) -> &Self;
}

impl StrOrSlice for str {
    fn str_or_slice(s: &str) -> &Self { s }
}

impl StrOrSlice for [u8] {
    fn str_or_slice(s: &str) -> &Self { s.as_bytes() }
}
