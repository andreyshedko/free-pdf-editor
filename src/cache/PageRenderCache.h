#pragma once

#include <QHash>
#include <QImage>
#include <QString>

namespace cache {

class PageRenderCache {
public:
    void clear();
    bool contains(const QString& key) const;
    QImage get(const QString& key) const;
    void put(const QString& key, const QImage& image);

private:
    QHash<QString, QImage> m_items;
};

} // namespace cache
