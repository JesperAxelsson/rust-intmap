[![crates.io](https://img.shields.io/crates/v/intmap.svg)](https://crates.io/crates/intmap)

# rust-intmap
Specialized hashmap for integer keys.

> [!WARNING]  
> Be aware that no effort is made against DoS attacks.

Benchmarks were performed on an AMD Ryzen 9 3900X running Manjaro with kernel version 6.6.40. Please remember to perform your own benchmarks if performance is important for your application.

Performance of 10 000 elements compared to some other common hash tables:

```txt
basic_bench                              fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ u64_get_ahash                         29.57 µs      │ 38.53 µs      │ 29.73 µs      │ 29.88 µs      │ 100     │ 100
├─ u64_get_brown                         35.66 µs      │ 47.53 µs      │ 35.93 µs      │ 36.24 µs      │ 100     │ 100
├─ u64_get_built_in                      123.4 µs      │ 133.7 µs      │ 124.3 µs      │ 124.6 µs      │ 100     │ 100
├─ u64_get_indexmap                      111.9 µs      │ 148.4 µs      │ 112 µs        │ 112.8 µs      │ 100     │ 100
├─ u64_get_intmap                        34.42 µs      │ 54.78 µs      │ 35.47 µs      │ 36.19 µs      │ 100     │ 100
├─ u64_get_no_op                         17.38 µs      │ 20.38 µs      │ 17.62 µs      │ 17.7 µs       │ 100     │ 100
├─ u64_insert_ahash                      64.11 µs      │ 137.8 µs      │ 120 µs        │ 115.3 µs      │ 100     │ 100
├─ u64_insert_brown                      61.57 µs      │ 103 µs        │ 88.73 µs      │ 85.99 µs      │ 100     │ 100
├─ u64_insert_built_in                   170.3 µs      │ 231.5 µs      │ 222.8 µs      │ 220.1 µs      │ 100     │ 100
├─ u64_insert_indexmap                   164.6 µs      │ 186.1 µs      │ 168.1 µs      │ 168.6 µs      │ 100     │ 100
├─ u64_insert_intmap                     64.91 µs      │ 208.8 µs      │ 65.54 µs      │ 67.49 µs      │ 100     │ 100
├─ u64_insert_intmap_checked             61.93 µs      │ 215.5 µs      │ 62.42 µs      │ 64.33 µs      │ 100     │ 100
├─ u64_insert_intmap_entry               71.37 µs      │ 206.9 µs      │ 72.2 µs       │ 73.89 µs      │ 100     │ 100
├─ u64_insert_no_op                      54.26 µs      │ 102.2 µs      │ 95.23 µs      │ 93.72 µs      │ 100     │ 100
├─ u64_insert_without_capacity_ahash     155.7 µs      │ 166.2 µs      │ 159 µs        │ 159.2 µs      │ 100     │ 100
├─ u64_insert_without_capacity_brown     159.3 µs      │ 223.3 µs      │ 214.2 µs      │ 210.8 µs      │ 100     │ 100
├─ u64_insert_without_capacity_built_in  387.2 µs      │ 521.5 µs      │ 495 µs        │ 487.5 µs      │ 100     │ 100
├─ u64_insert_without_capacity_intmap    744.4 µs      │ 772.3 µs      │ 752.6 µs      │ 753.8 µs      │ 100     │ 100
├─ u64_insert_without_capacity_no_op     160.3 µs      │ 178 µs        │ 161.7 µs      │ 164.9 µs      │ 100     │ 100
```
# Breaking Changes

Breaking changes are documented in the [changelog](CHANGELOG.md).

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
I use a specialized hash function for integers which multiplies the key with their largest prime. By keeping the internal cache a power 2 you can avoid the expensive modulus operator as mentioned in [this Stack Overflow post](http://stackoverflow.com/questions/6670715/mod-of-power-2-on-bitwise-operators). The hash function looks like this:

```rust
#[inline]
fn hash_u64(seed: u64) -> u64 {
    let a = 18446744073709551611u64;
    let val = a.wrapping_mul(seed);
    val
}
```
