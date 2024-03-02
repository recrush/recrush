mod admission;
mod eviction;

pub use admission::{AdmissionPolicy, AlwaysAdmissionPolicy};
pub use eviction::{EvictionPolicy, LruEvictionPolicy};
