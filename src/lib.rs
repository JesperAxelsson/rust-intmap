#[cfg(feature = "serde")]
mod serde;

mod entry;
mod iter;

use core::iter::{IntoIterator, Iterator};
use iter::*;

pub use entry::*;

#[derive(Clone)]
pub struct IntMap<V> {
    cache: Vec<Vec<(u64, V)>>,
    size: u32,
    mod_mask: u64,
    count: usize,
    load_factor: usize,
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
    /// assert_eq!(map, IntMap::default());
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
            load_factor: 909, // 90.9%
        };

        map.increase_cache();

        while map.lim() < capacity {
            map.increase_cache();
        }

        map
    }

    /// Sets load rate of IntMap rounded to the first decimal point.
    ///
    /// Values above 1.0 is allowed.
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

    /// Returns current load_factor
    pub fn get_load_factor(&self) -> f32 {
        self.load_factor as f32 / 1000.
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

        let vals = &mut self.cache[ix];
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
        let ix = self.calc_index(key);

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
    pub fn get_mut(&mut self, key: u64) -> Option<&mut V> {
        let ix = self.calc_index(key);

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
    pub fn remove(&mut self, key: u64) -> Option<V> {
        let ix = self.calc_index(key);

        let ref mut vals = self.cache[ix];

        for i in 0..vals.len() {
            let peek = vals[i].0;

            if peek == key {
                self.count -= 1;
                let kv = vals.swap_remove(i);
                return Some(kv.1);
            }
        }

        return None;
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
    pub(crate) fn calc_index(&self, key: u64) -> usize {
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

        let mut vec: Vec<Vec<(u64, V)>> = (0..new_lim).map(|_| Vec::new()).collect();
        std::mem::swap(&mut self.cache, &mut vec);

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
        // Tried using floats here but insert performance tanked.
        while ((self.count * 1000) / self.cache.len()) > self.load_factor {
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
        Entry::new(key, self)
    }
}

impl<V> Default for IntMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

// ***************** Equality *********************

impl<V> PartialEq for IntMap<V>
where
    V: PartialEq,
{
    fn eq(&self, other: &IntMap<V>) -> bool {
        self.iter().all(|(k, a)| other.get(*k) == Some(a))
            && other.iter().all(|(k, a)| self.get(*k) == Some(a))
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
