#pragma once

#include "cache/PageRenderCache.h"

#include <QImage>
#include <QString>

namespace document { class Document; }

namespace pdf_engine {

class PdfiumBridge;

class PdfRenderer {
public:
    PdfRenderer();
    ~PdfRenderer();

    bool open(const document::Document& document);
    QImage renderPage(const document::Document& document, int pageIndex, float scale) const;
    QString extractText(const document::Document& document, int pageIndex) const;

private:
    [[nodiscard]] QString cacheKey(const document::Document& document, int pageIndex, float scale) const;

    int m_pageCount {0};
    QString m_docPath;
    mutable cache::PageRenderCache m_cache;
    PdfiumBridge* m_pdfium {nullptr};
};

} // namespace pdf_engine
