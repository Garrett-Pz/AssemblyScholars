use dashmap::DashMap;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

pub const CACHE_LIMIT: usize = 1_000_000;
pub const USE_THRESHOLD: u32 = 2;

pub fn evict(cache: &DashMap<Vec<usize>, (usize, Vec<usize>, u32)>, threshold: u32,
            size: &AtomicUsize) {
    cache.retain(|_key, val| val.2 >= threshold || val.0 >= 5);

    for mut entry in cache.iter_mut() {
        entry.2 = 0;
    }

    size.store(cache.len(), Relaxed);
}
