use std::ops::RangeInclusive;
use std::{collections::HashMap, fmt::Debug, hash::Hash};

use intmap::{IntKey, IntMap};
use proptest::collection::vec;
use proptest::prelude::*;

pub trait TestIntKey: IntKey + Debug + Eq + Hash + 'static {
    type Range: Strategy<Value = Self>;

    fn small_range() -> Self::Range;
    fn total_range() -> Self::Range;
}

macro_rules! impl_test_int_for_int {
    ($int:ident) => {
        impl TestIntKey for $int {
            type Range = RangeInclusive<Self>;

            fn small_range() -> Self::Range {
                0..=10
            }
            fn total_range() -> Self::Range {
                0..=(Self::MAX)
            }
        }
    };
}

impl_test_int_for_int!(u8);
impl_test_int_for_int!(u16);
impl_test_int_for_int!(u32);
impl_test_int_for_int!(u64);
impl_test_int_for_int!(u128);
impl_test_int_for_int!(usize);
impl_test_int_for_int!(i8);
impl_test_int_for_int!(i16);
impl_test_int_for_int!(i32);
impl_test_int_for_int!(i64);
impl_test_int_for_int!(isize);
impl_test_int_for_int!(i128);

#[derive(Clone, Debug)]
pub struct Capacity(usize);

impl Capacity {
    const MAX: usize = 100;

    fn arb() -> impl Strategy<Value = Self> {
        (0usize..=Self::MAX).prop_map(Self)
    }
}

#[derive(Clone, Debug)]
pub struct LoadFactor(f32);

impl LoadFactor {
    fn arb() -> impl Strategy<Value = Self> {
        (0.1f32..=10.0f32).prop_map(Self)
    }
}

#[derive(Clone, Debug)]
pub struct Key<K>(K);

impl<K: TestIntKey> Key<K> {
    fn arb() -> impl Strategy<Value = Self> {
        prop_oneof![
            // Keys with high probability for collision
            10 => K::small_range(),
            // Totally arbitrary keys
            1 => K::total_range(),
        ]
        .prop_map(Self)
    }
}

#[derive(Clone, Debug)]
pub struct Value(u8);

impl Value {
    fn arb() -> impl Strategy<Value = Self> {
        any::<u8>().prop_map(Self)
    }
}

#[derive(Clone, Debug)]
pub struct Pairs<K>(Vec<(K, u8)>);

impl<K: TestIntKey> Pairs<K> {
    fn arb() -> impl Strategy<Value = Self> {
        vec(
            (Key::arb().prop_map(|k| k.0), Value::arb().prop_map(|v| v.0)),
            0usize..Capacity::MAX,
        )
        .prop_map(Self)
    }
}

#[derive(Clone, Debug)]
pub enum Ctor<K> {
    New,
    WithCapacity(Capacity),
    Default,
    FromIter(Pairs<K>),
}

impl<K: TestIntKey> Ctor<K> {
    pub fn arb() -> impl Strategy<Value = Self> {
        prop_oneof![
            Just(Self::New),
            Capacity::arb().prop_map(Self::WithCapacity),
            Just(Self::Default),
            Pairs::arb().prop_map(Self::FromIter),
        ]
    }

