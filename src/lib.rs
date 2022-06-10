extern crate core;

#[cfg(feature = "serde")]
mod serde;

use core::iter::{IntoIterator, Iterator};

#[derive(Clone)]
pub struct IntMap<V> {
    cache: Vec<Vec<(u64, V)>>,
    size: u32,
    mod_mask: u64,
    count: usize,
}

impl<V> IntMap<V> {
    /// Creates a new IntMap.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64> = IntMap::new();
    /// ```
    pub fn new() -> Self {
        IntMap::with_capacity(4)
    }

    /// Creates a new IntMap with at least the given capacity, rounded
    /// to the next power of two.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64> = IntMap::with_capacity(20);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        let mut map = IntMap {
            cache: Vec::new(),
            size: 0,
            count: 0,
            mod_mask: 0,
        };

        increase_cache(&mut map.cache, &mut map.size, &mut map.mod_mask);

        while map.lim() < capacity {
            map.increase_cache();
        }

        map
    }

    /// Ensures that the IntMap has space for at least `additional` more elements
    pub fn reserve(&mut self, additional: usize) {
        let capacity = (self.count + additional).next_power_of_two();
        while self.lim() < capacity {
            self.increase_cache();
        }
    }

    /// Insert key/value into the IntMap if the key is not yet inserted.
    ///
    /// This function returns true if key/value were inserted and false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map = IntMap::new();
    /// assert!(map.insert(21, "Eat my shorts"));
    /// assert!(!map.insert(21, "Ay, caramba"));
    /// assert_eq!(map.get(21), Some(&"Eat my shorts"));
    /// ```
    pub fn insert(&mut self, key: u64, value: V) -> bool {
        let ix = self.calc_index(key);

        let ref mut vals = self.cache[ix];
        if vals.iter().any(|kv| kv.0 == key) {
            return false;
        }

        self.count += 1;
        vals.push((key, value));
        if (self.count & 4) == 4 {
            self.ensure_load_rate();
        }

        true
    }

    /// Get value from the IntMap.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64> = IntMap::new();
    /// map.insert(21, 42);
    /// let val = map.get(21);
    /// assert!(val.is_some());
    /// assert_eq!(*val.unwrap(), 42);
    /// assert!(map.contains_key(21));
    /// ```
    pub fn get(&self, key: u64) -> Option<&V> {
        let cache_ix = cache_index(self.mod_mask, key);

        let ref vals = self.cache[cache_ix];

        if vals.len() > 0 {
            return vals.iter().find_map(|kv| (kv.0 == key).then(|| &kv.1));
        } else {
            return None;
        }
    }

    /// Get mutable value from the IntMap.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64> = IntMap::new();
    /// map.insert(21, 42);
    ///
    /// assert_eq!(*map.get(21).unwrap(), 42);
    /// assert!(map.contains_key(21));
    ///
    /// {
    ///     let mut val = map.get_mut(21).unwrap();
    ///     *val+=1;
    /// }
    ///     assert_eq!(*map.get(21).unwrap(), 43);
    /// ```
    pub fn get_mut(&mut self, key: u64) -> Option<&mut V> {
        let ix = self.calc_index(key);

        let ref mut vals = self.cache[ix];

        if vals.len() > 0 {
            return vals
                .iter_mut()
                .find_map(|kv| (kv.0 == key).then(move || &mut kv.1));
        } else {
            return None;
        }
    }

    /// Remove value from the IntMap.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64> = IntMap::new();
    /// map.insert(21, 42);
    /// let val = map.remove(21);
    /// assert!(val.is_some());
    /// assert_eq!(val.unwrap(), 42);
    /// assert!(!map.contains_key(21));
    /// ```
    pub fn remove(&mut self, key: u64) -> Option<V> {
        let ix = self.calc_index(key);

        let ref mut vals = self.cache[ix];

        if vals.len() > 0 {
            for i in 0..vals.len() {
                let peek = vals[i].0;

                if peek == key {
                    self.count -= 1;
                    let kv = vals.swap_remove(i);
                    return Some(kv.1);
                }
            }

            return None;
        } else {
            return None;
        }
    }

    /// Returns true if key is in map.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64> = IntMap::new();
    /// map.insert(21, 42);
    /// assert!(map.contains_key(21));
    /// ```
    pub fn contains_key(&self, key: u64) -> bool {
        match self.get(key) {
            Some(_) => true,
            None => false,
        }
    }

    /// Removes all elements from map.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64> = IntMap::new();
    /// map.insert(21, 42);
    /// map.clear();
    /// assert_eq!(map.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        for vals in &mut self.cache {
            vals.clear();
        }

        self.count = 0;
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements such that `f(key, &value)` returns false.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64> = IntMap::new();
    /// map.insert(1, 11);
    /// map.insert(2, 12);
    /// map.insert(4, 13);
    ///
    /// // retain only the odd values
    /// map.retain(|k, v| *v % 2 == 1);
    ///
    /// assert_eq!(map.len(), 2);
    /// assert!(map.contains_key(1));
    /// assert!(map.contains_key(4));
    /// ```
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(u64, &V) -> bool,
    {
        let mut removed = 0;
        for vals in &mut self.cache {
            vals.retain(|(k, v)| {
                let keep = (f)(*k, v);
                if !keep {
                    removed += 1;
                }
                keep
            });
        }

        self.count -= removed;
    }

    /// Returns true if map is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64> = IntMap::new();
    /// map.insert(21, 42);
    /// assert!(!map.is_empty());
    /// map.remove(21);
    /// assert!(map.is_empty());
    /// ```
    pub fn is_empty(&mut self) -> bool {
        self.count == 0
    }

    //**** Iterators *****

    pub fn iter(&self) -> Iter<u64, V> {
        Iter::new(&self.cache)
    }

    pub fn iter_mut(&mut self) -> IterMut<u64, V> {
        IterMut::new(&mut self.cache)
    }

    pub fn keys(&self) -> Keys<u64, V> {
        Keys { inner: self.iter() }
    }

    pub fn values(&self) -> Values<u64, V> {
        Values { inner: self.iter() }
    }

    pub fn values_mut(&mut self) -> ValuesMut<u64, V> {
        ValuesMut {
            inner: self.iter_mut(),
        }
    }

    pub fn drain(&mut self) -> Drain<u64, V> {
        Drain::new(&mut self.cache, &mut self.count)
    }

    //**** Internal hash stuff *****

    #[inline]
    fn hash_u64(seed: u64) -> u64 {
        let a = 11400714819323198549u64;
        let val = a.wrapping_mul(seed);
        val
    }

    #[inline]
    fn calc_index(&self, key: u64) -> usize {
        let hash = Self::hash_u64(key);
        // Faster modulus
        (hash & self.mod_mask) as usize
    }

    #[inline]
    fn lim(&self) -> usize {
        2u64.pow(self.size) as usize
    }

    fn increase_cache(&mut self) {
        self.size += 1;
        let new_lim = self.lim();
        self.mod_mask = (new_lim as u64) - 1;

        let mut vec: Vec<Vec<(u64, V)>> = Vec::new();

        vec.append(&mut self.cache);

        for _ in 0..new_lim {
            self.cache.push(Vec::with_capacity(0));
        }

        for k in vec.into_iter().flatten() {
            let ix = self.calc_index(k.0);

            let ref mut vals = self.cache[ix];
            vals.push(k);
        }

        debug_assert!(
            self.cache.len() == self.lim(),
            "cache vector the wrong length, lim: {:?} cache: {:?}",
            self.lim(),
            self.cache.len()
        );
    }

    fn ensure_load_rate(&mut self) {
        while ((self.count * 100) / self.cache.len()) > 70 {
            self.increase_cache();
        }
    }

    /// Number of elements in map.
    ///
    pub fn len(&self) -> usize {
        self.count as usize
    }

    /// Force count number of slots filled.
    ///
    pub fn load(&self) -> u64 {
        self.cache.iter().filter(|vals| !vals.is_empty()).count() as u64
    }

    pub fn load_rate(&self) -> f64 {
        (self.count as f64) / (self.cache.len() as f64) * 100f64
    }

    /// Total number of slots available.
    ///
    pub fn capacity(&self) -> usize {
        self.cache.len()
    }

    pub fn assert_count(&self) -> bool {
        let count = self.cache.iter().flatten().count();

        self.count == count
    }

    pub fn collisions(&self) -> IntMap<u64> {
        let mut map = IntMap::new();

        for s in self.cache.iter() {
            let key = s.len() as u64;
            if key > 1 {
                if !map.contains_key(key) {
                    map.insert(key, 1);
                } else {
                    let counter = map.get_mut(key).unwrap();
                    *counter += 1;
                }
            }
        }

        // map.sort();

        map
    }

    /// Gets the [`Entry`] that corresponds to the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::{IntMap, Entry};
    ///
    /// let mut counters = IntMap::new();
    ///
    /// for number in [10, 30, 10, 40, 50, 50, 60, 50] {
    ///     let counter = match counters.entry(number) {
    ///         Entry::Occupied(entry) => entry.into_mut(),
    ///         Entry::Vacant(entry) => entry.insert(0),
    ///     };
    ///     *counter += 1;
    /// }
    ///
    /// assert_eq!(counters.get(10), Some(&2));
    /// assert_eq!(counters.get(20), None);
    /// assert_eq!(counters.get(30), Some(&1));
    /// assert_eq!(counters.get(40), Some(&1));
    /// assert_eq!(counters.get(50), Some(&3));
    /// assert_eq!(counters.get(60), Some(&1));
    /// ```
    pub fn entry(&mut self, key: u64) -> Entry<V> {
        Entry::new(
            key,
            &mut self.cache,
            &mut self.size,
            &mut self.mod_mask,
            &mut self.count,
        )
    }
}

