use dashmap::DashMap;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

pub const CACHE_LIMIT: usize = 1_000_000;
pub const REMOVE_SIZE: usize = 10_000;
pub const USE_THRESHOLD: u32 = 2;

pub fn evict(cache: &DashMap<Vec<usize>, (usize, Vec<usize>, u32)>, threshold: u32,
            size: &AtomicUsize) {
    cache.retain(|_key, val| val.2 >= threshold || val.0 >= 5);

    for mut entry in cache.iter_mut() {
        entry.2 = 0;
    }

    size.store(cache.len(), Relaxed);
}

pub fn evict_lru(cache: &DashMap<Vec<usize>, (usize, Vec<usize>, u32)>, removecount: usize, size: &AtomicUsize) {
    //make a vec of the cache's use times
    let mut counts : Vec<u32> = cache
        .iter()
        .map(|e| e.value().2)
        .collect();
    //get the item that would be at the REMOVE_ITEM position, 
    let (_, &mut cutoff, _) = counts.select_nth_unstable(removecount-1);
    //might be better way to do these last 2 lines ^expensive
    //and retain all items that are used after the cutoff
    cache.retain(|_key, value| value.2 > cutoff);
    size.store(cache.len(), Relaxed);
}