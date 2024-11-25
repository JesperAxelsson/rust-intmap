use std::collections::HashMap;

use intmap::{IntKey, IntMap};
use intmap_integration_test_random_ops::{Ctor, Op, TestIntKey};
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_random_ops_u8(ctor in Ctor::<u8>::arb(), ops in Op::<u8>::arb_vec(200)) {
        test_random_ops(ctor, ops);
    }

    #[test]
    fn test_random_ops_u16(ctor in Ctor::<u16>::arb(), ops in Op::<u16>::arb_vec(200)) {
        test_random_ops(ctor, ops);
    }

    #[test]
    fn test_random_ops_u32(ctor in Ctor::<u32>::arb(), ops in Op::<u32>::arb_vec(200)) {
        test_random_ops(ctor, ops);
    }

    #[test]
    fn test_random_ops_u64(ctor in Ctor::<u64>::arb(), ops in Op::<u64>::arb_vec(200)) {
        test_random_ops(ctor, ops);
    }

    #[test]
    fn test_random_ops_u128(ctor in Ctor::<u128>::arb(), ops in Op::<u128>::arb_vec(200)) {
        test_random_ops(ctor, ops);
    }

    #[test]
    fn test_random_ops_usize(ctor in Ctor::<usize>::arb(), ops in Op::<usize>::arb_vec(200)) {
        test_random_ops(ctor, ops);
    }

    #[test]
    fn test_random_ops_i8(ctor in Ctor::<i8>::arb(), ops in Op::<i8>::arb_vec(200)) {
        test_random_ops(ctor, ops);
    }

    #[test]
    fn test_random_ops_i16(ctor in Ctor::<i16>::arb(), ops in Op::<i16>::arb_vec(200)) {
        test_random_ops(ctor, ops);
    }

    #[test]
    fn test_random_ops_i32(ctor in Ctor::<i32>::arb(), ops in Op::<i32>::arb_vec(200)) {
        test_random_ops(ctor, ops);
    }

    #[test]
    fn test_random_ops_i64(ctor in Ctor::<i64>::arb(), ops in Op::<i64>::arb_vec(200)) {
        test_random_ops(ctor, ops);
    }

    #[test]
    fn test_random_ops_i128(ctor in Ctor::<i128>::arb(), ops in Op::<i128>::arb_vec(200)) {
        test_random_ops(ctor, ops);
    }

    #[test]
    fn test_random_ops_isize(ctor in Ctor::<isize>::arb(), ops in Op::<isize>::arb_vec(200)) {
        test_random_ops(ctor, ops);
    }
}

// This test performs random operations on IntMap to ensure that no operation
// fails due to violated invariants. Also all mutable operations are performed
// on an reference implementation (HashMap). The elements of the final IntMap
// are compared with the elements of the reference implementation.
fn test_random_ops<K: TestIntKey + Ord>(ctor: Ctor<K>, ops: Vec<Op<K>>) {
    let (mut map, mut reference) = ctor.apply();
    assert_map(&map, &reference);

    for op in ops {
        op.apply(&mut map, &mut reference);
        assert_map(&map, &reference);
    }

    let mut map_values = map.iter().collect::<Vec<_>>();
    map_values.sort_by_key(|(key, _)| *key);

    let mut reference_values = reference.iter().map(|(&k, v)| (k, v)).collect::<Vec<_>>();
    reference_values.sort_by_key(|(key, _)| *key);

    assert_eq!(map_values, reference_values);
}

fn assert_map<K: IntKey>(map: &IntMap<K, u8>, reference: &HashMap<K, u8>) {
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
