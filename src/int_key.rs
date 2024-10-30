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
/// [`IntMap`]: crate::IntMap
/// [`Ipv4Addr`]: std::net::Ipv4Addr
/// [`NonZeroU64`]: std::num::NonZeroU64
pub trait IntKey: Copy {
    /// The underlying integer that will be used as actual key.
    type Int: Int;
    /// Converts the key into the underlying integer.
    ///
    /// [`IntMap`] assumes that this is a cheap operation and that two different values
    /// don't return the same integer.
    fn into_int(self) -> Self::Int;
}

macro_rules! impl_int_key_for_int {
    ($self:ident) => {
        impl IntKey for $self {
            type Int = $self;

            fn into_int(self) -> Self::Int {
                self
            }
        }
    };
}

impl_int_key_for_int!(u8);
impl_int_key_for_int!(u16);
impl_int_key_for_int!(u32);
impl_int_key_for_int!(u64);
impl_int_key_for_int!(u128);
impl_int_key_for_int!(usize);
impl_int_key_for_int!(i8);
impl_int_key_for_int!(i16);
impl_int_key_for_int!(i32);
impl_int_key_for_int!(i64);
impl_int_key_for_int!(i128);
impl_int_key_for_int!(isize);

macro_rules! impl_int_key_for_non_zero_int {
    ($non_zero_int:ident as $int:ident) => {
        impl IntKey for $non_zero_int {
            type Int = $int;

            fn into_int(self) -> Self::Int {
                self.get()
            }
        }
    };
}

impl_int_key_for_non_zero_int!(NonZeroU8 as u8);
impl_int_key_for_non_zero_int!(NonZeroU16 as u16);
impl_int_key_for_non_zero_int!(NonZeroU32 as u32);
impl_int_key_for_non_zero_int!(NonZeroU64 as u64);
impl_int_key_for_non_zero_int!(NonZeroUsize as usize);
impl_int_key_for_non_zero_int!(NonZeroI8 as i8);
impl_int_key_for_non_zero_int!(NonZeroI16 as i16);
impl_int_key_for_non_zero_int!(NonZeroI32 as i32);
impl_int_key_for_non_zero_int!(NonZeroI64 as i64);
impl_int_key_for_non_zero_int!(NonZeroIsize as isize);

impl<K: IntKey> IntKey for Wrapping<K> {
    type Int = K::Int;

    fn into_int(self) -> Self::Int {
        self.0.into_int()
    }
}

impl IntKey for std::net::Ipv4Addr {
    type Int = u32;

    fn into_int(self) -> Self::Int {
        // Copied from Ipv4Addr::to_bits, which does not exist for our MSRV
        u32::from_be_bytes(self.octets())
    }
}

impl IntKey for std::net::Ipv6Addr {
    type Int = u128;

    fn into_int(self) -> Self::Int {
        // Copied from Ipv6Addr::to_bits, which does not exist for our MSRV
        u128::from_be_bytes(self.octets())
    }
}