// ***************** Internal hash stuff *********************
#[inline]
fn hash_u64(seed: u64) -> u64 {
    let a = 11400714819323198549u64;
    let val = a.wrapping_mul(seed);
    val
}

#[inline]
fn cache_index(mod_mask: u64, key: u64) -> usize {
    let hash = hash_u64(key);
    // Faster modulus
    (hash & mod_mask) as usize
}

fn indices<V>(cache: &Vec<Vec<(u64, V)>>, mod_mask: u64, key: u64) -> (usize, Option<usize>) {
    let cache_ix = cache_index(mod_mask, key);

    let vals = &cache[cache_ix];

    for (vals_ix, (k, _)) in vals.iter().enumerate() {
        if k == &key {
            return (cache_ix, Some(vals_ix));
        }
    }

    (cache_ix, None)
}

#[inline]
fn lim(size: u32) -> usize {
    2u64.pow(size) as usize
}

#[cold]
fn increase_cache<V>(cache: &mut Vec<Vec<(u64, V)>>, size: &mut u32, mod_mask: &mut u64) {
    *size += 1;
    let new_lim = lim(*size);
    *mod_mask = (new_lim as u64) - 1;

    let mut vec: Vec<Vec<(u64, V)>> = Vec::new();

    vec.append(cache);

    for _ in 0..new_lim {
        cache.push(Vec::with_capacity(0));
    }

    while vec.len() > 0 {
        let mut values = vec.pop().unwrap();
        while values.len() > 0 {
            if let Some(k) = values.pop() {
                let cache_ix = cache_index(*mod_mask, k.0);

                let ref mut vals = cache[cache_ix];
                vals.push(k);
            }
        }
    }

    debug_assert!(
        cache.len() == lim(*size),
        "cache vector has the wrong length, lim: {:?} cache: {:?}",
        lim(*size),
        cache.len()
    );
}

