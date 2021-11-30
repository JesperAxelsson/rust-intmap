extern crate core;

use core::iter::{IntoIterator, Iterator};

#[derive(Clone)]
pub struct IntMap<V>{
    cache:  Vec<Vec<(u64, V)>>,
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

        let mut map = IntMap { cache: Vec::new(), size: 0, count: 0, mod_mask: 0 };

        map.increase_cache();

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

    /// Insert key/value into the IntMap.
    ///
    /// # Examples
    ///
    /// ```
    /// use intmap::IntMap;
    ///
    /// let mut map = IntMap::new();
    /// map.insert(21, "Eat my shorts");
    /// ```
    pub fn insert(&mut self, key: u64, value: V) -> bool {
        let ix = self.calc_index(key);

        {
        let ref mut vals = self.cache[ix];
        for ref kv in vals.iter() {
            if kv.0 == key {
                return false;
            }
        }

        self.count += 1;
        vals.push((key, value));
        }
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

        let ref vals = self.cache[ix];

        if vals.len() > 0 {

            for kv in vals.iter() {
                if kv.0 == key {
                    return Some(&kv.1);
                }
            }

            return None;

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
            for kv in vals {
                if kv.0 == key {
                    return Some(&mut kv.1);
                }
            }

            return None;

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
            None    => false
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
        for i in 0..self.cache.len() {
            self.cache[i].clear();
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
        for i in 0..self.cache.len() {
            self.cache[i].retain(|(k, v)| {
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
        ValuesMut { inner: self.iter_mut() }
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

        while vec.len() > 0 {
            let mut values = vec.pop().unwrap();
            while values.len() > 0 {
                if let Some(k) = values.pop() {
                    let ix = self.calc_index(k.0);

                    let ref mut vals = self.cache[ix];
                    vals.push(k);
                }
            }
        }

        debug_assert!(self.cache.len() == self.lim(), "cache vector the wrong length, lim: {:?} cache: {:?}", self.lim(), self.cache.len());
    }


    fn ensure_load_rate(&mut self) {
        while ((self.count*100) / self.cache.len()) > 70 {
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
        let mut count = 0;

        for i in 0..self.cache.len() {
            if self.cache[i].len() > 0 {
                count += 1;
            }
        }

        count
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
        let mut count = 0;

        for i in 0..self.cache.len() {
            for _ in self.cache[i].iter() {
                count += 1;
            }
        }

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
        let inner = outer.next()
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
            match self.inner.next() {
                Some(r) => return Some((&r.0, &r.1)),
                None => (),
            }

            match self.outer.next() {
                Some(v) => self.inner = v.iter(),
                None => return None,
            }
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
        let inner = outer.next()
                         .map(|v| v.iter_mut())
                         .unwrap_or_else(|| (&mut []).iter_mut() );

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
            match self.inner.next() {
                Some(r) => return Some((&r.0, &mut r.1)),
                None => (),
            }

            match self.outer.next() {
                Some(v) => self.inner = v.iter_mut(),
                None => return None,
            }
        }
    }
}


// ***************** Values Iter *********************

pub struct Values<'a, K:'a, V: 'a> {
    inner: Iter<'a, K, V>
}


impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    #[inline] fn next(&mut self) -> Option<&'a V> { self.inner.next().map(|kv| kv.1) }
    #[inline] fn size_hint(&self) -> (usize, Option<usize>) { self.inner.size_hint() }
}

// ***************** Keys Iter *********************

pub struct Keys<'a, K: 'a, V: 'a> {
    inner: Iter<'a, K, V>
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    #[inline] fn next(&mut self) -> Option<&'a K> { self.inner.next().map(|kv| kv.0) }
    #[inline] fn size_hint(&self) -> (usize, Option<usize>) { self.inner.size_hint() }
}

// ***************** Values Mut *********************

pub struct ValuesMut<'a, K:'a, V: 'a> {
    inner: IterMut<'a, K, V>
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
            match self.inner.as_mut().and_then(|i| i.next()) {
                Some(r) => {
                    *self.count -= 1;
                    return Some((r.0, r.1))
                },
                None => (),
            }

            match self.outer.next() {
                Some(v) => self.inner = Some(v.drain(..)),
                None => return None,
            }
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
        let inner = outer.next()
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
            match self.inner.next() {
                Some(r) => return Some((r.0, r.1)),
                None => (),
            }

            match self.outer.next() {
                Some(v) => self.inner = v.into_iter(),
                None => return None,
            }
        }
    }
}

// ***************** Extend *********************

impl<V> Extend<(u64, V)> for IntMap<V>
{
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

impl<V> PartialEq for IntMap<V> where V: PartialEq {
    fn eq(&self, other: &IntMap<V>) -> bool {
        self.iter().all(|(k, a)| other.get(*k) == Some(a))
    }
}
impl<V> Eq for IntMap<V> where V: Eq {}


// ***************** Debug *********************

impl<V> std::fmt::Debug for IntMap<V> where V: std::fmt::Debug {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_map().entries(self.iter()).finish()
    }
}
