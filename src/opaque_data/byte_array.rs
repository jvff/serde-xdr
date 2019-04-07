pub trait ByteArray: AsMut<[u8]> {
    fn default() -> Self;
    fn len() -> usize;
}

macro_rules! byte_array_impl {
    ($length:expr) => {
        impl ByteArray for [u8; $length] {
            fn default() -> Self {
                [0u8; $length]
            }

            fn len() -> usize {
                $length
            }
        }
    };
}

macro_rules! byte_array_impls {
    ( $( $length:expr ),* $(,)* ) => {
        $( byte_array_impl!($length); )*
    };
}

byte_array_impls!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
);
