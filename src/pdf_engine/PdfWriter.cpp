#include "pdf_engine/PdfWriter.h"

#include "document/Document.h"
#include "overlay/OverlayObject.h"
#include "pdf_engine/PdfRenderer.h"

#include <QPageLayout>
#include <QPageSize>
#include <QPainter>
#include <QPdfWriter>

namespace {

void drawOverlays(QPainter& painter, const document::PageModel& page) {
    for (const auto& overlay : page.overlayObjects) {
        if (!overlay) {
            continue;
        }

        switch (overlay->kind()) {
        case overlay::OverlayObject::Kind::Annotation: {
            const auto* a = static_cast<const overlay::AnnotationObject*>(overlay.get());
            painter.setPen(QPen(QColor(220, 48, 48), 2));
            painter.setBrush(QColor(255, 220, 220, 96));
            painter.drawRect(a->rect);
            painter.drawText(a->rect.adjusted(4, 4, -4, -4), a->text);
            break;
        }
        case overlay::OverlayObject::Kind::TextEdit: {
            const auto* t = static_cast<const overlay::TextEditObject*>(overlay.get());
            painter.setPen(QPen(QColor(40, 70, 220), 2));
            painter.setBrush(QColor(220, 230, 255, 90));
            painter.drawRoundedRect(t->rect, 4, 4);
            auto f = painter.font();
            f.setPointSizeF(t->fontSize);
            painter.setFont(f);
            painter.drawText(t->rect.adjusted(6, 4, -6, -4), t->text);
            break;
        }
        case overlay::OverlayObject::Kind::ImageEdit: {
            const auto* i = static_cast<const overlay::ImageObject*>(overlay.get());
            painter.drawImage(i->rect, i->image);
            painter.setPen(QPen(QColor(56, 128, 56), 2));
            painter.setBrush(Qt::NoBrush);
            painter.drawRect(i->rect);
            break;
        }
        case overlay::OverlayObject::Kind::Shape: {
            const auto* s = static_cast<const overlay::ShapeObject*>(overlay.get());
            painter.setPen(QPen(QColor(128, 64, 32), 2));
            painter.drawRect(s->rect);
            break;
        }
        }
    }
}

} // namespace

namespace pdf_engine {

bool PdfWriter::save(const document::Document& document, const QString& path, PdfRenderer& renderer) const {
    if (!document.isOpen()) {
        return false;
    }

    QPdfWriter writer(path);
    writer.setResolution(144);

    const int pages = document.pageCount();
    if (pages <= 0) {
        return false;
    }

    QPainter painter(&writer);
    if (!painter.isActive()) {
        return false;
    }

    for (int pageIndex = 0; pageIndex < pages; ++pageIndex) {
        const QImage page = renderer.renderPage(document, pageIndex, 1.0f);
        if (page.isNull()) {
            continue;
        }

        writer.setPageSize(QPageSize(QSizeF(page.width(), page.height()), QPageSize::Point));
        if (pageIndex > 0) {
            writer.newPage();
        }

        painter.drawImage(QRect(0, 0, page.width(), page.height()), page);
        drawOverlays(painter, document.page(pageIndex));
    }

    painter.end();
    return true;
}

bool PdfWriter::saveRenderedPages(const QVector<QImage>& pages, const QString& path) const {
    if (pages.isEmpty()) {
        return false;
    }

    QPdfWriter writer(path);
    writer.setResolution(144);

    QPainter painter(&writer);
    if (!painter.isActive()) {
        return false;
    }

    bool hasDrawnPage = false;
    for (const QImage& page : pages) {
        if (page.isNull()) {
            continue;
        }

        writer.setPageSize(QPageSize(QSizeF(page.width(), page.height()), QPageSize::Point));
        if (hasDrawnPage) {
            writer.newPage();
        }

        painter.drawImage(QRect(0, 0, page.width(), page.height()), page);
        hasDrawnPage = true;
    }

    painter.end();
    return hasDrawnPage;
}

} // namespace pdf_engine
