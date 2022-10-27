//! Type aliases for common bit sizes of [`Uint`] and [`Bits`].
use crate::{Bits, Uint};

/// [`Uint`] for `0` bits. Always zero. Similar to `()`.
pub type U0 = Uint<0, 0>;

/// [`Uint`] for `1` bit. Similar to [`bool`].
pub type U1 = Uint<1, 1>;

/// [`Uint`] for `8` bits. Similar to [`u8`].
pub type U8 = Uint<8, 1>;

/// [`Uint`] for `16` bits. Similar to [`u16`].
pub type U16 = Uint<16, 1>;

/// [`Uint`] for `32` bits. Similar to [`u32`].
pub type U32 = Uint<32, 1>;

/// [`Uint`] for `64` bits. Similar to [`u64`].
pub type U64 = Uint<64, 1>;

/// [`Uint`] for `128` bits. Similar to [`u128`].
pub type U128 = Uint<128, 2>;

macro_rules! bit_alias {
    ($($name:ident($bits:expr, $limbs:expr);)*) => {$(
        #[doc = concat!("[`Bits`] for `", stringify!($bits),"` bits.")]
        pub type $name = Bits<$bits, $limbs>;
    )*};
}

bit_alias! {
    B0(0, 0);
    B1(1, 1);
    B8(8, 1);
    B16(16, 1);
    B32(32, 1);
    B64(64, 1);
    B128(128, 2);
}

macro_rules! alias {
    ($($uname:ident $bname:ident ($bits:expr, $limbs:expr);)*) => {$(
        #[doc = concat!("[`Uint`] for `", stringify!($bits),"` bits.")]
        pub type $uname = Uint<$bits, $limbs>;
        #[doc = concat!("[`Bits`] for `", stringify!($bits),"` bits.")]
        pub type $bname = Bits<$bits, $limbs>;
    )*};
}

alias! {
    U160 B160 (160, 3);
    U192 B192 (256, 3);
    U256 B256 (256, 4);
    U320 B320 (320, 5);
    U384 B384 (384, 6);
    U448 B448 (448, 7);
    U512 B512 (512, 8);
    U1024 B1024 (1024, 16);
    U2048 B2048 (2048, 32);
    U4096 B4096 (4096, 64);
}

// TODO: I0, I1, I8, ... I4096
