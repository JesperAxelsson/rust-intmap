use std::fmt::Debug;

/// A primitive integer that can be used as underlying key for [`IntMap`].
///
/// Note that this is a sealed trait that cannot be implemented externally. Consider implementing
/// [`IntKey`] instead.
///
/// [`IntMap`]: crate::IntMap
/// [`IntKey`]: crate::IntKey
pub trait Int: SealedInt {}

impl Int for u8 {}
impl Int for u16 {}
impl Int for u32 {}
impl Int for u64 {}
impl Int for u128 {}
impl Int for usize {}
impl Int for i8 {}
impl Int for i16 {}
impl Int for i32 {}
impl Int for i64 {}
impl Int for i128 {}
impl Int for isize {}

pub trait SealedInt: Copy + PartialEq + Debug + SerdeInt {
    fn calc_index(self, mod_mask: usize) -> usize;
}

#[cfg(not(feature = "serde"))]
pub trait SerdeInt {}

#[cfg(feature = "serde")]
pub trait SerdeInt: serde::Serialize + for<'de> serde::Deserialize<'de> {}

macro_rules! impl_sealed_int_for_int_with_highest_prime {
    ($uint:ident, $prime:expr) => {
        impl SealedInt for $uint {
            #[inline(always)]
            fn calc_index(self, mod_mask: usize) -> usize {
                let hash = $prime.wrapping_mul(self);
                // Faster modulus
                (hash as usize) & mod_mask
            }
        }

        impl SerdeInt for $uint {}
    };
}

macro_rules! impl_sealed_int_for_int_with_cast {
    ($int:ident as $uint:ident) => {
        impl SealedInt for $int {
            #[inline(always)]
            fn calc_index(self, mod_mask: usize) -> usize {
                (self as $uint).calc_index(mod_mask)
            }
        }

        impl SerdeInt for $int {}
    };
}

// Source: https://t5k.org/lists/2small/
// Checked with: https://www.numberempire.com/primenumbers.php
const U8_PRIME_MAX: u8 = u8::MAX - 4; // 251
const U16_PRIME_MAX: u16 = u16::MAX - 14; // 65521
const U32_PRIME_MAX: u32 = u32::MAX - 4; // 4294967291
const U64_PRIME_MAX: u64 = u64::MAX - 58; // 18446744073709551557
const U128_PRIME_MAX: u128 = u128::MAX - 158; // 340282366920938463463374607431768211297

impl_sealed_int_for_int_with_highest_prime!(u8, U8_PRIME_MAX);
impl_sealed_int_for_int_with_highest_prime!(u16, U16_PRIME_MAX);
impl_sealed_int_for_int_with_highest_prime!(u32, U32_PRIME_MAX);
impl_sealed_int_for_int_with_highest_prime!(u64, U64_PRIME_MAX);
impl_sealed_int_for_int_with_highest_prime!(u128, U128_PRIME_MAX);

#[cfg(target_pointer_width = "16")]
impl_sealed_int_for_int_with_cast!(usize as u16);
#[cfg(target_pointer_width = "32")]
impl_sealed_int_for_int_with_cast!(usize as u32);
#[cfg(target_pointer_width = "64")]
impl_sealed_int_for_int_with_cast!(usize as u64);

impl_sealed_int_for_int_with_cast!(i8 as u8);
impl_sealed_int_for_int_with_cast!(i16 as u16);
impl_sealed_int_for_int_with_cast!(i32 as u32);
impl_sealed_int_for_int_with_cast!(i64 as u64);
impl_sealed_int_for_int_with_cast!(i128 as u64);
impl_sealed_int_for_int_with_cast!(isize as usize);
