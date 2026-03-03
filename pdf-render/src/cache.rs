use crate::types::{CacheKey, RenderedPage};
use lru::LruCache;
use std::num::NonZeroUsize;
use tracing::{debug, instrument};

pub struct PageCache {
    inner: LruCache<CacheKey, RenderedPage>,
}

impl PageCache {
    pub fn new(capacity: usize) -> Self {
        let cap = NonZeroUsize::new(capacity.max(1)).unwrap();
        Self { inner: LruCache::new(cap) }
    }

    #[instrument(name = "cache_insert", skip(self, page), fields(page_index = page.page_index))]
    pub fn insert(&mut self, key: CacheKey, page: RenderedPage) {
        self.inner.put(key, page);
    }

    #[instrument(name = "cache_lookup", skip(self))]
    pub fn get(&mut self, key: &CacheKey) -> Option<&RenderedPage> {
        let hit = self.inner.get(key);
        if hit.is_some() {
            debug!(?key, "cache hit");
        } else {
            debug!(?key, "cache miss");
        }
        hit
    }

    pub fn evict_document(&mut self, document_id: u64) {
        let keys_to_remove: Vec<CacheKey> = self.inner
            .iter()
            .filter(|(k, _)| k.document_id == document_id)
            .map(|(k, _)| k.clone())
            .collect();
        for k in keys_to_remove {
            self.inner.pop(&k);
        }
    }

    pub fn len(&self) -> usize { self.inner.len() }
    pub fn is_empty(&self) -> bool { self.inner.is_empty() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_page(idx: u32) -> RenderedPage {
        RenderedPage { data: vec![255; 4], width: 1, height: 1, page_index: idx }
    }

    #[test]
    fn lru_eviction() {
        let mut cache = PageCache::new(2);
        let k0 = CacheKey::new(1, 0, 1.0);
        let k1 = CacheKey::new(1, 1, 1.0);
        let k2 = CacheKey::new(1, 2, 1.0);
        cache.insert(k0.clone(), make_page(0));
        cache.insert(k1.clone(), make_page(1));
        assert!(cache.get(&k0).is_some());
        cache.insert(k2.clone(), make_page(2));
        assert_eq!(cache.len(), 2);
        assert!(cache.get(&k0).is_some());
        assert!(cache.get(&k1).is_none());
        assert!(cache.get(&k2).is_some());
    }

    #[test]
    fn evict_document() {
        let mut cache = PageCache::new(10);
        for p in 0..3u32 {
            cache.insert(CacheKey::new(1, p, 1.0), make_page(p));
        }
        cache.insert(CacheKey::new(2, 0, 1.0), make_page(0));
        cache.evict_document(1);
        assert_eq!(cache.len(), 1);
        assert!(cache.get(&CacheKey::new(2, 0, 1.0)).is_some());
    }
}