fn ensure_load_rate<V>(
    cache: &mut Vec<Vec<(u64, V)>>,
    size: &mut u32,
    mod_mask: &mut u64,
    count: usize,
) -> bool {
    let mut has_cache_increased = false;
    while ((count * 100) / cache.len()) > 70 {
        increase_cache(cache, size, mod_mask);
        has_cache_increased = true;
    }
    has_cache_increased
}

use std::slice::Iter as SliceIter;
use std::slice::IterMut as SliceIterMut;
use std::vec::IntoIter as VecIntoIter;

// ***************** Iter *********************

pub struct Iter<'a, K: 'a, V: 'a> {
    outer: SliceIter<'a, Vec<(K, V)>>,
    inner: SliceIter<'a, (K, V)>,
}

impl<'a, K, V> Iter<'a, K, V> {
    pub fn new(vec: &'a Vec<Vec<(K, V)>>) -> Self {
        let mut outer = vec.iter();
        let inner = { outer.next() }
            .map(|v| v.iter())
            .unwrap_or_else(|| (&[]).iter());

        Iter {
            outer: outer,
            inner: inner,
        }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        loop {
            if let Some(r) = self.inner.next() {
                return Some((&r.0, &r.1));
            }

            self.inner = self.outer.next()?.iter();
        }
    }
}

// ***************** Iter Mut *********************

pub struct IterMut<'a, K: 'a, V: 'a> {
    outer: SliceIterMut<'a, Vec<(K, V)>>,
    inner: SliceIterMut<'a, (K, V)>,
}