    pub fn apply(&self) -> (IntMap<K, u8>, HashMap<K, u8>) {
        match self {
            Self::New => (IntMap::new(), HashMap::new()),
            Self::WithCapacity(capacity) => (IntMap::with_capacity(capacity.0), HashMap::new()),
            Self::Default => (IntMap::default(), HashMap::new()),
            Self::FromIter(pairs) => (
                IntMap::from_iter(pairs.0.clone()),
                HashMap::from_iter(pairs.0.clone()),
            ),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Op<K> {
    SetLoadFactor(LoadFactor),
    GetLoadFactor,
    Reserve(Capacity),
    Insert((Key<K>, Value)),
    InsertChecked((Key<K>, Value)),
    Get(Key<K>),
    GetMut(Key<K>),
    Remove(Key<K>),
    ContainsKey(Key<K>),
    Clear,
    Retain(Value),
    IsEmpty,
    Iter,
    IterMut,
    Keys,
    Values,
    ValuesMut,
    Drain,
    Len,
    Load,
    LoadRate,
    Capacity,
    Collisions,
    Entry(Key<K>),
    EntryInsert((Key<K>, Value)),
    EntryRemove(Key<K>),
    Clone,
    Debug,
    Extend(Pairs<K>),
}

impl<K: TestIntKey> Op<K> {
    pub fn arb_vec(max_size: usize) -> impl Strategy<Value = Vec<Self>> {
        vec(Op::arb(), 0..=max_size)
    }

    pub fn arb() -> impl Strategy<Value = Self> {
        prop_oneof![
            1 => LoadFactor::arb().prop_map(Self::SetLoadFactor),
            10 => Just(Self::GetLoadFactor),
            1 => Capacity::arb().prop_map(Self::Reserve),
            50 => (Key::arb(), Value::arb()).prop_map(Self::Insert),
            10 => (Key::arb(), Value::arb()).prop_map(Self::InsertChecked),
            10 => Key::arb().prop_map(Self::Get),
            10 => Key::arb().prop_map(Self::GetMut),
            10 => Key::arb().prop_map(Self::Remove),
            10 => Key::arb().prop_map(Self::ContainsKey),
            1 => Just(Self::Clear),
            1 => Value::arb().prop_map(Self::Retain),
            1 => Just(Self::IsEmpty),
            1 => Just(Self::Iter),
            1 => Just(Self::IterMut),
            1 => Just(Self::Keys),
            1 => Just(Self::Values),
            1 => Just(Self::ValuesMut),
            1 => Just(Self::Drain),
            1 => Just(Self::Len),
            1 => Just(Self::Load),
            1 => Just(Self::LoadRate),
            1 => Just(Self::Capacity),
            1 => Just(Self::Collisions),
            10 => Key::arb().prop_map(Self::Entry),
            10 => (Key::arb(), Value::arb()).prop_map(Self::EntryInsert),
            10 => Key::arb().prop_map(Self::EntryRemove),
            1 => Just(Self::Clone),
            1 => Just(Self::Debug),
            1 => Pairs::arb().prop_map(Self::Extend),
        ]
    }

    pub fn apply(&self, map: &mut IntMap<K, u8>, reference: &mut HashMap<K, u8>) {
        match self {
            Self::SetLoadFactor(load_factor) => {
                map.set_load_factor(load_factor.0);
            }
            Self::GetLoadFactor => {
                map.get_load_factor();
            }
            Self::Reserve(additional) => {
                map.reserve(additional.0);
            }
            Self::Insert((key, value)) => {
                assert_eq!(map.insert(key.0, value.0), reference.insert(key.0, value.0));
            }
            Self::InsertChecked((key, value)) => {
                map.insert_checked(key.0, value.0);
                reference.entry(key.0).or_insert(value.0);
            }
            Self::Get(key) => {
                assert_eq!(map.get(key.0), reference.get(&key.0));
            }
            Self::GetMut(key) => {
                assert_eq!(map.get_mut(key.0), reference.get_mut(&key.0));
            }
            Self::Remove(key) => {
                assert_eq!(map.remove(key.0), reference.remove(&key.0));
            }
            Self::ContainsKey(key) => {
                assert_eq!(map.contains_key(key.0), reference.contains_key(&key.0));
            }
            Self::Clear => {
                map.clear();
                reference.clear();
            }
            Self::Retain(value) => {
                map.retain(|_, &v| v != value.0);
                reference.retain(|_, &mut v| v != value.0);
            }
            Self::IsEmpty => {
                assert_eq!(map.is_empty(), reference.is_empty())
            }
            Self::Iter => {
                assert_eq!(map.iter().count(), reference.len())
            }
            Self::IterMut => {
                assert_eq!(map.iter_mut().count(), reference.len())
            }
            Self::Keys => {
                assert_eq!(map.keys().count(), reference.len())
            }
            Self::Values => {
                assert_eq!(map.values().count(), reference.len())
            }
            Self::ValuesMut => {
                assert_eq!(map.values_mut().count(), reference.len())
            }
            Self::Drain => {
                assert_eq!(map.drain().count(), reference.drain().count());
            }
            Self::Len => {
                assert_eq!(map.len(), reference.len());
            }
            Self::Load => {
                map.load();
            }
            Self::LoadRate => {
                map.load_rate();
            }
            Self::Capacity => {
                map.capacity();
            }
            Self::Collisions => {
                map.collisions();
            }
            Self::Entry(key) => {
                map.entry(key.0);
            }
            Self::EntryInsert((key, value)) => {
                match map.entry(key.0) {
                    intmap::Entry::Occupied(mut entry) => {
                        entry.insert(value.0);
                    }
                    intmap::Entry::Vacant(entry) => {
                        entry.insert(value.0);
                    }
                }
                reference.insert(key.0, value.0);
            }
            Self::EntryRemove(key) => {
                if let intmap::Entry::Occupied(entry) = map.entry(key.0) {
                    entry.remove();
                }
                reference.remove(&key.0);
            }
            Self::Clone => {
                *map = map.clone();
            }
            Self::Debug => {
                let _string = format!("{map:?}");
            }
            Self::Extend(pairs) => {
                map.extend(pairs.0.clone());
                reference.extend(pairs.0.clone());
            }
        }
    }
}
