#![forbid(unsafe_code)]

//! Specialized hashmap for integer based keys.
//!
//! For more information see the [README](https://github.com/JesperAxelsson/rust-intmap/blob/master/README.md).
//!
//! <div class="warning">
//! Be aware that no effort is made against DoS attacks.
//! </div>

#[cfg(feature = "serde")]
mod serde;

mod entry;
mod int;
mod int_key;
mod iter;

use core::iter::{IntoIterator, Iterator};
use int::SealedInt;

pub use entry::*;
pub use int::Int;
pub use int_key::IntKey;
pub use iter::*;

// Test examples from the README.
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

/// A hashmap that maps an integer based `K` to `V`.
#[derive(Clone)]
pub struct IntMap<K, V> {
    // The slots for the key/value pairs.
    //
    // The number of slots is what we call "capacity". Two or more key/value pairs occupy the same
    // slot if they have a hash collision.
    // The size of `cache` as binary exponent. The actual size of `cache` is `2^size`.
    cache: Vec<Vec<(K, V)>>,
    // The size of `cache` as binary exponent. The actual size of `cache` is `2^size`.
    size: u32,
    // A bit mask for calculating an index for `cache`. Must be recomputed if `size` changes.
    mod_mask: usize,
    // The number of stored key/value pairs.
    count: usize,
    // The ratio between key/value pairs and available slots that we try to ensure.
    //
    // Multiplied by 1000, e.g. a load factor of 90.9% will result in the value 909.
    load_factor: usize,
}

impl<K, V> IntMap<K, V> {
    /// Creates a new [`IntMap`].
    ///
    /// The [`IntMap`] is initially created with a capacity of 0, so it will not allocate until it
    /// is first inserted into.
    ///
    /// The [`IntMap`] is initially created with a capacity of 0, so it will not allocate until it
    /// is first inserted into.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64, u64> = IntMap::new();
    /// assert_eq!(map, IntMap::default());
    /// ```
    pub const fn new() -> Self {
        Self {
            cache: Vec::new(),
            size: 0,
            count: 0,
            mod_mask: 0,
            load_factor: 909, // 90.9%
        }
    }
}

