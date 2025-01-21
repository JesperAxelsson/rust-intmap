// ***************** Entry *********************

use crate::{int::SealedInt, IntKey, IntMap};

/// A view into a single entry in a [`IntMap`], which may either be vacant or occupied.
///
/// The entry can be constructed by calling [`IntMap::entry`] with a key. It allows inspection
/// and in-place manipulation of its value without repeated lookups.
pub enum Entry<'a, K: IntKey, V: 'a> {
    /// The entry is occupied.
    Occupied(OccupiedEntry<'a, K, V>),
    /// The entry is vacant.
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K: IntKey, V> Entry<'a, K, V> {
    #[inline]
    pub(crate) fn new(key: K, int_map: &'a mut IntMap<K, V>) -> Self {
        let (cache_ix, vals_ix) = Self::indices(key, int_map);

        match vals_ix {
            Some(vals_ix) => Entry::Occupied(OccupiedEntry {
                vals_ix,
                vals: &mut int_map.cache[cache_ix],
                count: &mut int_map.count,
            }),
            None => Entry::Vacant(VacantEntry {
                key,
                cache_ix,
                int_map,
            }),
        }
    }

    fn indices(key: K, int_map: &IntMap<K, V>) -> (usize, Option<usize>) {
        if int_map.is_empty() {
            // Returning 0 is okay because we'll increase the cache and recalculate the index if the
            // user calls `insert`.
            return (0, None);
        }

        let k = key.into_int();
        let cache_ix = k.calc_index(int_map.mod_mask, K::PRIME);

        let vals = &int_map.cache[cache_ix];
        let vals_ix = vals.iter().position(|(key, _)| key.into_int() == k);

        (cache_ix, vals_ix)
    }

    /// Ensures a value is in the entry by inserting the provided value if empty, and returns
    /// a mutable reference to the value in the entry.
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    /// Ensures a value is in the entry by inserting the result of the provided function if empty,
    /// and returns a mutable reference to the value in the entry.
    pub fn or_insert_with<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default()),
        }
    }

    /// Ensures a value is in the entry by inserting, if empty, the result of the provided function.
    pub fn or_insert_with_key<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce(K) -> V,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let d = default(entry.key);
                entry.insert(d)
            }
        }
    }
}

impl<'a, K: IntKey, V> Entry<'a, K, V>
where
    V: Default,
{
    /// Ensures a value is in the entry by inserting the default value if empty,
    /// and returns a mutable reference to the value in the entry.
    pub fn or_default(self) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(Default::default()),
        }
    }
}

/// A view into an occupied entry in a [`IntMap`]. It is part of the [`Entry`] enum.
pub struct OccupiedEntry<'a, K: IntKey, V: 'a> {
    // Index to vals, guaranteed to be valid
    vals_ix: usize,
    // Element of IntMap::cache, guaranteed to be non-empty
    vals: &'a mut Vec<(K, V)>,
    // IntMap::count, guaranteed to be non-zero
    count: &'a mut usize,
}

impl<'a, K: IntKey, V> OccupiedEntry<'a, K, V> {
    /// Gets a reference to the value in the entry.
    pub fn get(&self) -> &V {
        // Safety: We didn't modify the cache since we calculated the index
        &self.vals.get(self.vals_ix).unwrap().1
    }

    /// Gets a mutable reference to the value in the entry.
    pub fn get_mut(&mut self) -> &mut V {
        // Safety: We didn't modify the cache since we calculated the index
        &mut self.vals.get_mut(self.vals_ix).unwrap().1
    }

    /// Converts the entry into a mutable reference to the value in the entry with a
    /// lifetime bound to the [`IntMap`] itself.
    pub fn into_mut(self) -> &'a mut V {
        // Safety: We didn't modify the cache since we calculated the index
        &mut self.vals.get_mut(self.vals_ix).unwrap().1
    }

    /// Sets the value of the entry and returns the old value.
    pub fn insert(&mut self, value: V) -> V {
        std::mem::replace(&mut self.vals[self.vals_ix].1, value)
    }

    /// Removes the value out of the entry and returns it.
    pub fn remove(self) -> V {
        // Warning: We modify the cache here, so the index is now invalid
        *self.count -= 1;
        let kv = self.vals.swap_remove(self.vals_ix);

        kv.1
    }
}

/// A view into a vacant entry in a [`IntMap`]. It is part of the [`Entry`] enum.
pub struct VacantEntry<'a, K: IntKey, V: 'a> {
    key: K,
    cache_ix: usize,
    int_map: &'a mut IntMap<K, V>,
}

impl<'a, K: IntKey, V: 'a> VacantEntry<'a, K, V> {
    pub fn insert(mut self, value: V) -> &'a mut V {
        if self.int_map.increase_cache_if_needed() {
            // Recompute cache_ix for the new size.
            let k = self.key.into_int();
            self.cache_ix = k.calc_index(self.int_map.mod_mask, K::PRIME);
        }

        self.int_map.count += 1;
        let vals = &mut self.int_map.cache[self.cache_ix];
        vals.push((self.key, value));
        &mut vals.last_mut().unwrap().1
    }
}
