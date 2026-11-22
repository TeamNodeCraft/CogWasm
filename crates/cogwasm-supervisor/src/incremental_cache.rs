//! An in-memory [`CacheStore`] for Cranelift's incremental compilation cache.
//!
//! The supervisor swaps a new artifact on every edit. Most of a plugin's
//! functions do not change between edits, so Cranelift can reuse their compiled
//! machine code if it can look it up again. Wasmtime exposes that lookup as
//! [`CacheStore`]; this module is a small process-local implementation with a
//! byte cap and first-in-first-out eviction.
//!
//! Keeping the cache in memory avoids a disk-cache directory in the edit loop.
//! Its keys already include the Cranelift version, target and flags, so a store
//! never needs to be invalidated by hand across supervisor runs: a fresh
//! process starts empty.

use std::borrow::Cow;
use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, MutexGuard};

use wasmtime::CacheStore;

/// The default cache budget: 256 MiB of serialized machine code.
pub const DEFAULT_CAPACITY_BYTES: usize = 256 * 1024 * 1024;

/// A process-local, size-capped [`CacheStore`].
///
/// Entries are evicted in insertion order once the next insertion would exceed
/// the capacity. A value larger than the whole cache is rejected rather than
/// evicting everything for something that cannot fit.
#[derive(Debug)]
pub struct InMemoryCacheStore {
    capacity_bytes: usize,
    state: Mutex<State>,
}

#[derive(Debug, Default)]
struct State {
    entries: HashMap<Vec<u8>, Vec<u8>>,
    /// Insertion order, oldest first; each cached key appears once.
    order: VecDeque<Vec<u8>>,
    size_bytes: usize,
}

impl InMemoryCacheStore {
    /// A store that holds at most `capacity_bytes` bytes of values.
    #[must_use]
    pub fn new(capacity_bytes: usize) -> Self {
        Self {
            capacity_bytes,
            state: Mutex::new(State::default()),
        }
    }

    /// The number of entries currently cached.
    #[must_use]
    pub fn len(&self) -> usize {
        self.state().entries.len()
    }

    /// Whether the cache is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The number of value bytes currently cached.
    #[must_use]
    pub fn size_bytes(&self) -> usize {
        self.state().size_bytes
    }

    fn state(&self) -> MutexGuard<'_, State> {
        // A panic while holding the lock must not disable the cache: the state
        // is still a consistent map and the next compilation can keep using it.
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

impl Default for InMemoryCacheStore {
    fn default() -> Self {
        Self::new(DEFAULT_CAPACITY_BYTES)
    }
}

impl CacheStore for InMemoryCacheStore {
    fn get(&self, key: &[u8]) -> Option<Cow<'_, [u8]>> {
        // The cache can evict at any time, so a borrowed slice cannot outlive
        // the lock guard. The store's public contract allows an owned value.
        self.state().entries.get(key).cloned().map(Cow::Owned)
    }

    fn insert(&self, key: &[u8], value: Vec<u8>) -> bool {
        if value.len() > self.capacity_bytes {
            return false;
        }
        let mut state = self.state();
        if let Some(previous) = state.entries.remove(key) {
            // Drop the old queue slot too, or it would later evict this new
            // value as if it were the oldest entry.
            state.size_bytes -= previous.len();
            state.order.retain(|queued| queued.as_slice() != key);
        }
        while state.size_bytes + value.len() > self.capacity_bytes {
            let Some(oldest) = state.order.pop_front() else {
                break;
            };
            if let Some(evicted) = state.entries.remove(&oldest) {
                state.size_bytes -= evicted.len();
            }
        }
        state.size_bytes += value.len();
        state.order.push_back(key.to_vec());
        state.entries.insert(key.to_vec(), value);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::{DEFAULT_CAPACITY_BYTES, InMemoryCacheStore};
    use wasmtime::CacheStore as _;

    #[test]
    fn stores_and_evicts_oldest_first() {
        let store = InMemoryCacheStore::new(8);

        assert!(store.insert(b"a", vec![1, 2, 3]));
        assert!(store.insert(b"b", vec![4, 5, 6]));
        assert_eq!(store.len(), 2);
        assert_eq!(store.size_bytes(), 6);
        assert_eq!(store.get(b"a").as_deref(), Some([1, 2, 3].as_slice()));

        // A third three-byte value exceeds the eight-byte budget, so `a` (the
        // oldest entry) is evicted and `b`/`c` remain.
        assert!(store.insert(b"c", vec![7, 8, 9]));
        assert_eq!(store.len(), 2);
        assert_eq!(store.size_bytes(), 6);
        assert!(store.get(b"a").is_none());
        assert_eq!(store.get(b"b").as_deref(), Some([4, 5, 6].as_slice()));
        assert_eq!(store.get(b"c").as_deref(), Some([7, 8, 9].as_slice()));
    }

    #[test]
    fn overwrites_do_not_double_count() {
        let store = InMemoryCacheStore::new(8);
        assert!(store.insert(b"a", vec![1, 2, 3]));
        assert!(store.insert(b"a", vec![4, 5]));
        assert_eq!(store.len(), 1);
        assert_eq!(store.size_bytes(), 2);
        assert_eq!(store.get(b"a").as_deref(), Some([4, 5].as_slice()));
    }

    #[test]
    fn an_overwrite_makes_the_entry_newest() {
        let store = InMemoryCacheStore::new(8);
        assert!(store.insert(b"a", vec![1, 2, 3]));
        assert!(store.insert(b"b", vec![4, 5, 6]));
        assert!(store.insert(b"a", vec![7, 8])); // a is now newer than b
        assert!(store.insert(b"c", vec![9, 10, 11])); // 8 bytes: fits
        assert!(store.insert(b"d", vec![12])); // 9 bytes: evicts b, not a
        assert!(store.get(b"b").is_none());
        assert_eq!(store.get(b"a").as_deref(), Some([7, 8].as_slice()));
        assert_eq!(store.size_bytes(), 6);
    }

    #[test]
    fn oversized_values_are_rejected() {
        let store = InMemoryCacheStore::new(4);
        assert!(!store.insert(b"big", vec![0; 5]));
        assert!(store.is_empty());
        assert_eq!(store.size_bytes(), 0);
    }

    #[test]
    fn default_capacity_is_256_mib() {
        assert_eq!(DEFAULT_CAPACITY_BYTES, 256 * 1024 * 1024);
        assert_eq!(
            InMemoryCacheStore::default().capacity_bytes,
            256 * 1024 * 1024
        );
    }
}
