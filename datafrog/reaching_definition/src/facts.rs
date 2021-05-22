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

        impl Into<usize> for $t {
            fn into(self) -> usize {
                self.index as usize
            }
        }
    };
}

index_type!(Point);
index_type!(Definition);
