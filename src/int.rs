/// A primitive unsigned integer that can be used as underlying key for [`IntMap`].
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

pub trait SealedInt: Copy + PartialEq {
    fn calc_index(self, mod_mask: usize, prime: Self) -> usize;
}

macro_rules! impl_sealed_int_for_int {
    ($uint:ident) => {
        impl SealedInt for $uint {
            #[inline(always)]
            fn calc_index(self, mod_mask: usize, prime: Self) -> usize {
                let hash = prime.wrapping_mul(self);
                // Faster modulus
                (hash as usize) & mod_mask
            }
        }
    };
}

impl_sealed_int_for_int!(u8);
impl_sealed_int_for_int!(u16);
impl_sealed_int_for_int!(u32);
impl_sealed_int_for_int!(u64);
impl_sealed_int_for_int!(u128);
impl_sealed_int_for_int!(usize);
