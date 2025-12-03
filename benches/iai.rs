macro_rules! main {
    ($($day:literal $id:ident,)*) => {
        paste::paste! {
            $(
                fn [<bench_ $id>]() {
                    let input = include_str!(concat!("../inputs/day", stringify!($day), ".txt"));
                    iai::black_box(::aoc2025::$id(::aoc2025::StrOrSlice::str_or_slice(input)));
                }
            )*
            iai::main!($([<bench_ $id>]),*);
        }
    }
}

aoc2025::idents!(main);
