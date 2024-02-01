use std::borrow::Borrow;
use std::hash::Hash;

#[cfg(feature = "async")]
mod concurrent_cache;
mod modular_cache;
mod ttl_cache;

#[cfg(feature = "async")]
pub use concurrent_cache::ConcurrentCache;

pub use modular_cache::ModularCache;
pub use ttl_cache::TtlCache;

pub trait Cache {
    type Key: Hash + Eq + std::fmt::Debug;
    type Value;

    /// Insert an item in the cache.
    ///
    /// The first return value indicates whether an insertion has taken place (because the cache can refuse to insert an item).
    /// The second return value is the optional eviction victim, returned only if this call to insert caused an eviction.
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> (bool, Option<Self::Value>);

    fn get<Q: ?Sized>(&mut self, key: &Q) -> Option<&Self::Value>
    where
        Self::Key: Borrow<Q>,
        Q: Hash + Eq;

    fn invalidate<Q: ?Sized>(&mut self, key: &Q)
    where
        Self::Key: Borrow<Q>,
        Q: Hash + Eq;

    fn clear(&mut self);
}
