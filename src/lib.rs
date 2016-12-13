
struct Kv<V> {
    key: u64, 
    value: V
}

pub struct IntMap<V>{
    cache:  Vec<Vec<Kv<V>>>,
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


    /// Creates a new IntMap with a at least capacity, all sizes is a power of 2.
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
            if kv.key == key {
                return false;
            }
        }

        self.count += 1;
        vals.push(Kv { key: key, value: value });
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
                if kv.key == key {
                    return Some(&kv.value);
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
                if kv.key == key {
                    return Some(&mut kv.value);
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
                let peek = vals[i].key;

                if peek == key {
                    self.count -= 1;
                    let kv = vals.swap_remove(i);
                    return Some(kv.value);
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

        let mut vec: Vec<Vec<Kv<V>>> = Vec::new();

        vec.append(&mut self.cache);

        for _ in 0..new_lim {
            self.cache.push(Vec::with_capacity(0));
        }
        
        while vec.len() > 0 {
            let mut values = vec.pop().unwrap();
            while values.len() > 0 {
                if let Some(k) = values.pop() {
                    let ix = self.calc_index(k.key);

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
    // pub fn collisions(&self) -> HashMap<u64, u64> {
    //     let mut map = HashMap::new();

    //     for s in self.cache.iter() {
    //         if s.len() > 1 {
    //             let counter = map.entry(s.len() as u64).or_insert(0);
    //             *counter += s.len() as u64;
    //             // vec.push(s.len() as u64);
    //         }
    //     }

    //     // map.sort();

    //     map
    // }

}
