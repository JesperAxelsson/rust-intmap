[![crates.io](https://img.shields.io/crates/v/intmap.svg)](https://crates.io/crates/intmap)

# rust-intmap
Specialized hashmap for u64 keys

Might be missing some functionality but you can remove, add, get and clear for now.

Be aware that no effort is made against DoS attacks.

Performace compared to the standard hashmap:

````
test tests::u64_get_built_in    ... bench:      22,320 ns/iter (+/- 446)
test tests::u64_get_intmap      ... bench:       2,959 ns/iter (+/- 148)
test tests::u64_insert_built_in ... bench:      27,666 ns/iter (+/- 1,230)
test tests::u64_insert_intmap   ... bench:      14,047 ns/iter (+/- 1,461)
````

# How to use
Simple example.

````rust
extern crate intmap;

use intmap::IntMap;

let mut map = IntMap::new();

for i in 0..20_000 {
    map.insert(i, format!("item: {:?}", i));
}
````

# How can it be so much faster?
I use a specialized hash function for u64 it multiplies the key with the largest prime for u64. By keeping the internal cache a power 2 you can avoid the expensive modulus operator as per http://stackoverflow.com/questions/6670715/mod-of-power-2-on-bitwise-operators.
````
#[inline]
fn hash_u64(seed: u64) -> u64 {
    let a = 11400714819323198549u64;
    let val = a.wrapping_mul(seed);
    val
}
````
