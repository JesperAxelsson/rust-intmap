
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

        assert_eq!(map.len(), count);
        assert!(map.assert_count());

        for s in data.iter() {
            assert_eq!(*map.get(*s).unwrap(), *s, "intmap get failed! key: {:?}", s);
        }

        assert_eq!(map.len(), count);

        for s in data.iter() {
            assert!(map.contains_key(*s), "intmap contains_key failed! key: {:?}", s);
        }

        assert_eq!(map.len(), count);

        for s in data.iter() {
            let val = map.remove(*s).unwrap();
            assert_eq!(val, *s, "intmap remove failed! key: {:?}", s);
        }

        assert_eq!(map.len(), 0);
        assert!(map.assert_count());
    }

    #[test]
    fn add_string() {
        let mut map = IntMap::new();

        for i in 0..20_000 {
            map.insert(i, format!("item: {:?}", i));
        }
    }

    #[test]
    fn single_add_get() {
        let mut map: IntMap<u64> = IntMap::new();
        map.insert(21, 42);
        let val = map.get(21);
        assert!(val.is_some());
        assert_eq!(*val.unwrap(), 42);
    }

    #[test]
    fn map_iter() {
        let count = 20_000;
        let mut map: IntMap<u64> = IntMap::new();

        for i in 0..count {
            map.insert(i, i);
        }

        for (k, v) in map.iter() {
            assert_eq!(*k, *v);
        }
    }

    #[test]
    fn map_iter_keys() {
        let count = 20_000;
        let data: Vec<_> = (0..count).collect();
        let mut map: IntMap<u64> = IntMap::new();

        for i in 0..count {
            map.insert(i, i);
        }

        for k in map.keys() {
            assert_eq!(*k, data[*k as usize]);
        }
    }

    #[test]
    fn map_iter_values() {
        let count = 20_000;
        let data: Vec<_> = (0..count).collect();
        let mut map: IntMap<u64> = IntMap::new();

        for i in 0..count {
            map.insert(i, i);
        }

        for v in map.values() {
            assert_eq!(*v, data[*v as usize]);
        }
    }

    #[test]
    fn map_mut_iter() {
        let count = 20_000;
        let mut map: IntMap<u64> = IntMap::new();

        for i in 0..count {
            map.insert(i, i);
        }

        for kv in map.iter_mut() {
            *kv.1 += 1;
        }

        for n in 0..count {
            assert_eq!(n+1, *map.get(n).expect("Failed to get number!"));
        }

    }

    #[test]
    fn map_iter_empty() {
        let mut map: IntMap<u64> = IntMap::new();
        map.clear();

        for kv in map.iter() {
            println!("Not printing: {:?}", kv);
        }
    }

    #[test]
    fn map_mut_iter_empty() {
        let mut map: IntMap<u64> = IntMap::new();
        map.clear();

        for kv in map.iter_mut() {
            *kv.1 += 1;
        }
    }
}
