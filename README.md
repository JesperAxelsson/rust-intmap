# rust-intmap
Specialized hashmap for u64 keys

Might be missing some functionality but you can remove, add, get and clear for now.

Performace compared to the standard hashmap: 

````
test tests::u64_get_built_in    ... bench:      43,925 ns/iter (+/- 2,712)
test tests::u64_get_intmap      ... bench:       1,099 ns/iter (+/- 136)
test tests::u64_insert_built_in ... bench:      64,717 ns/iter (+/- 2,830)
test tests::u64_insert_intmap   ... bench:      28,419 ns/iter (+/- 983)
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
