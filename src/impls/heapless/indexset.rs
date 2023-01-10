use std::borrow::Borrow;
use std::hash::{BuildHasher, Hash};
use crate::{Clear, Collection, CollectionMut, CollectionRef, Get, Insert, Iter, Len, Remove, SimpleCollectionRef};
use heapless::{IndexSet, IndexSetIter};

impl<T, S, const N: usize> Collection for IndexSet<T, S, N> {
    type Item = T;
}

impl<T, S, const N: usize> CollectionRef for IndexSet<T, S, N> {
    type ItemRef<'a> = &'a T where Self: 'a;

    crate::covariant_item_ref!();
}

impl<T, S, const N: usize> CollectionMut for IndexSet<T, S, N> {
    type ItemMut<'a> = &'a mut T where Self: 'a;

    crate::covariant_item_mut!();
}

impl<T, S, const N: usize> SimpleCollectionRef for IndexSet<T, S, N> {
    fn into_ref<'a>(r: Self::ItemRef<'a>) -> &'a T
    where
        Self: 'a,
    {
        r
    }
}

impl<T, S, const N: usize> Len for IndexSet<T, S, N> {
    #[inline(always)]
    fn len(&self) -> usize {
        IndexSet::len(self)
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        IndexSet::is_empty(self)
    }
}

impl<'a, Q, T, S, const N: usize> Get<&'a Q> for IndexSet<T, S, N>
where
    Q: Eq + Hash + ?Sized,
    T: Borrow<Q> + Eq + Hash,
    S: BuildHasher,
{
    #[inline(always)]
    fn get(&self, _: &'a Q) -> Option<&T> {
        unimplemented!()
    }

    #[inline(always)]
    fn contains(&self, key: &'a Q) -> bool {
        IndexSet::contains(self, key)
    }
}

impl<T, S, const N: usize> Insert for IndexSet<T, S, N>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    type Output = bool;

    #[inline(always)]
    fn insert(&mut self, t: Self::Item) -> Self::Output {
        match IndexSet::insert(self, t) {
            Ok(v) => v,
            Err(_) => panic!("tried to insert element past the supported capacity"),
        }
    }
}

impl<'a, Q, T, S, const N: usize> Remove<&'a Q> for IndexSet<T, S, N> {
    fn remove(&mut self, _: &'a Q) -> Option<T> {
        unimplemented!()
    }
}

impl<T, S, const N: usize> Clear for IndexSet<T, S, N> {
    #[inline(always)]
    fn clear(&mut self) {
        IndexSet::clear(self)
    }
}

impl<T, S, const N: usize> Iter for IndexSet<T, S, N> {
    type Iter<'a> = IndexSetIter<'a, T> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        IndexSet::iter(self)
    }
}
