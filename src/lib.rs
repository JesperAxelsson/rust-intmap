#![forbid(unsafe_code)]

#[cfg(feature = "serde")]
mod serde;

mod entry;
mod int;
mod iter;

use core::iter::{IntoIterator, Iterator};
use iter::*;

pub use entry::*;

// Test examples from the README.
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

/// An integer that can be used as key for [`IntMap`].
///
/// Note that this is a sealed trait that cannot be implemented externally.
pub trait Int: int::SealedInt {}

impl Int for u32 {}

impl Int for u64 {}

#[derive(Clone)]
pub struct IntMap<V, I = u64> {
    cache: Vec<Vec<(I, V)>>,
    size: u32,
    mod_mask: usize,
    count: usize,
    load_factor: usize,
}

impl<V> IntMap<V, u64> {
    /// Creates a new IntMap.
    ///
    /// The IntMap is initially created with a capacity of 0, so it will not allocate until it
    /// is first inserted into.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64> = IntMap::new();
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

    /// Creates a new IntMap with at least the given capacity.
    ///
    /// If capacity is 0, the IntMap will not allocate. Otherwise the capacity is rounded
    /// to the next power of two and space for elements is allocated accordingly.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64> = IntMap::with_capacity(20);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        let mut map = Self::new();
        map.reserve(capacity);
        map
    }
}

impl<V, I> IntMap<V, I> {
    /// Same as [`Self::new`] but for any integer type.
    pub const fn new_with() -> Self {
        Self {
            cache: Vec::new(),
            size: 0,
            count: 0,
            mod_mask: 0,
            load_factor: 909, // 90.9%
        }
    }
}

impl<V, I: Int> IntMap<V, I> {
    /// Same as [`Self::with_capacity`] but for any integer type.
    pub fn with_capacity_with(capacity: usize) -> Self {
        let mut map = Self::new_with();
        map.reserve(capacity);
        map
    }

    /// Sets the load factor of IntMap rounded to the first decimal point.
    ///
    /// A load factor between 0.0 and 1.0 will reduce hash collisions but use more space.
    /// A load factor above 1.0 will tolerate hash collisions and use less space.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap<u64> = IntMap::with_capacity(20);
    /// map.set_load_factor(0.909); // Sets load factor to 90.9%
    /// ```
    pub fn set_load_factor(&mut self, load_factor: f32) {
        self.load_factor = (load_factor * 1000.) as usize;
        self.ensure_load_rate();
    }

    /// Returns the current load factor
    pub fn get_load_factor(&self) -> f32 {
        self.load_factor as f32 / 1000.
    }

    /// Ensures that the IntMap has space for at least `additional` more elements
    pub fn reserve(&mut self, additional: usize) {
        let capacity = self.count + additional;
        while self.lim() < capacity {
            self.increase_cache();
        }
    }

