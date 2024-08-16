use std::fmt::Debug;

pub trait SealedInt: Copy + PartialEq + Debug + SerdeInt {
    fn calc_index(key: Self, mod_mask: usize) -> usize;
}

#[cfg(not(feature = "serde"))]
pub trait SerdeInt {}

#[cfg(feature = "serde")]
pub trait SerdeInt: serde::Serialize + for<'de> serde::Deserialize<'de> {}

macro_rules! impl_sealed_int {
    ($int:ident, $prime:expr) => {
        impl SealedInt for $int {
            #[inline(always)]
            fn calc_index(key: Self, mod_mask: usize) -> usize {
                let hash = $prime.wrapping_mul(key);
                // Faster modulus
                (hash as usize) & mod_mask
            }
        }

        impl SerdeInt for $int {}
    };
}

impl_sealed_int! {
    u32,
    4294967291u32 // Largest prime for u32
}

impl_sealed_int! {
    u64,
    11400714819323198549u64 // Largest prime for u64
}
