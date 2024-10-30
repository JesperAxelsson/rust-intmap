use std::collections::HashMap;
use std::fmt::Debug;

use intmap::{IntKey, IntMap};
use proptest::collection::hash_map;
use proptest::prelude::*;
use serde::{de::DeserializeOwned, Serialize};

proptest! {
    #[test]
    fn test_roundtrip_u8(m in hash_map(any::<u8>(), any::<String>(), 0..20)) {
        test_roundtrip(m)?;
    }

    #[test]
    fn test_roundtrip_u16(m in hash_map(any::<u16>(), any::<String>(), 0..20)) {
        test_roundtrip(m)?;
    }

    #[test]
    fn test_roundtrip_u32(m in hash_map(any::<u32>(), any::<String>(), 0..20)) {
        test_roundtrip(m)?;
    }

    #[test]
    fn test_roundtrip_u64(m in hash_map(any::<u64>(), any::<String>(), 0..20)) {
        test_roundtrip(m)?;
    }

    #[test]
    fn test_roundtrip_u128(m in hash_map(any::<u128>(), any::<String>(), 0..20)) {
        test_roundtrip(m)?;
    }

    #[test]
    fn test_roundtrip_usize(m in hash_map(any::<usize>(), any::<String>(), 0..20)) {
        test_roundtrip(m)?;
    }

    #[test]
    fn test_roundtrip_i8(m in hash_map(any::<i8>(), any::<String>(), 0..20)) {
        test_roundtrip(m)?;
    }

    #[test]
    fn test_roundtrip_i16(m in hash_map(any::<i16>(), any::<String>(), 0..20)) {
        test_roundtrip(m)?;
    }

    #[test]
    fn test_roundtrip_i32(m in hash_map(any::<i32>(), any::<String>(), 0..20)) {
        test_roundtrip(m)?;
    }

    #[test]
    fn test_roundtrip_i64(m in hash_map(any::<i64>(), any::<String>(), 0..20)) {
        test_roundtrip(m)?;
    }

    #[test]
    fn test_roundtrip_i128(m in hash_map(any::<i128>(), any::<String>(), 0..20)) {
        test_roundtrip(m)?;
    }

    #[test]
    fn test_roundtrip_isize(m in hash_map(any::<isize>(), any::<String>(), 0..20)) {
        test_roundtrip(m)?;
    }
}

fn test_roundtrip<K>(m: HashMap<K, String>) -> Result<(), TestCaseError>
where
    K: IntKey + Serialize + DeserializeOwned + Debug,
{
    let im: IntMap<K, _> = m.into_iter().collect();
    let bytes = serde_json::to_vec(&im).unwrap();
    let im_copy = serde_json::from_slice(&bytes[..]).unwrap();
    prop_assert_eq!(im, im_copy);
    Ok(())
}
