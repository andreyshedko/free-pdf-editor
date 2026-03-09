#include "document/Document.h"

#include <QFile>
#include <QFileInfo>

#include <algorithm>

namespace {

int detectPageCountHeuristic(const QByteArray& bytes) {
    const QByteArray marker("/Type /Page");
    int count = 0;
    int from = 0;
    while (true) {
        const int idx = bytes.indexOf(marker, from);
        if (idx < 0) {
            break;
        }
        ++count;
        from = idx + marker.size();
    }
    return std::max(1, count);
}

} // namespace

namespace document {

bool Document::open(const QString& path) {
    QFileInfo info(path);
    if (!info.exists() || !info.isFile()) {
        return false;
    }

    QFile file(info.absoluteFilePath());
    if (!file.open(QIODevice::ReadOnly)) {
        return false;
    }

    m_sourceBytes = file.readAll();
    m_path = info.absoluteFilePath();
    m_metadata.title = info.completeBaseName();

    const int pages = detectPageCountHeuristic(m_sourceBytes);
    m_pages.clear();
    m_pages.reserve(static_cast<size_t>(pages));

    for (int i = 0; i < pages; ++i) {
        PageModel page;
        page.pageNumber = i;
        m_pages.push_back(std::move(page));
    }

    return true;
}

bool Document::save(const QString& path) {
    if (!isOpen()) {
        return false;
    }
    m_path = path;
    return true;
}

bool Document::isOpen() const {
    return !m_pages.empty();
}

const QString& Document::path() const {
    return m_path;
}

const QByteArray& Document::sourceBytes() const {
    return m_sourceBytes;
}

const DocumentMetadata& Document::metadata() const {
    return m_metadata;
}

int Document::pageCount() const {
    return static_cast<int>(m_pages.size());
}

PageModel& Document::page(int index) {
    return m_pages.at(static_cast<size_t>(index));
}

const PageModel& Document::page(int index) const {
    return m_pages.at(static_cast<size_t>(index));
}

bool Document::deletePage(int index, PageModel* removedPage) {
    if (!isOpen() || index < 0 || index >= pageCount() || pageCount() <= 1) {
        return false;
    }

    const auto it = m_pages.begin() + index;
    if (removedPage) {
        *removedPage = *it;
    }
    m_pages.erase(it);
    renumberPages();
    return true;
}

bool Document::insertPage(int index, const PageModel& pageModel) {
    if (index < 0 || index > pageCount()) {
        return false;
    }
    m_pages.insert(m_pages.begin() + index, pageModel);
    renumberPages();
    return true;
}

void Document::reset() {
    m_path.clear();
    m_sourceBytes.clear();
    m_pages.clear();
    m_metadata = {};
}

void Document::renumberPages() {
    for (int i = 0; i < static_cast<int>(m_pages.size()); ++i) {
        m_pages[static_cast<size_t>(i)].pageNumber = i;
    }
}

} // namespace document
