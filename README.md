[![crates.io](https://img.shields.io/crates/v/intmap.svg)](https://crates.io/crates/intmap)

# rust-intmap
Specialized hashmap for integer keys.

Might be missing some functionality but you can remove, add, get and clear for now.

> [!WARNING]  
> Be aware that no effort is made against DoS attacks.

Benchmarks were performed on an AMD Ryzen 9 3900X running Manjaro with kernel version 6.6.40. Please remember to perform your own benchmarks if performance is important for your application.

Performance compared to the standard hashmap and hashbrown:

```txt
test tests::u64_get_ahash                        ... bench:      33,612.79 ns/iter (+/- 1,338.91)
test tests::u64_get_brown                        ... bench:      34,459.40 ns/iter (+/- 563.82)
test tests::u64_get_built_in                     ... bench:     136,051.06 ns/iter (+/- 4,299.34)
test tests::u64_get_indexmap                     ... bench:     152,267.24 ns/iter (+/- 1,558.03)
test tests::u64_get_intmap                       ... bench:      30,576.66 ns/iter (+/- 1,642.70)
test tests::u64_get_no_op                        ... bench:      19,615.53 ns/iter (+/- 458.64)
test tests::u64_insert_ahash                     ... bench:     113,385.46 ns/iter (+/- 874.49)
test tests::u64_insert_ahash_without_capacity    ... bench:     258,242.55 ns/iter (+/- 54,208.86)
test tests::u64_insert_brown                     ... bench:     106,650.39 ns/iter (+/- 4,901.79)
test tests::u64_insert_brown_without_capacity    ... bench:     266,451.22 ns/iter (+/- 3,946.98)
test tests::u64_insert_built_in                  ... bench:     228,473.96 ns/iter (+/- 3,778.64)
test tests::u64_insert_built_in_without_capacity ... bench:     512,591.70 ns/iter (+/- 12,306.74)
test tests::u64_insert_indexmap                  ... bench:     218,257.40 ns/iter (+/- 72,881.46)
test tests::u64_insert_intmap                    ... bench:     101,611.15 ns/iter (+/- 4,536.83)
test tests::u64_insert_intmap_checked            ... bench:     107,639.17 ns/iter (+/- 1,862.78)
test tests::u64_insert_intmap_entry              ... bench:      94,155.26 ns/iter (+/- 1,357.05)
test tests::u64_insert_intmap_without_capacity   ... bench:     766,954.68 ns/iter (+/- 12,577.93)
test tests::u64_insert_no_op                     ... bench:      90,375.35 ns/iter (+/- 1,144.02)
test tests::u64_insert_no_op_without_capacity    ... bench:     190,528.64 ns/iter (+/- 5,733.59)
test tests::u64_resize_intmap                    ... bench:      55,155.88 ns/iter (+/- 648.32)
```
# Breaking Changes
2.0.0 - Changed behavior of `insert` to match std::HashMap. The old behavior is renamed to `insert_checked`. 

# How to use
Simple example:

```rust
extern crate intmap;

use intmap::IntMap;

let mut map = IntMap::new();

for i in 0..20_000 {
    map.insert(i, format!("item: {:?}", i));
}
```

# How can it be so much faster?
I use a specialized hash function for integers which multiplies the key with the their largest prime. By keeping the internal cache a power 2 you can avoid the expensive modulus operator as mentioned in [this Stack Overflow post](http://stackoverflow.com/questions/6670715/mod-of-power-2-on-bitwise-operators). The hash function looks like this:

```rust
#[inline]
fn hash_u64(seed: u64) -> u64 {
    let a = 18446744073709551611u64;
    let val = a.wrapping_mul(seed);
    val
}
```