impl<'a, K, V> IterMut<'a, K, V> {
    fn new(vec: &'a mut Vec<Vec<(K, V)>>) -> IterMut<'a, K, V> {
        let mut outer = vec.iter_mut();
        let inner = { outer.next() }
            .map(|v| v.iter_mut())
            .unwrap_or_else(|| (&mut []).iter_mut());

        IterMut {
            outer: outer,
            inner: inner,
        }
    }
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<(&'a K, &'a mut V)> {
        loop {
            if let Some(r) = self.inner.next() {
                return Some((&r.0, &mut r.1));
            }

            self.inner = self.outer.next()?.iter_mut();
        }
    }
}

// ***************** Values Iter *********************

pub struct Values<'a, K: 'a, V: 'a> {
    inner: Iter<'a, K, V>,
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<&'a V> {
        self.inner.next().map(|kv| kv.1)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

// ***************** Keys Iter *********************

pub struct Keys<'a, K: 'a, V: 'a> {
    inner: Iter<'a, K, V>,
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    #[inline]
    fn next(&mut self) -> Option<&'a K> {
        self.inner.next().map(|kv| kv.0)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

// ***************** Values Mut *********************

pub struct ValuesMut<'a, K: 'a, V: 'a> {
    inner: IterMut<'a, K, V>,
}

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    #[inline]
    fn next(&mut self) -> Option<&'a mut V> {
        self.inner.next().map(|kv| kv.1)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

pub struct Drain<'a, K: 'a, V: 'a> {
    count: &'a mut usize,
    outer: SliceIterMut<'a, Vec<(K, V)>>,
    inner: Option<std::vec::Drain<'a, (K, V)>>,
}

impl<'a, K, V> Drain<'a, K, V> {
    fn new(vec: &'a mut Vec<Vec<(K, V)>>, count: &'a mut usize) -> Drain<'a, K, V> {
        let mut outer = vec.iter_mut();
        let inner = outer
            .next()
            .map(|v| Some(v.drain(..)))
            .unwrap_or_else(|| None);

        Drain {
            count: count,
            outer: outer,
            inner: inner,
        }
    }
}

impl<'a, K, V> Iterator for Drain<'a, K, V> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<(K, V)> {
        loop {
            if let Some(r) = self.inner.as_mut().and_then(|i| i.next()) {
                *self.count -= 1;
                return Some((r.0, r.1));
            }

            self.inner = Some(self.outer.next()?.drain(..));
        }
    }
}

// ***************** Into Iter *********************

impl<V> IntoIterator for IntMap<V> {
    type Item = (u64, V);
    type IntoIter = IntoIter<u64, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.cache)
    }
}

pub struct IntoIter<K, V> {
    outer: VecIntoIter<Vec<(K, V)>>,
    inner: VecIntoIter<(K, V)>,
}

impl<K, V> IntoIter<K, V> {
    pub fn new(vec: Vec<Vec<(K, V)>>) -> Self {
        let mut outer = vec.into_iter();
        let inner = { outer.next() }
            .map(|v| v.into_iter())
            .unwrap_or_else(|| (Vec::new()).into_iter());

        IntoIter {
            outer: outer,
            inner: inner,
        }
    }
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<(K, V)> {
        loop {
            if let Some(r) = self.inner.next() {
                return Some((r.0, r.1));
            }

            self.inner = self.outer.next()?.into_iter();
        }
    }
}

// ***************** Extend *********************

impl<V> Extend<(u64, V)> for IntMap<V> {
    #[inline]
    fn extend<T: IntoIterator<Item = (u64, V)>>(&mut self, iter: T) {
        for elem in iter {
            self.insert(elem.0, elem.1);
        }
    }
}

// ***************** FromIterator *********************

impl<V> std::iter::FromIterator<(u64, V)> for IntMap<V> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = (u64, V)>>(iter: T) -> Self {
        let iterator = iter.into_iter();
        let (lower_bound, _) = iterator.size_hint();

        let mut map = IntMap::with_capacity(lower_bound);
        for elem in iterator {
            map.insert(elem.0, elem.1);
        }
        map
    }
}

// ***************** Equality *********************

impl<V> PartialEq for IntMap<V>
where
    V: PartialEq,
{
    fn eq(&self, other: &IntMap<V>) -> bool {
        self.iter().all(|(k, a)| other.get(*k) == Some(a))
    }
}
impl<V> Eq for IntMap<V> where V: Eq {}

// ***************** Debug *********************

impl<V> std::fmt::Debug for IntMap<V>
where
    V: std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_map().entries(self.iter()).finish()
    }
}

// ***************** Entry *********************