    /// Insert key/value into the IntMap.
    ///
    /// This function returns the previous value if any otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap::<_, u64> = IntMap::new();
    /// assert_eq!(map.insert(21, "Eat my shorts"), None);
    /// assert_eq!(map.insert(21, "Ay, caramba"), Some("Eat my shorts"));
    /// assert_eq!(map.get(21), Some(&"Ay, caramba"));
    /// ```
    pub fn insert(&mut self, key: I, value: V) -> Option<V> {
        self.ensure_load_rate();

        let ix = I::calc_index(key, self.mod_mask);

        let vals = &mut self.cache[ix];
        let pos = vals.iter().position(|kv| kv.0 == key);

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

    /// Insert key/value into the IntMap if the key is not yet inserted.
    ///
    /// This function returns true if key/value were inserted and false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map: IntMap::<_, u64> = IntMap::new();
    /// assert!(map.insert_checked(21, "Eat my shorts"));
    /// assert!(!map.insert_checked(21, "Ay, caramba"));
    /// assert_eq!(map.get(21), Some(&"Eat my shorts"));
    /// ```
    pub fn insert_checked(&mut self, key: I, value: V) -> bool {
        self.ensure_load_rate();

        let ix = I::calc_index(key, self.mod_mask);

        let vals = &mut self.cache[ix];
        if vals.iter().any(|kv| kv.0 == key) {
            return false;
        }

        self.count += 1;
        vals.push((key, value));

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
    pub fn get(&self, key: I) -> Option<&V> {
        if self.is_empty() {
            return None;
        }

        let ix = I::calc_index(key, self.mod_mask);

        let vals = &self.cache[ix];

        vals.iter().find_map(|kv| (kv.0 == key).then(|| &kv.1))
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
    pub fn get_mut(&mut self, key: I) -> Option<&mut V> {
        if self.is_empty() {
            return None;
        }

        let ix = I::calc_index(key, self.mod_mask);

        let vals = &mut self.cache[ix];

        return vals
            .iter_mut()
            .find_map(|kv| (kv.0 == key).then(move || &mut kv.1));
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
    pub fn remove(&mut self, key: I) -> Option<V> {
        if self.is_empty() {
            return None;
        }

        let ix = I::calc_index(key, self.mod_mask);

        let vals = &mut self.cache[ix];

        for i in 0..vals.len() {
            let peek = vals[i].0;

            if peek == key {
                self.count -= 1;
                let kv = vals.swap_remove(i);
                return Some(kv.1);
            }
        }

        None
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
    pub fn contains_key(&self, key: I) -> bool {
        self.get(key).is_some()
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
        F: FnMut(I, &V) -> bool,
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
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    //**** Iterators *****

    pub fn iter(&self) -> Iter<I, V> {
        Iter::new(&self.cache)
    }

    pub fn iter_mut(&mut self) -> IterMut<I, V> {
        IterMut::new(&mut self.cache)
    }

    pub fn keys(&self) -> Keys<I, V> {
        Keys { inner: self.iter() }
    }

    pub fn values(&self) -> Values<I, V> {
        Values { inner: self.iter() }
    }

    pub fn values_mut(&mut self) -> ValuesMut<I, V> {
        ValuesMut {
            inner: self.iter_mut(),
        }
    }

    pub fn drain(&mut self) -> Drain<I, V> {
        Drain::new(&mut self.cache, &mut self.count)
    }

    //**** Internal hash stuff *****

    #[inline(always)]
    fn lim(&self) -> usize {
        if self.size == 0 {
            0
        } else {
            2u64.pow(self.size) as usize
        }
    }

    fn increase_cache(&mut self) {
        self.size += 1;
        let new_lim = self.lim();
        self.mod_mask = new_lim - 1;

        let mut vec: Vec<Vec<(I, V)>> = (0..new_lim).map(|_| Vec::new()).collect();
        std::mem::swap(&mut self.cache, &mut vec);

        for k in vec.into_iter().flatten() {
            let ix = I::calc_index(k.0, self.mod_mask);

            let vals = &mut self.cache[ix];
            vals.push(k);
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

    /// Number of elements in map.
    ///
    pub fn len(&self) -> usize {
        self.count
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
    pub fn entry(&mut self, key: I) -> Entry<V, I> {
        Entry::new(key, self)
    }
}

impl<V, I> Default for IntMap<V, I> {
    fn default() -> Self {
        Self::new_with()
    }
}

// ***************** Equality *********************

impl<V, I: Int> PartialEq for IntMap<V, I>
where
    V: PartialEq,
{
    fn eq(&self, other: &IntMap<V, I>) -> bool {
        self.iter().all(|(k, a)| other.get(*k) == Some(a))
            && other.iter().all(|(k, a)| self.get(*k) == Some(a))
    }
}
impl<V> Eq for IntMap<V> where V: Eq {}

// ***************** Debug *********************

impl<V, I: Int> std::fmt::Debug for IntMap<V, I>
where
    V: std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_map().entries(self.iter()).finish()
    }
}
