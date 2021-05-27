macro_rules! index_type {
    ($t:ident) => {
        #[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Debug, Hash)]
        pub(crate) struct $t {
            index: u32,
        }

        impl From<usize> for $t {
            fn from(index: usize) -> $t {
                $t {
                    index: index as u32,
                }
            }
        }
    };
}

index_type!(Point);
index_type!(Variable);

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub(crate) struct Field {
    pub v: Variable,
    pub f: char,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub(crate) enum Variable_or_field {
    variable(Variable),
    field(Field),
}