impl<K: IntKey, V> IntMap<K, V> {
    /// Creates a new [`IntMap`] with at least the given capacity.
    ///
    /// If the capacity is 0, the [`IntMap`] will not allocate. Otherwise the capacity is rounded
    /// to the next power of two and space for elements is allocated accordingly.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64, u64> = IntMap::with_capacity(20);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        let mut map = Self::new();
        map.reserve(capacity);
        map
    }

    /// Sets the load factor of the [`IntMap`] rounded to the first decimal point.
    ///
    /// A load factor between 0.0 and 1.0 will reduce hash collisions but use more space.
    /// A load factor above 1.0 will tolerate hash collisions and use less space.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64, u64> = IntMap::with_capacity(20);
    /// map.set_load_factor(0.909); // Sets load factor to 90.9%
    /// ```
    pub fn set_load_factor(&mut self, load_factor: f32) {
        self.load_factor = (load_factor * 1000.) as usize;
        self.ensure_load_rate();
    }

    /// Returns the current load factor.
    pub fn get_load_factor(&self) -> f32 {
        self.load_factor as f32 / 1000.
    }

    /// Ensures that the [`IntMap`] has space for at least `additional` more elements
    pub fn reserve(&mut self, additional: usize) {
        let capacity = self.count + additional;
        while self.lim() < capacity {
            self.increase_cache();
        }
    }

    /// Inserts a key/value pair into the [`IntMap`].
    ///
    /// This function returns the previous value if any otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap::<u64, _> = IntMap::new();
    /// assert_eq!(map.insert(21, "Eat my shorts"), None);
    /// assert_eq!(map.insert(21, "Ay, caramba"), Some("Eat my shorts"));
    /// assert_eq!(map.get(21), Some(&"Ay, caramba"));
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.ensure_load_rate();

        let k = key.into_int();
        let ix = k.calc_index(self.mod_mask, K::PRIME);

        let vals = &mut self.cache[ix];
        let pos = vals.iter().position(|kv| kv.0.into_int() == k);

        let old = if let Some(pos) = pos {
            Some(vals.swap_remove(pos).1)
        } else {
            // Only increase count if we actually add a new entry
            self.count += 1;
            None
        };

        vals.push((key, value));

        old
    }

    /// Insert a key/value pair into the [`IntMap`] if the key is not yet inserted.
    ///
    /// This function returns true if key/value were inserted and false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap::<u64, _> = IntMap::new();
    /// assert!(map.insert_checked(21, "Eat my shorts"));
    /// assert!(!map.insert_checked(21, "Ay, caramba"));
    /// assert_eq!(map.get(21), Some(&"Eat my shorts"));
    /// ```
    pub fn insert_checked(&mut self, key: K, value: V) -> bool {
        self.ensure_load_rate();

        let k = key.into_int();
        let ix = k.calc_index(self.mod_mask, K::PRIME);

        let vals = &mut self.cache[ix];
        if vals.iter().any(|kv| kv.0.into_int() == k) {
            return false;
        }

        self.count += 1;
        vals.push((key, value));

        true
    }

    /// Gets the value for the given key from the [`IntMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64, u64> = IntMap::new();
    /// map.insert(21, 42);
    /// let val = map.get(21);
    /// assert!(val.is_some());
    /// assert_eq!(*val.unwrap(), 42);
    /// assert!(map.contains_key(21));
    /// ```
    pub fn get(&self, key: K) -> Option<&V> {
        if self.is_empty() {
            return None;
        }

        let k = key.into_int();
        let ix = k.calc_index(self.mod_mask, K::PRIME);

        let vals = &self.cache[ix];

        vals.iter()
            .find_map(|kv| (kv.0.into_int() == k).then(|| &kv.1))
    }

    /// Gets the mutable value for the given key from the [`IntMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64, u64> = IntMap::new();
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
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        if self.is_empty() {
            return None;
        }

        let k = key.into_int();
        let ix = k.calc_index(self.mod_mask, K::PRIME);

        let vals = &mut self.cache[ix];

        return vals
            .iter_mut()
            .find_map(|kv| (kv.0.into_int() == k).then(move || &mut kv.1));
    }

    /// Removes the value for given key from the [`IntMap`] and returns it.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64, u64> = IntMap::new();
    /// map.insert(21, 42);
    /// let val = map.remove(21);
    /// assert!(val.is_some());
    /// assert_eq!(val.unwrap(), 42);
    /// assert!(!map.contains_key(21));
    /// ```
    pub fn remove(&mut self, key: K) -> Option<V> {
        if self.is_empty() {
            return None;
        }

        let k = key.into_int();
        let ix = k.calc_index(self.mod_mask, K::PRIME);

        let vals = &mut self.cache[ix];

        for i in 0..vals.len() {
            let peek = &vals[i].0;

            if peek.into_int() == k {
                self.count -= 1;
                let kv = vals.swap_remove(i);
                return Some(kv.1);
            }
        }

        None
    }

    /// Returns true if the key is present in the [`IntMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64, u64> = IntMap::new();
    /// map.insert(21, 42);
    /// assert!(map.contains_key(21));
    /// ```
    pub fn contains_key(&self, key: K) -> bool {
        self.get(key).is_some()
    }

    /// Removes all elements from the [`IntMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64, u64> = IntMap::new();
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

    /// Retains only the key/value pairs specified by the predicate.
    ///
    /// In other words, remove all elements such that `f(key, &value)` returns false.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64, u64> = IntMap::new();
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
        F: FnMut(K, &V) -> bool,
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

    /// Returns true if the [`IntMap`] is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64, u64> = IntMap::new();
    /// map.insert(21, 42);
    /// assert!(!map.is_empty());
    /// map.remove(21);
    /// assert!(map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    //**** Iterators *****

    /// Returns an [`Iterator`] over all key/value pairs.
    pub fn iter(&self) -> Iter<K, V> {
        Iter::new(&self.cache)
    }

    /// Returns an [`Iterator`] over all key/value pairs with mutable value.
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut::new(&mut self.cache)
    }

    /// Returns an [`Iterator`] over all keys.
    pub fn keys(&self) -> Keys<K, V> {
        Keys { inner: self.iter() }
    }

    /// Returns an [`Iterator`] over all values.
    pub fn values(&self) -> Values<K, V> {
        Values { inner: self.iter() }
    }

    /// Returns an [`Iterator`] over all mutable values.
    pub fn values_mut(&mut self) -> ValuesMut<K, V> {
        ValuesMut {
            inner: self.iter_mut(),
        }
    }

    /// Returns an [`Iterator`] over all key/value pairs that removes the pairs from the [`IntMap`]
    /// during iteration.
    ///
    /// If the [`Iterator`] is droppend then all remaining key/value pairs will be removed from
    /// the [`IntMap`].
    pub fn drain(&mut self) -> Drain<K, V> {
        Drain::new(&mut self.cache, &mut self.count)
    }

    //**** Internal hash stuff *****

    #[inline(always)]
    fn lim(&self) -> usize {
        if self.size == 0 {
            0
        } else {
            2usize.pow(self.size)
        }
    }

    fn increase_cache(&mut self) {
        self.size += 1;
        let new_lim = self.lim();
        self.mod_mask = new_lim - 1;

        let mut vec: Vec<Vec<(K, V)>> = (0..new_lim).map(|_| Vec::new()).collect();
        std::mem::swap(&mut self.cache, &mut vec);

        for key in vec.into_iter().flatten() {
            let k = key.0.into_int();
            let ix = k.calc_index(self.mod_mask, K::PRIME);

            let vals = &mut self.cache[ix];
            vals.push(key);
        }

        debug_assert!(
            self.cache.len() == self.lim(),
            "cache vector the wrong length, lim: {:?} cache: {:?}",
            self.lim(),
            self.cache.len()
        );
    }

    #[inline]
    fn ensure_load_rate(&mut self) {
        // Handle empty cache to prevent division by zero.
        if self.cache.is_empty() {
            self.increase_cache()
        }

        // Tried using floats here but insert performance tanked.
        while ((self.count * 1000) / self.cache.len()) > self.load_factor {
            self.increase_cache();
        }
    }

    //**** More public methods *****

    /// Returns the number of key/value pairs in the [`IntMap`].
    pub fn len(&self) -> usize {
        self.count
    }

    /// Returns the number of filled slots.
    pub fn load(&self) -> u64 {
        self.cache.iter().filter(|vals| !vals.is_empty()).count() as u64
    }

    /// Returns the ratio between key/value pairs and available slots as percentage.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64, u64> = IntMap::with_capacity(2);
    /// map.set_load_factor(2.0);
    /// assert_eq!(map.load_rate(), 0.0);
    /// map.insert(1, 42);
    /// assert_eq!(map.load_rate(), 50.0);
    /// map.insert(2, 42);
    /// assert_eq!(map.load_rate(), 100.0);
    /// map.insert(3, 42);
    /// assert_eq!(map.load_rate(), 150.0);
    /// ```
    pub fn load_rate(&self) -> f64 {
        (self.count as f64) / (self.cache.len() as f64) * 100f64
    }

    /// Returns the total number of available slots.
    pub fn capacity(&self) -> usize {
        self.cache.len()
    }

    //**** Testing methods *****

    /// Checks whether the actual count of key/value pairs matches [`IntMap::count`].
    ///
    /// Only for testing.
    #[doc(hidden)]
    pub fn assert_count(&self) -> bool {
        let count = self.cache.iter().flatten().count();

        self.count == count
    }

    /// Returns a new [`IntMap`] that contains only the collisions of the current [`IntMap`].
    ///
    /// Only for testing.
    #[doc(hidden)]
    pub fn collisions(&self) -> IntMap<u64, u64> {
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

        map
    }

    //**** Entry API *****

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
    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        Entry::new(key, self)
    }
}

impl<K, V> Default for IntMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

// ***************** Equality *********************

impl<K, V> PartialEq for IntMap<K, V>
where
    K: IntKey,
    V: PartialEq,
{
    fn eq(&self, other: &IntMap<K, V>) -> bool {
        self.iter().all(|(&k, a)| other.get(k) == Some(a))
            && other.iter().all(|(&k, a)| self.get(k) == Some(a))
    }
}
impl<K: IntKey, V: Eq> Eq for IntMap<K, V> {}

// ***************** Debug *********************

impl<K, V> std::fmt::Debug for IntMap<K, V>
where
    K: IntKey + std::fmt::Debug,
    V: std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_map().entries(self.iter()).finish()
    }
}
