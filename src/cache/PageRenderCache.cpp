#include "cache/PageRenderCache.h"

namespace cache {

void PageRenderCache::clear() {
    m_items.clear();
}

bool PageRenderCache::contains(const QString& key) const {
    return m_items.contains(key);
}

QImage PageRenderCache::get(const QString& key) const {
    return m_items.value(key);
}

void PageRenderCache::put(const QString& key, const QImage& image) {
    m_items.insert(key, image);
}

} // namespace cache
