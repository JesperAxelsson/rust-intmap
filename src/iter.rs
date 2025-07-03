use std::iter::FlatMap as IterFlatMap;
use std::iter::Flatten as IterFlatten;
use std::slice::Iter as SliceIter;
use std::slice::IterMut as SliceIterMut;
use std::vec::Drain as VecDrain;
use std::vec::IntoIter as VecIntoIter;

use crate::IntKey;
use crate::IntMap;

// ***************** Iter *********************

impl<'a, K: IntKey, V> IntoIterator for &'a IntMap<K, V> {
    type Item = (K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(&self.cache)
    }
}

/// An iterator over the entries of a [`IntMap`]
///
/// This struct is created by [`IntMap::iter`].
pub struct Iter<'a, K: IntKey, V> {
    inner: IterFlatten<SliceIter<'a, Vec<(K, V)>>>,
}

impl<'a, K: IntKey, V> Iter<'a, K, V> {
    pub(crate) fn new(vec: &'a [Vec<(K, V)>]) -> Self {
        Iter {
            inner: vec.iter().flatten(),
        }
    }
}

impl<'a, K: IntKey, V> Iterator for Iter<'a, K, V> {
    type Item = (K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<(K, &'a V)> {
        self.inner.next().map(|r| (r.0, &r.1))
    }
}

// ***************** Iter Mut *********************

impl<'a, K: IntKey, V> IntoIterator for &'a mut IntMap<K, V> {
    type Item = (K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut::new(&mut self.cache)
    }
}

/// A mutable iterator over the entries of a [`IntMap`].
///
/// This struct is created by [`IntMap::iter_mut`].
pub struct IterMut<'a, K: IntKey, V> {
    inner: IterFlatten<SliceIterMut<'a, Vec<(K, V)>>>,
}

impl<'a, K: IntKey, V> IterMut<'a, K, V> {
    pub(crate) fn new(vec: &'a mut [Vec<(K, V)>]) -> IterMut<'a, K, V> {
        IterMut {
            inner: vec.iter_mut().flatten(),
        }
    }
}

impl<'a, K: IntKey, V> Iterator for IterMut<'a, K, V> {
    type Item = (K, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<(K, &'a mut V)> {
        self.inner.next().map(|r| (r.0, &mut r.1))
    }
}

// ***************** Keys Iter *********************

/// An iterator over the keys of a [`IntMap`].
///
/// This struct is created by [`IntMap::keys`].
pub struct Keys<'a, K: IntKey, V> {
    pub(crate) inner: Iter<'a, K, V>,
}

impl<'a, K: IntKey, V> Iterator for Keys<'a, K, V> {
    type Item = K;

    #[inline]
    fn next(&mut self) -> Option<K> {
        self.inner.next().map(|kv| kv.0)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

// ***************** Values Iter *********************

/// An iterator over the values of a [`IntMap`].
///
/// This struct is created by [`IntMap::values`].
pub struct Values<'a, K: IntKey, V> {
    pub(crate) inner: Iter<'a, K, V>,
}

impl<'a, K: IntKey, V> Iterator for Values<'a, K, V> {
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

// ***************** Values Mut *********************

/// A mutable iterator over the values of a [`IntMap`].
///
/// This struct is created by [`IntMap::values_mut`].
pub struct ValuesMut<'a, K: IntKey, V> {
    pub(crate) inner: IterMut<'a, K, V>,
}

impl<'a, K: IntKey, V> Iterator for ValuesMut<'a, K, V> {
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

// ***************** Into Iter *********************

impl<K: IntKey, V> IntoIterator for IntMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.cache)
    }
}

/// An owning iterator over the entries of a [`IntMap`].
///
/// This struct is created by [`IntMap::into_iter`].
pub struct IntoIter<K: IntKey, V> {
    inner: IterFlatten<VecIntoIter<Vec<(K, V)>>>,
}

impl<K: IntKey, V> IntoIter<K, V> {
    pub(crate) fn new(vec: Vec<Vec<(K, V)>>) -> Self {
        IntoIter {
            inner: vec.into_iter().flatten(),
        }
    }
}

impl<K: IntKey, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<(K, V)> {
        self.inner.next()
    }
}

// ***************** Drain Iter *********************

/// A draining iterator for [`IntMap`].
///
/// This struct is created by [`IntMap::drain`].
#[allow(clippy::type_complexity)]
pub struct Drain<'a, K: IntKey, V> {
    count: &'a mut usize,
    inner: IterFlatMap<
        SliceIterMut<'a, Vec<(K, V)>>,
        VecDrain<'a, (K, V)>,
        fn(&mut Vec<(K, V)>) -> VecDrain<(K, V)>,
    >,
}

impl<'a, K: IntKey, V> Drain<'a, K, V> {
    pub(crate) fn new(vec: &'a mut [Vec<(K, V)>], count: &'a mut usize) -> Drain<'a, K, V> {
        Drain {
            count,
            inner: vec.iter_mut().flat_map(|v| v.drain(..)),
        }
    }
}

impl<'a, K: IntKey, V> Iterator for Drain<'a, K, V> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<(K, V)> {
        let next = self.inner.next();
        if next.is_some() {
            *self.count -= 1;
        }
        next
    }
}

// ***************** Extend *********************

impl<K: IntKey, V> Extend<(K, V)> for IntMap<K, V> {
    #[inline]
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for elem in iter {
            self.insert(elem.0, elem.1);
        }
    }
}

// ***************** FromIterator *********************

impl<K: IntKey, V> std::iter::FromIterator<(K, V)> for IntMap<K, V> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let iterator = iter.into_iter();
        let (lower_bound, _) = iterator.size_hint();

        let mut map = IntMap::with_capacity(lower_bound);
        for elem in iterator {
            map.insert(elem.0, elem.1);
        }

        map
    }
}
