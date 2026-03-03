//! LRU page render cache.
//!
//! # Thread ownership
//! `PageCache` is owned exclusively by the core loop thread.
//!
//! # Safety
//! No unsafe code.

use lru::LruCache;
use shared::PageCacheKey;
use std::num::NonZeroUsize;
use tracing::{instrument, span, Level};

/// A single cached rendered page.
#[derive(Debug, Clone)]
pub struct CachedPage {
    /// Raw RGBA pixel data (row-major, 4 bytes per pixel).
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    /// Monotonically increasing generation counter used for invalidation.
    pub generation: u64,
}

/// LRU cache for rendered page bitmaps.
///
/// The cache key is `(document_id, page_index, zoom_hundredths)`.
/// Eviction is automatic: the least-recently-used page is evicted when
/// `capacity` is exceeded.
pub struct PageCache {
    inner: LruCache<PageCacheKey, CachedPage>,
    generation: u64,
}

impl PageCache {
    /// Create a cache that holds at most `capacity` pages.
    pub fn new(capacity: usize) -> Self {
        let cap = NonZeroUsize::new(capacity).expect("cache capacity must be > 0");
        Self {
            inner: LruCache::new(cap),
            generation: 0,
        }
    }

    /// Insert a rendered page into the cache.
    #[instrument(name = "cache_miss", skip(self, data))]
    pub fn insert(&mut self, key: PageCacheKey, data: Vec<u8>, width: u32, height: u32) {
        self.generation += 1;
        self.inner.put(
            key,
            CachedPage {
                data,
                width,
                height,
                generation: self.generation,
            },
        );
    }

    /// Look up a page in the cache.
    ///
    /// Accessing an entry promotes it to most-recently-used.
    #[instrument(name = "cache_hit", skip(self))]
    pub fn get(&mut self, key: &PageCacheKey) -> Option<&CachedPage> {
        let _span = span!(Level::DEBUG, "cache_lookup").entered();
        self.inner.get(key)
    }

    /// Evict all entries whose `document_id` matches `id`.
    ///
    /// Used when a document is closed or zoom changes invalidate the cache.
    pub fn evict_document(&mut self, document_id: u64) {
        let keys_to_remove: Vec<PageCacheKey> = self
            .inner
            .iter()
            .filter(|(k, _)| k.document_id == document_id)
            .map(|(k, _)| k.clone())
            .collect();
        for key in keys_to_remove {
            self.inner.pop(&key);
        }
    }

    /// Return the number of cached pages.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Return `true` if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_page(gen_hint: u8) -> (Vec<u8>, u32, u32) {
        (vec![gen_hint; 100 * 4], 10, 10) // 10×10 stub bitmap
    }

    #[test]
    fn insert_and_retrieve() {
        let mut cache = PageCache::new(10);
        let key = PageCacheKey::new(1, 0, 1.0);
        let (data, w, h) = dummy_page(1);
        cache.insert(key.clone(), data.clone(), w, h);

        let page = cache.get(&key).expect("should be cached");
        assert_eq!(page.width, 10);
        assert_eq!(page.height, 10);
        assert_eq!(page.data, data);
    }

    #[test]
    fn lru_eviction() {
        let mut cache = PageCache::new(2); // capacity = 2

        let key0 = PageCacheKey::new(1, 0, 1.0);
        let key1 = PageCacheKey::new(1, 1, 1.0);
        let key2 = PageCacheKey::new(1, 2, 1.0);

        let (d, w, h) = dummy_page(0);
        cache.insert(key0.clone(), d, w, h);
        let (d, w, h) = dummy_page(1);
        cache.insert(key1.clone(), d, w, h);

        // Access key0 to make it most-recently-used.
        assert!(cache.get(&key0).is_some());

        // Inserting key2 should evict key1 (least recently used).
        let (d, w, h) = dummy_page(2);
        cache.insert(key2.clone(), d, w, h);

        assert_eq!(cache.len(), 2);
        assert!(cache.get(&key0).is_some(), "key0 should still be cached");
        assert!(cache.get(&key1).is_none(), "key1 should have been evicted");
        assert!(cache.get(&key2).is_some(), "key2 should be cached");
    }

    #[test]
    fn evict_document_removes_matching_entries() {
        let mut cache = PageCache::new(10);

        for page in 0u32..3 {
            let key = PageCacheKey::new(1, page, 1.0);
            let (d, w, h) = dummy_page(page as u8);
            cache.insert(key, d, w, h);
        }
        // Add an entry for a different document.
        let other_key = PageCacheKey::new(2, 0, 1.0);
        let (d, w, h) = dummy_page(99);
        cache.insert(other_key.clone(), d, w, h);

        cache.evict_document(1);

        assert_eq!(cache.len(), 1, "only the other-document entry should remain");
        assert!(cache.get(&other_key).is_some());
    }
}
