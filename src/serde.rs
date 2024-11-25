use crate::{IntKey, IntMap};
use serde::{
    de::{Deserializer, MapAccess, Visitor},
    ser::SerializeMap,
    Deserialize, Serialize, Serializer,
};

impl<K, V> Serialize for IntMap<K, V>
where
    K: IntKey + Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self.iter() {
            map.serialize_entry(&k, v)?;
        }
        map.end()
    }
}

impl<'de, K, V> Deserialize<'de> for IntMap<K, V>
where
    K: IntKey + Deserialize<'de>,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(IntMapVisitor::new())
    }
}

struct IntMapVisitor<K: IntKey, V> {
    marker: std::marker::PhantomData<fn() -> IntMap<K, V>>,
}

impl<K: IntKey, V> IntMapVisitor<K, V> {
    fn new() -> Self {
        IntMapVisitor {
            marker: std::marker::PhantomData,
        }
    }
}

impl<'de, K, V> Visitor<'de> for IntMapVisitor<K, V>
where
    K: IntKey + Deserialize<'de>,
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
