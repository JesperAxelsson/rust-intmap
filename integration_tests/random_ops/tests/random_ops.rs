use std::collections::HashMap;

use intmap::{Int, IntMap};
use intmap_integration_test_random_ops::{Ctor, Op};
use proptest::prelude::*;

// These tests performs random operations on IntMap to ensure that no operation
// fails due to violated invariants. Also all mutable operations are performed
// on an reference implementation (HashMap). The elements of the final IntMap
// are compared with the elements of the reference implementation.
proptest! {
    #[test]
    fn test_random_ops_u32(
        ctor in Ctor::<u32>::arb(),
        ops in Op::<u32>::arb_vec(200),
    ) {
        let (mut map, mut reference) = ctor.apply();
        assert_map(&map, &reference);

        for op in ops {
            op.apply(&mut map, &mut reference);
            assert_map(&map, &reference);
        }

        let mut map_values = map.iter().collect::<Vec<_>>();
        map_values.sort_by_key(|(&key, _)| key);

        let mut reference_values = reference.iter().collect::<Vec<_>>();
        reference_values.sort_by_key(|(&key, _)| key);

        assert_eq!(map_values, reference_values);
    }

    #[test]
    fn test_random_ops_u64(
        ctor in Ctor::<u64>::arb(),
        ops in Op::<u64>::arb_vec(200),
    ) {
        let (mut map, mut reference) = ctor.apply();
        assert_map(&map, &reference);

        for op in ops {
            op.apply(&mut map, &mut reference);
            assert_map(&map, &reference);
        }

        let mut map_values = map.iter().collect::<Vec<_>>();
        map_values.sort_by_key(|(&key, _)| key);

        let mut reference_values = reference.iter().collect::<Vec<_>>();
        reference_values.sort_by_key(|(&key, _)| key);

        assert_eq!(map_values, reference_values);
    }
}

fn assert_map<I: Int>(map: &IntMap<u8, I>, reference: &HashMap<I, u8>) {
    let debug = false;
    if debug {
        println!(
            "IntMap len={} capacity={} load={} load_rate={}",
            map.len(),
            map.capacity(),
            map.load(),
            map.load_rate(),
        );
    }

    map.assert_count();
    assert_eq!(map.len(), reference.len());
}
