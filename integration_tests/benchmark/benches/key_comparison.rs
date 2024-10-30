use divan::{bench, black_box, Bencher};
use intmap::{IntKey, IntMap};
use rand::distributions::{Distribution, Standard};

const VEC_COUNT: usize = 10_000;

fn main() {
    divan::main();
}

#[bench(types = [u16, u32, u64, u128])]
fn insert<K>(bencher: Bencher)
where
    K: IntKey + Ord + PartialEq,
    Standard: Distribution<K>,
{
    let data = get_random_range::<K>(VEC_COUNT);
    let mut map: IntMap<K, u64> = IntMap::with_capacity(data.len());

    bencher.bench_local(|| {
        map.clear();

        for (k, v) in data.iter() {
            black_box(map.insert(*k, *v));
        }
    });
}

#[bench(types = [u16, u32, u64, u128])]
fn insert_without_capacity<K>(bencher: Bencher)
where
    K: IntKey + Ord + PartialEq,
    Standard: Distribution<K>,
{
    let data = get_random_range::<K>(VEC_COUNT);

    bencher.bench_local(|| {
        let mut map: IntMap<K, u64> = IntMap::new();

        for (k, v) in data.iter() {
            black_box(map.insert(*k, *v));
        }

        black_box(&map);
    });
}

#[bench(types = [u16, u32, u64, u128])]
fn get<K>(bencher: Bencher)
where
    K: IntKey + Ord + PartialEq,
    Standard: Distribution<K>,
{
    let data = get_random_range::<K>(VEC_COUNT);

    let mut map: IntMap<K, u64> = IntMap::with_capacity(data.len());
    for (k, v) in data.iter() {
        map.insert(*k, *v);
    }

    bencher.bench_local(|| {
        for (k, _) in data.iter() {
            black_box(map.contains_key(*k));
        }
    });
}

fn get_random_range<K>(count: usize) -> Vec<(K, u64)>
where
    K: Ord + PartialEq,
    Standard: Distribution<K>,
{
    use rand::prelude::StdRng;
    use rand::{Rng, SeedableRng};

    let mut vec = Vec::new();
    let mut rng = StdRng::seed_from_u64(4242);

    for _ in 0..count {
        vec.push((rng.gen::<K>(), rng.gen::<u64>()));
    }

    vec.sort();
    vec.dedup();

    vec
}
