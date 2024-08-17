// ***************** Entry *********************

use crate::{Int, IntMap};

/// A view into a single entry in a [`IntMap`], which may either be vacant or occupied.
///
/// The entry can be constructed by calling [`IntMap::entry`] with a key. It allows inspection
/// and in-place manipulation of its value without repeated lookups.
pub enum Entry<'a, V: 'a, I = u64> {
    /// The entry is occupied.
    Occupied(OccupiedEntry<'a, V, I>),
    /// The entry is vacant.
    Vacant(VacantEntry<'a, V, I>),
}

impl<'a, V, I: Int> Entry<'a, V, I> {
    #[inline]
    pub(crate) fn new(key: I, int_map: &'a mut IntMap<V, I>) -> Self {
        let indices = Self::indices(key, int_map);

        match indices {
            Some((cache_ix, vals_ix)) => Entry::Occupied(OccupiedEntry {
                vals_ix,
                vals: &mut int_map.cache[cache_ix],
                count: &mut int_map.count,
            }),
            None => Entry::Vacant(VacantEntry { key, int_map }),
        }
    }

    fn indices(key: I, int_map: &IntMap<V, I>) -> Option<(usize, usize)> {
        if int_map.is_empty() {
            return None;
        }

        let cache_ix = I::calc_index(key, int_map.mod_mask);

        let vals = &int_map.cache[cache_ix];
        let vals_ix = { vals.iter() }
            .enumerate()
            .find_map(|(vals_ix, &(k, _))| (k == key).then(|| vals_ix))?;

        Some((cache_ix, vals_ix))
    }
}

/// A view into an occupied entry in a [`IntMap`]. It is part of the [`Entry`] enum.
pub struct OccupiedEntry<'a, V: 'a, I = u64> {
    // Index to vals, guaranteed to be valid
    vals_ix: usize,
    // Element of IntMap::cache, guaranteed to be non-empty
    vals: &'a mut Vec<(I, V)>,
    // IntMap::count, guaranteed to be non-zero
    count: &'a mut usize,
}

impl<'a, V, I: Int> OccupiedEntry<'a, V, I> {
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
pub struct VacantEntry<'a, V: 'a, I = u64> {
    key: I,
    int_map: &'a mut IntMap<V, I>,
}

impl<'a, V: 'a> VacantEntry<'a, V> {
    pub fn insert(self, value: V) -> &'a mut V {
        self.int_map.insert(self.key, value);
        return self.int_map.get_mut(self.key).unwrap();
    }
}
