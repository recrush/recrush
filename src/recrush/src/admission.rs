mod always;

pub use always::AlwaysAdmissionPolicy;

use std::borrow::Borrow;
use std::hash::Hash;

pub trait AdmissionPolicy<K> {
    fn should_add(&mut self, key: &K) -> bool;
    fn should_replace(&mut self, candidate: &K, victim: &K) -> bool;

    fn on_cache_hit<Q: ?Sized>(&mut self, key: &Q)
    where
        K: Borrow<Q>,
        Q: Hash + Eq;

    fn on_cache_miss<Q: ?Sized>(&mut self, key: &Q)
    where
        K: Borrow<Q>,
        Q: Hash + Eq;

    fn clear(&mut self);

    fn invalidate<Q: ?Sized>(&mut self, key: &Q)
    where
        K: Borrow<Q>,
        Q: Hash + Eq;
}
