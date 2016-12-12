
extern crate rand;

extern crate intmap;

use intmap::IntMap;

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn intmap_get_insert_impl() {
        let count = 20_000;
        let data = get_random_range(count as usize);
        let mut map: IntMap<u64> = IntMap::new();

        println!("");
        println!("Starting test");

        for s in data.iter() {
            assert!(map.insert(*s, *s), "intmap insert failed! ix: {:?}", s);
        }

        assert_eq!(map.count(), count);
        assert!(map.assert_count());

        for s in data.iter() {
            assert_eq!(*map.get(*s).unwrap(), *s, "intmap get failed! key: {:?}", s);
        }

        assert_eq!(map.count(), count);

        for s in data.iter() {
            assert!(map.contains_key(*s), "intmap contains_key failed! key: {:?}", s);
        }

        assert_eq!(map.count(), count);

        for s in data.iter() {
            let val = map.remove(*s).unwrap();
            assert_eq!(val, *s, "intmap remove failed! key: {:?}", s);
        }

        assert_eq!(map.count(), 0);
        assert!(map.assert_count());
    }
}
