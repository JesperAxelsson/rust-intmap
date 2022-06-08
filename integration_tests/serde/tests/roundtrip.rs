use intmap::IntMap;
use proptest::collection::hash_map;
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_roundtrip(m in hash_map(any::<u64>(), any::<String>(), 0..20)) {
        let im: IntMap<_> = m.into_iter().collect();
        let bytes = serde_json::to_vec(&im).unwrap();
        let im_copy = serde_json::from_slice(&bytes[..]).unwrap();
        prop_assert_eq!(im, im_copy);
    }
}
