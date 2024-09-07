use divan::{bench, black_box, Bencher};
use indexmap::IndexMap;
use intmap::{Entry, IntMap};
use std::collections::HashMap;

const VEC_COUNT: usize = 10_000;

fn main() {
    divan::main();
}

// ********** Built in **********

#[bench]
fn u64_insert_built_in(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT);
    let mut map = HashMap::with_capacity(data.len());

    bencher.bench_local(|| {
        map.clear();

        for s in data.iter() {
            black_box(map.insert(s, s));
        }
    });
}

#[bench]
fn u64_insert_built_in_without_capacity(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT);

    bencher.bench_local(|| {
        let mut map = HashMap::new();

        for s in data.iter() {
            black_box(map.insert(s, s));
        }

        black_box(&map);
    });
}

#[bench]
fn u64_get_built_in(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT);
    let mut map: HashMap<&u64, &u64> = HashMap::with_capacity(data.len());

    for s in data.iter() {
        black_box(map.insert(s, s));
    }

    bencher.bench_local(|| {
        for s in data.iter() {
            black_box({
                map.contains_key(s);
            });
        }
    });
}

// ********** IndexMap **********

#[bench]
fn u64_insert_indexmap(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT);
    let mut map = IndexMap::with_capacity(data.len());

    bencher.bench_local(|| {
        map.clear();

        for s in data.iter() {
            black_box(map.insert(s, s));
        }
    });
}

#[bench]
fn u64_get_indexmap(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT);
    let mut map: IndexMap<&u64, &u64> = IndexMap::with_capacity(data.len());

    for s in data.iter() {
        black_box(map.insert(s, s));
    }

    bencher.bench_local(|| {
        for s in data.iter() {
            black_box({
                map.contains_key(s);
            });
        }
    });
}

// ********** Intmap **********

#[bench]
fn u64_insert_intmap(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT);
    let mut map = IntMap::with_capacity(data.len());

    bencher.bench_local(|| {
        map.clear();

        for s in data.iter() {
            black_box(map.insert(*s, s));
        }
    });
}

#[bench]
fn u64_insert_intmap_checked(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT);
    let mut map = IntMap::with_capacity(data.len());

    bencher.bench_local(|| {
        map.clear();

        for s in data.iter() {
            black_box(map.insert_checked(*s, s));
        }
    });
}

#[bench]
fn u64_insert_intmap_entry(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT);

    let mut map = IntMap::with_capacity(data.len());

    bencher.bench_local(|| {
        map.clear();

        for s in data.iter() {
            black_box(match map.entry(*s) {
                Entry::Occupied(_) => panic!("unexpected while insert, i = {}", s),
                Entry::Vacant(entry) => entry.insert(s),
            });
        }
    });
}

#[bench]
fn u64_insert_intmap_without_capacity(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT);

    bencher.bench_local(|| {
        let mut map = IntMap::new();

        for s in data.iter() {
            black_box(map.insert(*s, s));
        }

        black_box(&map);
    });
}

#[bench]
fn u64_resize_intmap(bencher: Bencher) {
    bencher.bench_local(|| {
        let mut map: IntMap<u64> = IntMap::new();
        map.reserve(VEC_COUNT);
        black_box(&map);
    });
}

#[bench]
fn u64_get_intmap(bencher: Bencher) {
    let data = get_random_range(VEC_COUNT);

    let mut map = IntMap::with_capacity(data.len());
    for s in data.iter() {
        map.insert(*s, s);
    }

    bencher.bench_local(|| {
        for s in data.iter() {
            black_box(map.contains_key(*s));
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