/// A view into a single entry in a [`IntMap`], which may either be vacant or occupied.
///
/// The entry can be constructed by calling [`IntMap::entry`] with a key. It allows inspection
/// and in-place manipulation of its value without repeated lookups.
pub enum Entry<'a, V: 'a> {
    /// The entry is occupied.
    Occupied(OccupiedEntry<'a, V>),
    /// The entry is vacant.
    Vacant(VacantEntry<'a, V>),
}

impl<'a, V> Entry<'a, V> {
    #[inline]
    fn new(
        key: u64,
        cache: &'a mut Vec<Vec<(u64, V)>>,
        size: &'a mut u32,
        mod_mask: &'a mut u64,
        count: &'a mut usize,
    ) -> Self {
        let (cache_ix, val_ix) = indices(cache, *mod_mask, key);

        match val_ix {
            Some(vals_ix) => Entry::Occupied(OccupiedEntry {
                vals_ix,
                vals: &mut cache[cache_ix],
                count,
            }),
            None => Entry::Vacant(VacantEntry {
                key,
                cache_ix,
                cache,
                size,
                mod_mask,
                count,
            }),
        }
    }
}

/// A view into an occupied entry in a [`IntMap`]. It is part of the [`Entry`] enum.
pub struct OccupiedEntry<'a, V: 'a> {
    // Index to vals, guaranteed to be valid
    vals_ix: usize,
    // Element of IntMap::cache, guaranteed to be non-empty
    vals: &'a mut Vec<(u64, V)>,
    // IntMap::count, guaranteed to be non-zero
    count: &'a mut usize,
}

impl<'a, V> OccupiedEntry<'a, V> {
    /// Gets a reference to the value in the entry.
    pub fn get(&self) -> &V {
        // Safety: We didn't modify the cache since we calculated the index
        unsafe { &self.vals.get_unchecked(self.vals_ix).1 }
    }

    /// Gets a mutable reference to the value in the entry.
    pub fn get_mut(&mut self) -> &mut V {
        // Safety: We didn't modify the cache since we calculated the index
        unsafe { &mut self.vals.get_unchecked_mut(self.vals_ix).1 }
    }

    /// Converts the entry into a mutable reference to the value in the entry with a
    /// lifetime bound to the [`IntMap`] itself.
    pub fn into_mut(self) -> &'a mut V {
        // Safety: We didn't modify the cache since we calculated the index
        unsafe { &mut self.vals.get_unchecked_mut(self.vals_ix).1 }
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
pub struct VacantEntry<'a, V: 'a> {
    key: u64,
    // Index to cache, guaranteed to be valid
    cache_ix: usize,
    // IntMap::cache, guaranteed to be non-empty
    cache: &'a mut Vec<Vec<(u64, V)>>,
    // IntMap::size
    size: &'a mut u32,
    // IntMap::mod_mask
    mod_mask: &'a mut u64,
    // IntMap::count
    count: &'a mut usize,
}

impl<'a, V: 'a> VacantEntry<'a, V> {
    fn insert_impl(&mut self, value: V) -> Option<(usize, usize)> {
        let cache_ix = self.cache_ix;
        // Safety: We didn't modify the cache since we calculated the index
        let vals = unsafe { self.cache.get_unchecked_mut(cache_ix) };
        // The index to vals after we'll push the value
        let vals_ix = vals.len();

        // We modify the cache here, but the indices are still valid
        *self.count += 1;
        vals.push((self.key, value));

        let has_cache_increased = if (*self.count & 4) == 4 {
            // Warning: If this functions returns true, the cache has been increased and
            // both indices are invalid
            ensure_load_rate(self.cache, self.size, self.mod_mask, *self.count)
        } else {
            false
        };

        // Returns the indices if they are still valid
        if has_cache_increased {
            None
        } else {
            Some((cache_ix, vals_ix))
        }
    }

    /// Inserts a value into the entry and returns a mutable reference to it.
    pub fn insert(mut self, value: V) -> &'a mut V {
        let (new_cache_ix, new_val_ix) = match self.insert_impl(value) {
            Some((cache_ix, vals_ix)) => {
                // Indices are still valid
                (cache_ix, vals_ix)
            }
            None => {
                // The old indices are not valid anymore, we need to recalculate them
                let (cache_ix, val_ix) = indices(self.cache, *self.mod_mask, self.key);
                // Safety: We inserted the key and value, so the index must be available
                (cache_ix, unsafe { val_ix.unwrap_unchecked() })
            }
        };

        // Safety: We ensured that either the old indices are valid or new indices were calculated
        unsafe {
            &mut self
                .cache
                .get_unchecked_mut(new_cache_ix)
                .get_unchecked_mut(new_val_ix)
                .1
        }
    }
}
