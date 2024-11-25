use std::num::{
    NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
    NonZeroU64, NonZeroU8, NonZeroUsize, Wrapping,
};

use crate::Int;

/// A type that can be used as key for [`IntMap`].
///
/// The type needs to be integer based, i.e. it can be represented by a unique integer.
/// This can be useful for types that wraps integers for type safety (e.g. [`Ipv4Addr`])
/// or for enforcing invariants (e.g. [`NonZeroU64`]).
///
/// # Example
///
/// ```
/// use intmap::{IntKey, IntMap};
///
/// #[derive(Clone, Copy)]
/// struct MyKey(u64);
///
/// impl IntKey for MyKey {
///     type Int = u64;
///
///     // You could also choose another prime number
///     const PRIME: Self::Int = u64::PRIME;
///
///     fn into_int(self) -> Self::Int {
///         self.0
///     }
/// }
///
/// let map: IntMap<MyKey, f32> = IntMap::new();
/// ```
///
/// [`IntMap`]: crate::IntMap
/// [`Ipv4Addr`]: std::net::Ipv4Addr
/// [`NonZeroU64`]: std::num::NonZeroU64
pub trait IntKey: Copy {
    /// The underlying integer that will be used as actual key.
    type Int: Int;

    /// The prime number used for hashing.
    ///
    /// The choice might influence the number of key collisions which affects the performance.
    const PRIME: Self::Int;

    /// Converts the key into the underlying integer.
    ///
    /// [`IntMap`] assumes that this is a cheap operation and that two different values
    /// don't return the same integer.
    ///
    /// [`IntMap`]: crate::IntMap
    fn into_int(self) -> Self::Int;
}

macro_rules! impl_int_key_for_int {
    ($self:ident, $prime:expr) => {
        impl IntKey for $self {
            type Int = $self;

            const PRIME: Self::Int = $prime;

            fn into_int(self) -> Self::Int {
                self
            }
        }
    };
}

// Source: https://t5k.org/lists/2small/
// Checked with: https://www.numberempire.com/primenumbers.php
const U8_PRIME_MAX: u8 = u8::MAX - 4; // 251
const U16_PRIME_MAX: u16 = u16::MAX - 14; // 65521
const U32_PRIME_MAX: u32 = u32::MAX - 4; // 4294967291
const U64_PRIME_MAX: u64 = u64::MAX - 58; // 18446744073709551557
const U128_PRIME_MAX: u128 = u128::MAX - 158; // 340282366920938463463374607431768211297

impl_int_key_for_int!(u8, U8_PRIME_MAX);
impl_int_key_for_int!(u16, U16_PRIME_MAX);
impl_int_key_for_int!(u32, U32_PRIME_MAX);
impl_int_key_for_int!(u64, U64_PRIME_MAX);
impl_int_key_for_int!(u128, U128_PRIME_MAX);
#[cfg(target_pointer_width = "16")]
impl_int_key_for_int!(usize, U16_PRIME_MAX as usize);
#[cfg(target_pointer_width = "32")]
impl_int_key_for_int!(usize, U32_PRIME_MAX as usize);
#[cfg(target_pointer_width = "64")]
impl_int_key_for_int!(usize, U64_PRIME_MAX as usize);

macro_rules! impl_int_key_for_signed_int {
    ($self:ident, $unsigned:ident) => {
        impl IntKey for $self {
            type Int = $unsigned;

            const PRIME: Self::Int = $unsigned::PRIME;

            fn into_int(self) -> Self::Int {
                self as $unsigned
            }
        }
    };
}

impl_int_key_for_signed_int!(i8, u8);
impl_int_key_for_signed_int!(i16, u16);
impl_int_key_for_signed_int!(i32, u32);
impl_int_key_for_signed_int!(i64, u64);
impl_int_key_for_signed_int!(i128, u128);
impl_int_key_for_signed_int!(isize, usize);

macro_rules! impl_int_key_for_non_zero_int {
    ($non_zero_int:ident, $int:ident) => {
        impl IntKey for $non_zero_int {
            type Int = <$int as IntKey>::Int;

            const PRIME: Self::Int = $int::PRIME;

            fn into_int(self) -> Self::Int {
                self.get().into_int()
            }
        }
    };
}

impl_int_key_for_non_zero_int!(NonZeroU8, u8);
impl_int_key_for_non_zero_int!(NonZeroU16, u16);
impl_int_key_for_non_zero_int!(NonZeroU32, u32);
impl_int_key_for_non_zero_int!(NonZeroU64, u64);
impl_int_key_for_non_zero_int!(NonZeroUsize, usize);
impl_int_key_for_non_zero_int!(NonZeroI8, i8);
impl_int_key_for_non_zero_int!(NonZeroI16, i16);
impl_int_key_for_non_zero_int!(NonZeroI32, i32);
impl_int_key_for_non_zero_int!(NonZeroI64, i64);
impl_int_key_for_non_zero_int!(NonZeroIsize, isize);

impl<K: IntKey> IntKey for Wrapping<K> {
    type Int = K::Int;

    const PRIME: Self::Int = K::PRIME;

    fn into_int(self) -> Self::Int {
        self.0.into_int()
    }
}

impl IntKey for std::net::Ipv4Addr {
    type Int = u32;

    const PRIME: Self::Int = u32::PRIME;

    fn into_int(self) -> Self::Int {
        // Copied from Ipv4Addr::to_bits, which does not exist for our MSRV
        u32::from_be_bytes(self.octets())
    }
}

impl IntKey for std::net::Ipv6Addr {
    type Int = u128;

    const PRIME: Self::Int = u128::PRIME;

    fn into_int(self) -> Self::Int {
        // Copied from Ipv6Addr::to_bits, which does not exist for our MSRV
        u128::from_be_bytes(self.octets())
    }
}
