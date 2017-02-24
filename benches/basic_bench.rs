#![feature(test)]

extern crate intmap;
extern crate rand;
extern crate test;

use intmap::IntMap;
use intmap::IntMap2;
use std::collections::HashMap;
use std::collections::BTreeMap;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;


    const VEC_COUNT: usize = 1000;


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
    fn u64_get_built_in(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        let mut map: HashMap<&u64, &u64>  = HashMap::with_capacity(data.len());

        for s in data.iter() {
            test::black_box({
                map.insert(s, s);
            });
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(s);
                    
                });
            }
        });
    }

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
    fn u64_get_intmap(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        
        let mut map = IntMap::with_capacity(data.len());
        for s in data.iter() {            
            map.insert(*s, s);
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(*s);
                    
                });
            }
        });
    }


    #[bench]
    fn u64_insert_intmap2(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        
        let mut map = IntMap2::with_capacity(data.len());
        
        let mut cols = map.collisions();
        let mut vec = Vec::new();

        for c in cols.iter() {
            vec.push(c.clone());
        
        }
        vec.sort();
        println!("Collisions: {}", vec.len());
        for v in vec.iter() {
            println!("col: {:?}", v);
        }


        b.iter(|| {
            map.clear();
            for s in data.iter() {
                test::black_box(map.insert(*s, s));
            }
        });
    }


    #[bench]
    fn u64_get_intmap2(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        
        let mut map = IntMap2::with_capacity(data.len());
        for s in data.iter() {            
            map.insert(*s, s);
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(*s);
                    
                });
            }
        });
    }



        #[bench]
    fn u64_insert_btree(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        
        let mut map = BTreeMap::new();

        b.iter(|| {
            map.clear();
            for s in data.iter() {
                test::black_box(map.insert(s, s));
            }
        });
    }


    #[bench]
    fn u64_get_btree(b: &mut Bencher) {
        let data = get_random_range(VEC_COUNT);
        
        let mut map = BTreeMap::new();
        for s in data.iter() {            
            map.insert(s, s);
        }

        b.iter(|| {
            for s in data.iter() {
                test::black_box({
                    map.contains_key(s);
                    
                });
            }
        });
    }


    fn get_random_range(count: usize) -> Vec<u64> {
        use rand::{Rng, SeedableRng, StdRng};
        
        let mut vec = Vec::new();

        let seed: &[_] = &[4, 2, 4, 2];
        let mut rng: StdRng = SeedableRng::from_seed(seed);

        for _ in 0..count {
            vec.push(rng.gen::<u64>());
        }

        vec.sort();
        vec.dedup();
        
        vec    
    }
}
