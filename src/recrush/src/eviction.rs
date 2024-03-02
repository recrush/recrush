mod lru;

pub use lru::LruEvictionPolicy;

use std::borrow::Borrow;
use std::hash::Hash;

pub trait EvictionPolicy<K> {
    fn get_victim(&mut self) -> Option<K>;

    fn on_eviction(&mut self, key: &K);
    fn on_insert(&mut self, key: &K);
    fn on_update(&mut self, key: &K);
    fn on_cache_hit<Q: ?Sized>(&mut self, key: &Q)
    where
        K: Borrow<Q>,
        Q: Hash + Eq;

    fn clear(&mut self);

    fn invalidate<Q: ?Sized>(&mut self, key: &Q)
    where
        K: Borrow<Q>,
        Q: Hash + Eq;
}
