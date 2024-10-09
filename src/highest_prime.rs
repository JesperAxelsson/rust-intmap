pub trait HighestPrime {
    fn highest_prime() -> Self;
}

impl HighestPrime for u64 {
    #[inline(always)]
    fn highest_prime() -> Self {
        18_446_744_073_709_551_557
    }
}

impl HighestPrime for u32 {
    #[inline(always)]
    fn highest_prime() -> Self {
        2_147_483_647
    }
}

impl HighestPrime for u16 {
    #[inline(always)]
    fn highest_prime() -> Self {
        65_521
    }
}

impl HighestPrime for u8 {
    #[inline(always)]
    fn highest_prime() -> Self {
        251
    }
}
