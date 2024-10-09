use std::ops::{BitAnd, Sub};

use crate::{highest_prime::HighestPrime, IntMap};
use num_traits::{AsPrimitive, WrappingMul};
use serde::{
    de::{Deserializer, MapAccess, Visitor},
    ser::SerializeMap,
    Deserialize, Serialize, Serializer,
};

impl<K, V> Serialize for IntMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self.iter() {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

impl<'de, K, V> Deserialize<'de> for IntMap<K, V>
where
    K: AsPrimitive<usize>
        + BitAnd
        + Copy
        + Deserialize<'de>
        + HighestPrime
        + PartialEq
        + Sub
        + WrappingMul,
    <K as BitAnd>::Output: AsPrimitive<usize>,
    usize: AsPrimitive<K>,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(IntMapVisitor::new())
    }
}

struct IntMapVisitor<K, V> {
    marker: std::marker::PhantomData<fn() -> IntMap<K, V>>,
}

impl<K, V> IntMapVisitor<K, V> {
    fn new() -> Self {
        IntMapVisitor {
            marker: std::marker::PhantomData,
        }
    }
}

impl<'de, K, V> Visitor<'de> for IntMapVisitor<K, V>
where
    K: AsPrimitive<usize>
        + BitAnd
        + Copy
        + Deserialize<'de>
        + HighestPrime
        + PartialEq
        + Sub
        + WrappingMul,
    <K as BitAnd>::Output: AsPrimitive<usize>,
    usize: AsPrimitive<K>,
    V: Deserialize<'de>,
{
    type Value = IntMap<K, V>;

    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IntMap<{}>", std::any::type_name::<V>())
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = IntMap::with_capacity(access.size_hint().unwrap_or(0));

        while let Some((key, value)) = access.next_entry()? {
            map.insert(key, value);
        }

        Ok(map)
    }
}
