#include "pdf_engine/PdfWriter.h"

#include "document/Document.h"
#include "overlay/OverlayObject.h"
#include "pdf_engine/PdfRenderer.h"

#include <QPageLayout>
#include <QPageSize>
#include <QPainter>
#include <QPdfWriter>

namespace {

QString stripAnnotationTag(QString text) {
    if (text.startsWith('[')) {
        const int end = text.indexOf(']');
        if (end > 0) {
            text = text.mid(end + 1).trimmed();
        }
    }
    return text;
}

bool hasAnnotationTag(const QString& text, const char* tag) {
    return text.contains(QString::fromLatin1(tag), Qt::CaseInsensitive);
}

void drawOverlays(QPainter& painter, const document::PageModel& page) {
    for (const auto& overlay : page.overlayObjects) {
        if (!overlay) {
            continue;
        }

        switch (overlay->kind()) {
        case overlay::OverlayObject::Kind::Annotation: {
            const auto* a = static_cast<const overlay::AnnotationObject*>(overlay.get());
            const QString sourceText = a->text;
            const QString bodyText = stripAnnotationTag(sourceText);

            if (hasAnnotationTag(sourceText, "[highlight]")) {
                painter.setPen(Qt::NoPen);
                painter.setBrush(QColor(255, 235, 59, 135));
                painter.drawRect(a->rect);
                if (!bodyText.isEmpty()) {
                    painter.setPen(QColor(60, 60, 60));
                    painter.drawText(a->rect.adjusted(4, 2, -4, -2), Qt::AlignVCenter | Qt::AlignLeft, bodyText);
                }
            } else if (hasAnnotationTag(sourceText, "[underline]")) {
                painter.setPen(QPen(QColor(0, 105, 180), 2));
                painter.setBrush(QColor(220, 240, 255, 80));
                painter.drawRect(a->rect);
                painter.setPen(QColor(25, 25, 25));
                painter.drawText(a->rect.adjusted(6, 2, -6, -8), Qt::AlignVCenter | Qt::AlignLeft, bodyText);
                painter.setPen(QPen(QColor(0, 105, 180), 2));
                painter.drawLine(a->rect.bottomLeft() + QPointF(5, -4), a->rect.bottomRight() + QPointF(-5, -4));
            } else if (hasAnnotationTag(sourceText, "[strikeout]")) {
                painter.setPen(QPen(QColor(170, 40, 40), 2));
                painter.setBrush(QColor(255, 228, 228, 90));
                painter.drawRect(a->rect);
                painter.setPen(QColor(25, 25, 25));
                painter.drawText(a->rect.adjusted(6, 2, -6, -2), Qt::AlignVCenter | Qt::AlignLeft, bodyText);
                painter.setPen(QPen(QColor(170, 40, 40), 2));
                const qreal y = a->rect.center().y();
                painter.drawLine(QPointF(a->rect.left() + 5, y), QPointF(a->rect.right() - 5, y));
            } else if (hasAnnotationTag(sourceText, "[sticky note]")) {
                painter.setPen(QPen(QColor(145, 112, 20), 2));
                painter.setBrush(QColor(255, 244, 176));
                painter.drawRoundedRect(a->rect, 3, 3);
                const QRectF fold(a->rect.right() - 16, a->rect.top(), 16, 16);
                painter.setBrush(QColor(245, 219, 119));
                painter.drawPolygon(QPolygonF({
                    QPointF(fold.left(), fold.top()),
                    QPointF(fold.right(), fold.top()),
                    QPointF(fold.right(), fold.bottom())
                }));
                painter.setPen(QColor(60, 60, 60));
                painter.drawText(a->rect.adjusted(6, 6, -6, -6), Qt::AlignLeft | Qt::AlignTop | Qt::TextWordWrap, bodyText);
            } else if (hasAnnotationTag(sourceText, "[comment]")) {
                painter.setPen(QPen(QColor(90, 90, 90), 2));
                painter.setBrush(QColor(240, 240, 245, 170));
                painter.drawRoundedRect(a->rect, 8, 8);
                QPolygonF tail;
                tail << QPointF(a->rect.left() + 18, a->rect.bottom())
                     << QPointF(a->rect.left() + 30, a->rect.bottom())
                     << QPointF(a->rect.left() + 20, a->rect.bottom() + 12);
                painter.drawPolygon(tail);
                painter.setPen(QColor(35, 35, 35));
                painter.drawText(a->rect.adjusted(8, 6, -8, -8), Qt::AlignLeft | Qt::AlignTop | Qt::TextWordWrap, bodyText);
            } else {
                painter.setPen(QPen(QColor(220, 48, 48), 2));
                painter.setBrush(QColor(255, 220, 220, 96));
                painter.drawRect(a->rect);
                painter.drawText(a->rect.adjusted(4, 4, -4, -4), bodyText.isEmpty() ? sourceText : bodyText);
            }
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
