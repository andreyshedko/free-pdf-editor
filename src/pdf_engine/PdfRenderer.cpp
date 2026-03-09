#include "pdf_engine/PdfRenderer.h"

#include "document/Document.h"
#include "pdf_engine/PdfiumBridge.h"

#include <algorithm>
#include <QColor>
#include <QPainter>

namespace pdf_engine {

PdfRenderer::PdfRenderer()
    : m_pdfium(new PdfiumBridge) {}

PdfRenderer::~PdfRenderer() {
    delete m_pdfium;
}

bool PdfRenderer::open(const document::Document& document) {
    const QString newPath = document.path();
    if (newPath != m_docPath) {
        m_cache.clear();
        m_docPath = newPath;
    }
    if (m_pdfium && m_pdfium->isAvailable()) {
        const int realCount = m_pdfium->pageCount(document.sourceBytes());
        m_pageCount = realCount > 0 ? realCount : document.pageCount();
    } else {
        m_pageCount = document.pageCount();
    }
    return m_pageCount > 0;
}

QImage PdfRenderer::renderPage(const document::Document& document, int pageIndex, float scale) const {
    const QString key = cacheKey(document, pageIndex, scale);
    if (m_cache.contains(key)) {
        return m_cache.get(key);
    }

    const float clampedScale = std::clamp(scale, 0.25f, 4.0f);
    QImage image;
    if (m_pdfium && m_pdfium->isAvailable()) {
        image = m_pdfium->renderPage(document.sourceBytes(), pageIndex, clampedScale);
    }

    if (image.isNull()) {
        const int width = static_cast<int>(900 * clampedScale);
        const int height = static_cast<int>(1200 * clampedScale);
        image = QImage(width, height, QImage::Format_ARGB32_Premultiplied);
        image.fill(QColor(250, 250, 250));

        QPainter painter(&image);
        painter.setRenderHint(QPainter::Antialiasing, true);
        painter.setPen(QPen(QColor(205, 205, 205), 2));
        painter.drawRect(image.rect().adjusted(1, 1, -2, -2));
        painter.setPen(QColor(120, 120, 120));
        painter.drawText(image.rect(), Qt::AlignCenter,
            QStringLiteral("Page %1\n(PDFium unavailable)").arg(pageIndex + 1));
    }

    m_cache.put(key, image);
    return image;
}

QString PdfRenderer::cacheKey(const document::Document& document, int pageIndex, float scale) const {
    const int scaleKey = static_cast<int>(scale * 100.0f);
    return QStringLiteral("%1:%2:%3").arg(document.path()).arg(pageIndex).arg(scaleKey);
}

} // namespace pdf_engine
