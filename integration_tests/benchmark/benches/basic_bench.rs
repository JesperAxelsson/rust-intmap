use std::collections::HashMap;

use ahash::AHashMap;
use divan::{bench, black_box, Bencher};
use hashbrown::HashMap as BrownMap;
use indexmap::IndexMap;
use intmap::{Entry, IntMap};
use rustc_hash::{FxBuildHasher, FxHashMap};

const VEC_COUNT: usize = 10_000;

fn main() {
    divan::main();
}

// ********** Built in **********

#[bench]
fn u64_insert_built_in(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);
    let mut map: HashMap<u64, u64> = HashMap::with_capacity(data.len());

    bencher.bench_local(|| {
        map.clear();

        for s in data.iter() {
            black_box(map.insert(*s, *s));
        }
    });
}

#[bench]
fn u64_insert_without_capacity_built_in(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);

    bencher.bench_local(|| {
        let mut map: HashMap<u64, u64> = HashMap::new();

        for s in data.iter() {
            black_box(map.insert(*s, *s));
        }

        black_box(&map);
    });
}

#[bench]
fn u64_get_built_in(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);
    let mut map: HashMap<u64, u64> = HashMap::with_capacity(data.len());

    for s in data.iter() {
        black_box(map.insert(*s, *s));
    }

    bencher.bench_local(|| {
        for s in data.iter() {
            black_box(map.contains_key(s));
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
fn u64_insert_no_op(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);
    let mut map: HashMap<u64, u64, NoOpHasher> =
        HashMap::with_capacity_and_hasher(data.len(), NoOpHasher(0));

    bencher.bench_local(|| {
        map.clear();

        for s in data.iter() {
            black_box(map.insert(*s, *s));
        }
    });
}

#[bench]
fn u64_insert_without_capacity_no_op(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);

    bencher.bench_local(|| {
        let mut map: HashMap<u64, u64, NoOpHasher> = HashMap::with_hasher(NoOpHasher(0));

        for s in data.iter() {
            black_box(map.insert(*s, *s));
        }

        black_box(&map);
    });
}

#[bench]
fn u64_get_no_op(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);
    let mut map: HashMap<u64, u64, NoOpHasher> =
        HashMap::with_capacity_and_hasher(data.len(), NoOpHasher(0));

    for s in data.iter() {
        black_box(map.insert(*s, *s));
    }

    bencher.bench_local(|| {
        for s in data.iter() {
            black_box(map.contains_key(s));
        }
    });
}

// ********** HashBrown **********

#[bench]
fn u64_insert_brown(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);
    let mut map: BrownMap<u64, u64> = BrownMap::with_capacity(data.len());

    bencher.bench_local(|| {
        map.clear();

        for s in data.iter() {
            black_box(map.insert(*s, *s));
        }
    });
}

#[bench]
fn u64_insert_without_capacity_brown(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);

    bencher.bench_local(|| {
        let mut map: BrownMap<u64, u64> = BrownMap::new();

        for s in data.iter() {
            black_box(map.insert(*s, *s));
        }

        black_box(&map);
    });
}

#[bench]
fn u64_get_brown(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);
    let mut map: BrownMap<u64, u64> = BrownMap::with_capacity(data.len());

    for s in data.iter() {
        black_box(map.insert(*s, *s));
    }

    bencher.bench_local(|| {
        for s in data.iter() {
            black_box(map.contains_key(s));
        }
    });
}

// ********** Ahash **********

#[bench]
fn u64_insert_ahash(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);
    let mut map: AHashMap<u64, u64> = AHashMap::with_capacity(data.len());

    bencher.bench_local(|| {
        map.clear();

        for s in data.iter() {
            black_box(map.insert(*s, *s));
        }
    });
}

#[bench]
fn u64_insert_without_capacity_ahash(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);

    bencher.bench_local(|| {
        let mut map: AHashMap<u64, u64> = AHashMap::new();

        for s in data.iter() {
            black_box(map.insert(*s, *s));
        }

        black_box(&map);
    });
}

#[bench]
fn u64_get_ahash(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);
    let mut map: AHashMap<u64, u64> = AHashMap::with_capacity(data.len());

    for s in data.iter() {
        black_box(map.insert(*s, *s));
    }

    bencher.bench_local(|| {
        for s in data.iter() {
            black_box(map.contains_key(s));
        }
    });
}

// ********** FxHashMap **********

#[bench]
fn u64_insert_fxhashmap(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);
    let mut map: FxHashMap<u64, u64> =
        FxHashMap::with_capacity_and_hasher(data.len(), FxBuildHasher);

    bencher.bench_local(|| {
        map.clear();

        for s in data.iter() {
            black_box(map.insert(*s, *s));
        }
    });
}

