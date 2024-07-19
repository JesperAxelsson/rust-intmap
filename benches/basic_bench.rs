#![feature(test)]

extern crate intmap;
extern crate rand;
extern crate test;

extern crate indexmap;

use ahash::AHashMap;
use hashbrown::HashMap as BrownMap;
use indexmap::IndexMap;
use intmap::IntMap;
use std::collections::HashMap;
#[cfg(test)]
mod tests {
    use super::*;
    use intmap::Entry;
    use test::Bencher;

    const VEC_COUNT: usize = 10_000;

    // ********** HashBrown **********

    #[bench]
    fn u64_insert_brown(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map = BrownMap::with_capacity(data.len());

        b.iter(|| {
            map.clear();

            for s in data.iter() {
                test::black_box(map.insert(s, s));
            }
        });
    }

    #[bench]
    fn u64_insert_brown_without_capacity(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);

        b.iter(|| {
            let mut map = BrownMap::new();

            for s in data.iter() {
                test::black_box(map.insert(s, s));
            }

            test::black_box(&map);
        });
    }

    #[bench]
    fn u64_get_brown(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map: BrownMap<&u64, &u64> = BrownMap::with_capacity(data.len());

        for s in data.iter() {
            test::black_box(map.insert(s, s));
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(s);
                });
            }
        });
    }

    // ********** Ahash **********

    #[bench]
    fn u64_insert_ahash(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map = AHashMap::with_capacity(data.len());

        b.iter(|| {
            map.clear();

            for s in data.iter() {
                test::black_box(map.insert(s, s));
            }
        });
    }

    #[bench]
    fn u64_insert_ahash_without_capacity(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);

        b.iter(|| {
            let mut map = AHashMap::new();

            for s in data.iter() {
                test::black_box(map.insert(s, s));
            }

            test::black_box(&map);
        });
    }

    #[bench]
    fn u64_get_ahash(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map: AHashMap<&u64, &u64> = AHashMap::with_capacity(data.len());

        for s in data.iter() {
            test::black_box(map.insert(s, s));
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(s);
                });
            }
        });
    }

    // ********** No Op **********
    struct NoOpHasher(u64);
    impl std::hash::Hasher for NoOpHasher {
        fn finish(&self) -> u64 {
            self.0
        }

        fn write(&mut self, _: &[u8]) {
            unimplemented!()
        }

        fn write_u64(&mut self, i: u64) {
            self.0 = i;
        }
    }
    impl std::hash::BuildHasher for NoOpHasher {
        type Hasher = NoOpHasher;

        fn build_hasher(&self) -> Self::Hasher {
            NoOpHasher(0)
        }
    }
    #[bench]
    fn u64_insert_no_op(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map = HashMap::with_capacity_and_hasher(data.len(), NoOpHasher(0));

        b.iter(|| {
            map.clear();

            for s in data.iter() {
                test::black_box(map.insert(s, s));
            }
        });
    }

    #[bench]
    fn u64_insert_no_op_without_capacity(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);

        b.iter(|| {
            let mut map = HashMap::with_hasher(NoOpHasher(0));

            for s in data.iter() {
                test::black_box(map.insert(s, s));
            }

            test::black_box(&map);
        });
    }

    #[bench]
    fn u64_get_no_op(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map: HashMap<&u64, &u64, NoOpHasher> =
            HashMap::with_capacity_and_hasher(data.len(), NoOpHasher(0));

        for s in data.iter() {
            test::black_box(map.insert(s, s));
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(s);
                });
            }
        });
    }

    // ********** Built in **********

    #[bench]
    fn u64_insert_built_in(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map = HashMap::with_capacity(data.len());

        b.iter(|| {
            map.clear();

            for s in data.iter() {
                test::black_box(map.insert(s, s));
            }
        });
    }

    #[bench]
    fn u64_insert_built_in_without_capacity(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);

        b.iter(|| {
            let mut map = HashMap::new();

            for s in data.iter() {
                test::black_box(map.insert(s, s));
            }

            test::black_box(&map);
        });
    }

    #[bench]
    fn u64_get_built_in(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map: HashMap<&u64, &u64> = HashMap::with_capacity(data.len());

        for s in data.iter() {
            test::black_box(map.insert(s, s));
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(s);
                });
            }
        });
    }

    // ********** IndexMap **********

    #[bench]
    fn u64_insert_indexmap(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map = IndexMap::with_capacity(data.len());

        b.iter(|| {
            map.clear();

            for s in data.iter() {
                test::black_box(map.insert(s, s));
            }
        });
    }

    #[bench]
    fn u64_get_indexmap(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map: IndexMap<&u64, &u64> = IndexMap::with_capacity(data.len());

        for s in data.iter() {
            test::black_box(map.insert(s, s));
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(s);
                });
            }
        });
    }

    // ********** Intmap **********

    #[bench]
    fn u64_insert_intmap(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map = IntMap::with_capacity(data.len());

        b.iter(|| {
            map.clear();

            for s in data.iter() {
                test::black_box(map.insert(*s, s));
            }
        });
    }

    #[bench]
    fn u64_insert_intmap_checked(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map = IntMap::with_capacity(data.len());

        b.iter(|| {
            map.clear();

            for s in data.iter() {
                test::black_box(map.insert_checked(*s, s));
            }
        });
    }

    #[bench]
    fn u64_insert_intmap_entry(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);

        let mut map = IntMap::with_capacity(data.len());

        b.iter(|| {
            map.clear();

            for s in data.iter() {
                test::black_box(match map.entry(*s) {
                    Entry::Occupied(_) => panic!("unexpected while insert, i = {}", s),
                    Entry::Vacant(entry) => entry.insert(s),
                });
            }
        });
    }

    #[bench]
    fn u64_insert_intmap_without_capacity(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);

        b.iter(|| {
            let mut map = IntMap::new();

            for s in data.iter() {
                test::black_box(map.insert(*s, s));
            }

            test::black_box(&map);
        });
    }

    #[bench]
    fn u64_resize_intmap(b: &mut Bencher) {
        b.iter(|| {
            let mut map: IntMap<u64> = IntMap::new();
            map.reserve(VEC_COUNT);
            test::black_box(&map);
        });
    }

    #[bench]
    fn u64_get_intmap(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);

        let mut map = IntMap::with_capacity(data.len());
        for s in data.iter() {
            map.insert(*s, s);
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box(map.contains_key(*s));
            }
        });
    }

    // ********** Misc **********

    fn get_random_range(count: usize) -> Vec<u64> {
        use rand::prelude::StdRng;
        use rand::{Rng, SeedableRng};

        let mut vec = Vec::new();
        let mut rng = StdRng::seed_from_u64(4242);

        for _ in 0..count {
            vec.push(rng.gen::<u64>());
        }

        vec.sort();
        vec.dedup();

        vec
    }
}
