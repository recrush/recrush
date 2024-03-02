mod cache;
mod admission;
mod eviction;

pub use cache::{ModularCache, TtlCache};

pub mod policy {
    pub use super::admission::AdmissionPolicy;
    pub use super::eviction::EvictionPolicy;
}

#[cfg(feature = "async")]
pub use crate::cache::ConcurrentCache;

pub mod preconfig {
    use super::admission::AlwaysAdmissionPolicy;
    use super::eviction::LruEvictionPolicy;
    use super::ModularCache;

    pub type LruCache<K, V> = ModularCache<K, V, AlwaysAdmissionPolicy, LruEvictionPolicy<K>>;
    pub type TtlLruCache<K, V> = super::TtlCache<K, V, AlwaysAdmissionPolicy, LruEvictionPolicy<K>>;

    #[cfg(feature = "async")]
    pub mod concurrent {
        use super::LruCache as SingleThreadLruCache;
        use super::TtlLruCache as SingleThreadTtlCache;
        use crate::cache::ConcurrentCache;
        use std::{hash::Hash, time::Duration};

        pub type LruCache<K, V> = ConcurrentCache<SingleThreadLruCache<K, V>>;

        pub fn new_lru_cache<K, V>(maximum_size: usize) -> LruCache<K, V>
        where
            K: Hash + Eq + Clone + std::fmt::Debug,
            V: Clone,
        {
            LruCache::new(SingleThreadLruCache::new(maximum_size))
        }

        pub type TtlLruCache<K, V> = ConcurrentCache<SingleThreadTtlCache<K, V>>;

        pub fn new_ttl_cache<K, V>(maximum_size: usize, ttl: Duration) -> TtlLruCache<K, V>
        where
            K: Hash + Eq + Clone + std::fmt::Debug,
            V: Clone,
        {
            TtlLruCache::new(SingleThreadTtlCache::new(maximum_size, ttl))
        }
    }
}