#[bench]
fn u64_insert_without_capacity_fxhashmap(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);

    bencher.bench_local(|| {
        let mut map: FxHashMap<u64, u64> = FxHashMap::default();

        for s in data.iter() {
            black_box(map.insert(*s, *s));
        }

        black_box(&map);
    });
}

#[bench]
fn u64_get_fxhashmap(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);
    let mut map: FxHashMap<u64, u64> =
        FxHashMap::with_capacity_and_hasher(data.len(), FxBuildHasher);

    for s in data.iter() {
        black_box(map.insert(*s, *s));
    }

    bencher.bench_local(|| {
        for s in data.iter() {
            black_box(map.contains_key(s));
        }
    });
}

// ********** IndexMap **********

#[bench]
fn u64_insert_indexmap(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);
    let mut map: IndexMap<u64, u64> = IndexMap::with_capacity(data.len());

    bencher.bench_local(|| {
        map.clear();

        for s in data.iter() {
            black_box(map.insert(*s, *s));
        }
    });
}

#[bench]
fn u64_get_indexmap(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);
    let mut map: IndexMap<u64, u64> = IndexMap::with_capacity(data.len());

    for s in data.iter() {
        black_box(map.insert(*s, *s));
    }

    bencher.bench_local(|| {
        for s in data.iter() {
            black_box(map.contains_key(s));
        }
    });
}

// ********** Intmap **********

#[bench]
fn u64_insert_intmap(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);
    let mut map: IntMap<u64, u64> = IntMap::with_capacity(data.len());

    bencher.bench_local(|| {
        map.clear();

        for s in data.iter() {
            black_box(map.insert(*s, *s));
        }
    });
}

#[bench]
fn u64_insert_intmap_checked(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);
    let mut map: IntMap<u64, u64> = IntMap::with_capacity(data.len());

    bencher.bench_local(|| {
        map.clear();

        for s in data.iter() {
            black_box(map.insert_checked(*s, *s));
        }
    });
}

#[bench]
fn u64_insert_intmap_entry(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);

    let mut map: IntMap<u64, u64> = IntMap::with_capacity(data.len());

    bencher.bench_local(|| {
        map.clear();

        for s in data.iter() {
            black_box(match map.entry(*s) {
                Entry::Occupied(_) => panic!("unexpected while insert, i = {}", s),
                Entry::Vacant(entry) => entry.insert(*s),
            });
        }
    });
}

#[bench]
fn u64_insert_without_capacity_intmap(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);

    bencher.bench_local(|| {
        let mut map: IntMap<u64, u64> = IntMap::new();

        for s in data.iter() {
            black_box(map.insert(*s, *s));
        }

        black_box(&map);
    });
}

#[bench]
fn u64_resize_intmap(bencher: Bencher) {
    bencher.bench_local(|| {
        let mut map: IntMap<u64, u64> = IntMap::new();
        map.reserve(VEC_COUNT);
        black_box(&map);
    });
}

#[bench]
fn u64_get_intmap(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT, 4242);

    let mut map: IntMap<u64, u64> = IntMap::with_capacity(data.len());
    for s in data.iter() {
        map.insert(*s, *s);
    }

    bencher.bench_local(|| {
        for s in data.iter() {
            black_box(map.contains_key(*s));
        }
    });
}

#[bench]
fn u64_eq_intmap(bencher: Bencher) {
    let data1 = get_random_range(VEC_COUNT, 4242);
    let data2 = get_random_range(VEC_COUNT, 1212);
    let data3 = get_random_range(VEC_COUNT + 1, 1212);

    let mut map1: IntMap<u64, u64> = IntMap::with_capacity(data1.len());
    for s in data1.iter() {
        map1.insert(*s, *s);
    }
    let mut map2: IntMap<u64, u64> = IntMap::with_capacity(data2.len());
    for s in data2.iter() {
        map2.insert(*s, *s);
    }
    let mut map3: IntMap<u64, u64> = IntMap::with_capacity(data3.len());
    for s in data3.iter() {
        map3.insert(*s, *s);
    }
    let map4 = map3.clone();

    bencher.bench_local(|| {
        for _ in 0..100 {
            black_box(map1 == map2);
            black_box(map2 == map3);
            black_box(map3 == map4);
        }
    });
}

// ********** Misc **********

fn get_random_range(count: usize, seed: u64) -> Vec<u64> {
    use rand::prelude::StdRng;
    use rand::{Rng, SeedableRng};

    let mut vec = Vec::new();
    let mut rng = StdRng::seed_from_u64(seed);

    for _ in 0..count {
        vec.push(rng.gen::<u64>());
    }

    vec.sort();
    vec.dedup();

    vec
}
