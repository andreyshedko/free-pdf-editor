#include "ui/PageView.h"

#include "overlay/OverlayObject.h"

#include <algorithm>
#include <QInputDialog>
#include <QHBoxLayout>
#include <QColorDialog>
#include <QKeyEvent>
#include <QLabel>
#include <QLineEdit>
#include <QMenu>
#include <QMouseEvent>
#include <QPainter>
#include <QSlider>
#include <QWidgetAction>
#include <QWheelEvent>

namespace {

struct ArrowStyle {
    QColor color {180, 80, 20};
    qreal width {3.0};
};

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

bool isArrowAnnotation(const QString& text) {
    if (!text.startsWith('[')) {
        return false;
    }
    const int end = text.indexOf(']');
    if (end <= 0) {
        return false;
    }
    const QString inside = text.mid(1, end - 1);
    const QString head = inside.section(';', 0, 0).trimmed();
    return head.compare(QStringLiteral("Arrow"), Qt::CaseInsensitive) == 0;
}

ArrowStyle parseArrowStyle(const QString& text) {
    ArrowStyle style;
    if (!isArrowAnnotation(text)) {
        return style;
    }

    const int end = text.indexOf(']');
    const QString inside = text.mid(1, end - 1);
    const QStringList parts = inside.split(';', Qt::SkipEmptyParts);
    for (int i = 1; i < parts.size(); ++i) {
        const QString part = parts.at(i).trimmed();
        const int eq = part.indexOf('=');
        if (eq <= 0) {
            continue;
        }
        const QString key = part.left(eq).trimmed().toLower();
        const QString value = part.mid(eq + 1).trimmed();
        if (key == QStringLiteral("color")) {
            const QColor parsed(value);
            if (parsed.isValid()) {
                style.color = parsed;
            }
        } else if (key == QStringLiteral("width")) {
            bool ok = false;
            const qreal w = value.toDouble(&ok);
            if (ok) {
                style.width = std::clamp(w, 1.0, 12.0);
            }
        }
    }
    return style;
}

QString composeArrowTag(const QString& original, const ArrowStyle& style) {
    const int end = original.indexOf(']');
    const QString body = end > 0 ? original.mid(end + 1).trimmed() : QString{};
    const QString tag = QStringLiteral("[Arrow;color=%1;width=%2]")
        .arg(style.color.name(QColor::HexRgb))
        .arg(QString::number(style.width, 'f', 1));
    if (body.isEmpty()) {
        return tag;
    }
    return QStringLiteral("%1 %2").arg(tag, body);
}

QString annotationTagPrefix(const QString& text) {
    if (!text.startsWith('[')) {
        return {};
    }
    const int end = text.indexOf(']');
    if (end <= 0) {
        return {};
    }
    return text.left(end + 1);
}

QString composeTaggedAnnotationText(const QString& original, const QString& body) {
    const QString tag = annotationTagPrefix(original);
    if (tag.isEmpty()) {
        return body.trimmed();
    }
    const QString trimmed = body.trimmed();
    if (trimmed.isEmpty()) {
        return tag;
    }
    return QStringLiteral("%1 %2").arg(tag, trimmed);
}

} // namespace

PageView::PageView(editor::EditorController& controller, QWidget* parent)
    : QWidget(parent), m_controller(controller) {
    setMinimumSize(640, 480);
    setAutoFillBackground(true);
    setFocusPolicy(Qt::StrongFocus);

    connect(&m_controller, &editor::EditorController::documentChanged, this, [this]() { update(); });
    connect(&m_controller, &editor::EditorController::pageChanged, this, [this]() {
        m_pan = {};
        m_activeOverlay = -1;
        m_dragMode = DragMode::None;
        update();
    });
}

void PageView::setZoom(float zoom) {
    const float clamped = std::clamp(zoom, 0.25f, 4.0f);
    if (qFuzzyCompare(m_zoom, clamped)) {
        return;
    }
    m_zoom = clamped;
    m_pan = clampPanForImageSize(m_lastPageImageSize, m_pan);
    emit zoomChanged(m_zoom);
    update();
}

void PageView::setActiveOverlay(int overlayIndex) {
    if (overlayIndex < 0) {
        m_activeOverlay = -1;
        update();
        return;
    }
    if (overlayIndex >= m_controller.currentPageOverlayCount()) {
        m_activeOverlay = -1;
        update();
        return;
    }
    m_activeOverlay = overlayIndex;
    update();
}

void PageView::setSearchQuery(QString query) {
    const QString normalized = query.trimmed();
    if (m_searchQuery == normalized) {
        return;
    }
    m_searchQuery = normalized;
    update();
}

void PageView::paintEvent(QPaintEvent*) {
    QPainter painter(this);
    painter.fillRect(rect(), QColor(235, 238, 243));

    const QImage pageImage = m_controller.renderCurrentPage(m_zoom);
    if (pageImage.isNull()) {
        m_lastTarget = {};
        m_lastPageImageSize = {};
        painter.setPen(QColor(80, 80, 80));
        painter.drawText(rect(), Qt::AlignCenter, QStringLiteral("Open PDF to start"));
        return;
    }

    const QSize targetSize = pageImage.size();
    const QPointF clampedPan = clampPanForImageSize(targetSize, m_pan);
    m_pan = clampedPan;
    const QRect target(
        static_cast<int>((width() - targetSize.width()) / 2.0 + clampedPan.x()),
        static_cast<int>((height() - targetSize.height()) / 2.0 + clampedPan.y()),
        targetSize.width(),
        targetSize.height());
    m_lastTarget = target;
    m_lastPageImageSize = pageImage.size();
    painter.drawImage(target, pageImage);

    const auto* pageModel = m_controller.currentPageModel();
    if (!pageModel) {
        return;
    }

    const qreal sx = static_cast<qreal>(target.width()) / static_cast<qreal>(pageImage.width());
    const qreal sy = static_cast<qreal>(target.height()) / static_cast<qreal>(pageImage.height());
    const qreal overlayScaleX = sx * static_cast<qreal>(m_zoom);
    const qreal overlayScaleY = sy * static_cast<qreal>(m_zoom);

    painter.save();
    painter.translate(target.topLeft());
    painter.scale(overlayScaleX, overlayScaleY);

    for (const auto& objPtr : pageModel->overlayObjects) {
        if (!objPtr) {
            continue;
        }

        switch (objPtr->kind()) {
        case overlay::OverlayObject::Kind::Annotation: {
            const auto* annotation = static_cast<const overlay::AnnotationObject*>(objPtr.get());
            const QString sourceText = annotation->text;
            const QString bodyText = stripAnnotationTag(sourceText);

            if (hasAnnotationTag(sourceText, "[highlight]")) {
                painter.setPen(Qt::NoPen);
                painter.setBrush(QColor(255, 235, 59, 135));
                painter.drawRect(annotation->rect);
                if (!bodyText.isEmpty()) {
                    painter.setPen(QColor(60, 60, 60));
                    painter.drawText(annotation->rect.adjusted(4, 2, -4, -2), Qt::AlignVCenter | Qt::AlignLeft, bodyText);
                }
            } else if (hasAnnotationTag(sourceText, "[underline]")) {
                painter.setPen(QPen(QColor(0, 105, 180), 2));
                painter.setBrush(QColor(220, 240, 255, 80));
                painter.drawRect(annotation->rect);
                painter.setPen(QColor(25, 25, 25));
                painter.drawText(annotation->rect.adjusted(6, 2, -6, -8), Qt::AlignVCenter | Qt::AlignLeft, bodyText);
                painter.setPen(QPen(QColor(0, 105, 180), 2));
                painter.drawLine(annotation->rect.bottomLeft() + QPointF(5, -4), annotation->rect.bottomRight() + QPointF(-5, -4));
            } else if (hasAnnotationTag(sourceText, "[strikeout]")) {
                painter.setPen(QPen(QColor(170, 40, 40), 2));
                painter.setBrush(QColor(255, 228, 228, 90));
                painter.drawRect(annotation->rect);
                painter.setPen(QColor(25, 25, 25));
                painter.drawText(annotation->rect.adjusted(6, 2, -6, -2), Qt::AlignVCenter | Qt::AlignLeft, bodyText);
                painter.setPen(QPen(QColor(170, 40, 40), 2));
                const qreal y = annotation->rect.center().y();
                painter.drawLine(QPointF(annotation->rect.left() + 5, y), QPointF(annotation->rect.right() - 5, y));
            } else if (hasAnnotationTag(sourceText, "[sticky note]")) {
                painter.setPen(QPen(QColor(145, 112, 20), 2));
                painter.setBrush(QColor(255, 244, 176));
                painter.drawRoundedRect(annotation->rect, 3, 3);
                const QRectF fold(annotation->rect.right() - 16, annotation->rect.top(), 16, 16);
                painter.setBrush(QColor(245, 219, 119));
                painter.drawPolygon(QPolygonF({
                    QPointF(fold.left(), fold.top()),
                    QPointF(fold.right(), fold.top()),
                    QPointF(fold.right(), fold.bottom())
                }));
                painter.setPen(QColor(60, 60, 60));
                painter.drawText(annotation->rect.adjusted(6, 6, -6, -6), Qt::AlignLeft | Qt::AlignTop | Qt::TextWordWrap, bodyText);
            } else if (hasAnnotationTag(sourceText, "[comment]")) {
                painter.setPen(QPen(QColor(90, 90, 90), 2));
                painter.setBrush(QColor(240, 240, 245, 170));
                painter.drawRoundedRect(annotation->rect, 8, 8);
                QPolygonF tail;
                tail << QPointF(annotation->rect.left() + 18, annotation->rect.bottom())
                     << QPointF(annotation->rect.left() + 30, annotation->rect.bottom())
                     << QPointF(annotation->rect.left() + 20, annotation->rect.bottom() + 12);
                painter.drawPolygon(tail);
                painter.setPen(QColor(35, 35, 35));
                painter.drawText(annotation->rect.adjusted(8, 6, -8, -8), Qt::AlignLeft | Qt::AlignTop | Qt::TextWordWrap, bodyText);
            } else if (isArrowAnnotation(sourceText)) {
                const ArrowStyle style = parseArrowStyle(sourceText);
                const QPointF start(annotation->rect.left(), annotation->rect.center().y());
                const QPointF end(annotation->rect.right(), annotation->rect.center().y());
                painter.setPen(QPen(style.color, style.width));
                painter.setBrush(style.color);
                painter.drawLine(start, end);

                const qreal headLen = std::min(18.0, std::max(10.0, annotation->rect.width() * 0.18));
                const qreal headHalf = std::max(5.0, annotation->rect.height() * 0.22 + style.width * 0.5);
                QPolygonF head;
                head << end
                     << QPointF(end.x() - headLen, end.y() - headHalf)
                     << QPointF(end.x() - headLen, end.y() + headHalf);
                painter.drawPolygon(head);
            } else {
                painter.setPen(QPen(QColor(220, 48, 48), 2));
                painter.setBrush(QColor(255, 220, 220, 96));
                painter.drawRect(annotation->rect);
                painter.drawText(annotation->rect.adjusted(4, 4, -4, -4), bodyText.isEmpty() ? sourceText : bodyText);
            }
            break;
        }
        case overlay::OverlayObject::Kind::TextEdit: {
            const auto* textEdit = static_cast<const overlay::TextEditObject*>(objPtr.get());
            painter.setPen(QPen(QColor(40, 70, 220), 2));
            painter.setBrush(QColor(220, 230, 255, 90));
            painter.drawRoundedRect(textEdit->rect, 4, 4);
            auto font = painter.font();
            if (!textEdit->fontFamily.trimmed().isEmpty()) {
                font.setFamily(textEdit->fontFamily);
            }
            font.setPointSizeF(textEdit->fontSize);
            painter.setFont(font);
            painter.drawText(textEdit->rect.adjusted(6, 4, -6, -4), textEdit->text);
            break;
        }
        case overlay::OverlayObject::Kind::ImageEdit: {
            const auto* imageEdit = static_cast<const overlay::ImageObject*>(objPtr.get());
            painter.drawImage(imageEdit->rect, imageEdit->image);
            painter.setPen(QPen(QColor(56, 128, 56), 2));
            painter.setBrush(Qt::NoBrush);
            painter.drawRect(imageEdit->rect);
            break;
        }
        case overlay::OverlayObject::Kind::Shape: {
            const auto* shape = static_cast<const overlay::ShapeObject*>(objPtr.get());
            painter.setPen(QPen(QColor(128, 64, 32), 2));
            painter.setBrush(Qt::NoBrush);
            painter.drawRect(shape->rect);
            break;
        }
        }
    }

    if (!m_searchQuery.isEmpty()) {
        for (const auto& objPtr : pageModel->overlayObjects) {
            if (!objPtr) {
                continue;
            }

            bool matchesSearch = false;
            if (objPtr->kind() == overlay::OverlayObject::Kind::TextEdit) {
                const auto* t = static_cast<const overlay::TextEditObject*>(objPtr.get());
                matchesSearch = t->text.contains(m_searchQuery, Qt::CaseInsensitive);
            } else if (objPtr->kind() == overlay::OverlayObject::Kind::Annotation) {
                const auto* a = static_cast<const overlay::AnnotationObject*>(objPtr.get());
                matchesSearch = a->text.contains(m_searchQuery, Qt::CaseInsensitive);
            }

            if (!matchesSearch) {
                continue;
            }

            const QRectF matchRect = overlayRect(objPtr.get());
            if (matchRect.isEmpty()) {
                continue;
            }

            painter.setPen(Qt::NoPen);
            painter.setBrush(QColor(255, 215, 0, 120));
            painter.drawRect(matchRect);
            painter.setPen(QPen(QColor(255, 186, 0), 2));
            painter.setBrush(Qt::NoBrush);
            painter.drawRect(matchRect);
        }
    }

    if (m_activeOverlay >= 0) {
        const auto* active = m_controller.currentPageOverlayAt(m_activeOverlay);
        const QRectF selectedRect = overlayRect(active);
        if (active && !selectedRect.isEmpty()) {
            painter.setBrush(QColor(255, 235, 59, 110));
            painter.setPen(Qt::NoPen);
            painter.drawRect(selectedRect);

            painter.setPen(QPen(QColor(250, 168, 64), 2, Qt::DashLine));
            painter.setBrush(Qt::NoBrush);
            painter.drawRect(selectedRect);

            const QRectF handle(selectedRect.right() - 8.0, selectedRect.bottom() - 8.0, 8.0, 8.0);
            painter.setBrush(QColor(250, 168, 64));
            painter.setPen(Qt::NoPen);
            painter.drawRect(handle);
        }
    }

    painter.restore();
}

void PageView::wheelEvent(QWheelEvent* event) {
    if (event->modifiers().testFlag(Qt::ControlModifier) && m_activeOverlay >= 0) {
        auto* overlay = m_controller.currentPageOverlayAt(m_activeOverlay);
        if (overlay && overlay->kind() == overlay::OverlayObject::Kind::TextEdit) {
            auto* textObj = static_cast<overlay::TextEditObject*>(overlay);
            const qreal step = event->angleDelta().y() > 0 ? 1.0 : -1.0;
            m_controller.setTextOverlayFontSize(m_activeOverlay, textObj->fontSize + step);
            event->accept();
            return;
        }
    }

    if (event->modifiers().testFlag(Qt::ControlModifier)) {
        const float factor = event->angleDelta().y() > 0 ? 1.1f : 0.9f;
        setZoom(m_zoom * factor);
        event->accept();
        return;
    }

    const qreal scrollStep = wheelScrollStep(event);
    const QPointF nextPan = clampPanForImageSize(m_lastPageImageSize, m_pan + QPointF(0.0, scrollStep));
    if (!qFuzzyCompare(m_pan.y() + 1.0, nextPan.y() + 1.0)) {
        m_pan = nextPan;
        update();
        event->accept();
        return;
    }

    event->ignore();
}

QRectF PageView::overlayRect(const overlay::OverlayObject* overlay) const {
    if (!overlay) {
        return {};
    }
    switch (overlay->kind()) {
    case overlay::OverlayObject::Kind::Annotation:
        return static_cast<const overlay::AnnotationObject*>(overlay)->rect;
    case overlay::OverlayObject::Kind::TextEdit:
        return static_cast<const overlay::TextEditObject*>(overlay)->rect;
    case overlay::OverlayObject::Kind::ImageEdit:
        return static_cast<const overlay::ImageObject*>(overlay)->rect;
    case overlay::OverlayObject::Kind::Shape:
        return static_cast<const overlay::ShapeObject*>(overlay)->rect;
    }
    return {};
}

QPointF PageView::widgetToPage(const QPointF& pos) const {
    if (m_lastTarget.isEmpty() || m_lastPageImageSize.isEmpty()) {
        return {-1.0, -1.0};
    }
    if (!m_lastTarget.adjusted(-1, -1, 1, 1).contains(pos.toPoint())) {
        return {-1.0, -1.0};
    }

    const qreal zoom = std::max(0.01, static_cast<double>(m_zoom));
    const qreal x = ((pos.x() - m_lastTarget.left()) * m_lastPageImageSize.width() / std::max(1, m_lastTarget.width())) / zoom;
    const qreal y = ((pos.y() - m_lastTarget.top()) * m_lastPageImageSize.height() / std::max(1, m_lastTarget.height())) / zoom;
    return {x, y};
}

bool PageView::isPointOnResizeHandle(const QPointF& pagePos, const QRectF& rect) const {
    const QRectF handle(rect.right() - 10.0, rect.bottom() - 10.0, 12.0, 12.0);
    return handle.contains(pagePos);
}

int PageView::overlayAtPagePoint(const QPointF& pagePos) const {
    if (pagePos.x() < 0.0 || pagePos.y() < 0.0) {
        return -1;
    }
    const auto* page = m_controller.currentPageModel();
    if (!page) {
        return -1;
    }

    for (int i = static_cast<int>(page->overlayObjects.size()) - 1; i >= 0; --i) {
        const auto& obj = page->overlayObjects[static_cast<size_t>(i)];
        if (!obj) {
            continue;
        }
        if (overlayRect(obj.get()).contains(pagePos)) {
            return i;
        }
    }
    return -1;
}

QPointF PageView::clampPanForImageSize(const QSize& imageSize, const QPointF& pan) const {
    if (imageSize.isEmpty()) {
        return {};
    }

    const qreal overflowX = std::max(0.0, static_cast<double>(imageSize.width() - width()));
    const qreal overflowY = std::max(0.0, static_cast<double>(imageSize.height() - height()));

    const qreal minX = -overflowX / 2.0;
    const qreal maxX = overflowX / 2.0;
    const qreal minY = -overflowY / 2.0;
    const qreal maxY = overflowY / 2.0;

    return {
        std::clamp(pan.x(), minX, maxX),
        std::clamp(pan.y(), minY, maxY)
    };
}

qreal PageView::wheelScrollStep(const QWheelEvent* event) const {
    if (!event->pixelDelta().isNull()) {
        return event->pixelDelta().y();
    }

    constexpr qreal pixelsPerWheelStep = 56.0;
    return (static_cast<qreal>(event->angleDelta().y()) / 120.0) * pixelsPerWheelStep;
}

void PageView::openOverlayContextMenu(const QPoint& globalPos, int overlayIndex) {
    auto* selected = m_controller.currentPageOverlayAt(overlayIndex);
    if (!selected) {
        return;
    }

    QMenu menu(this);
    if (selected->kind() == overlay::OverlayObject::Kind::ImageEdit) {
        auto* rotateCw = menu.addAction(tr("Rotate 90° CW"));
        auto* rotateCcw = menu.addAction(tr("Rotate 90° CCW"));
        auto* flipH = menu.addAction(tr("Flip Horizontal"));
        auto* flipV = menu.addAction(tr("Flip Vertical"));
        menu.addSeparator();
        auto* deleteImage = menu.addAction(tr("Delete Image"));
        auto* chosen = menu.exec(globalPos);
        if (chosen == rotateCw) {
            m_controller.rotateImageOverlay(overlayIndex, true);
        } else if (chosen == rotateCcw) {
            m_controller.rotateImageOverlay(overlayIndex, false);
        } else if (chosen == flipH) {
            m_controller.flipImageOverlay(overlayIndex, true);
        } else if (chosen == flipV) {
            m_controller.flipImageOverlay(overlayIndex, false);
        } else if (chosen == deleteImage) {
            if (m_controller.deleteOverlayAt(overlayIndex)) {
                m_activeOverlay = -1;
            }
        }
        return;
    }

    if (selected->kind() == overlay::OverlayObject::Kind::TextEdit) {
        auto* increaseFont = menu.addAction(tr("Increase Font Size"));
        auto* decreaseFont = menu.addAction(tr("Decrease Font Size"));
        auto* editText = menu.addAction(tr("Edit Text"));
        menu.addSeparator();
        auto* deleteText = menu.addAction(tr("Delete Text"));
        auto* chosen = menu.exec(globalPos);
        auto* textObj = static_cast<overlay::TextEditObject*>(selected);
        if (chosen == increaseFont) {
            m_controller.setTextOverlayFontSize(overlayIndex, textObj->fontSize + 1.0);
        } else if (chosen == decreaseFont) {
            m_controller.setTextOverlayFontSize(overlayIndex, textObj->fontSize - 1.0);
        } else if (chosen == editText) {
            bool ok = false;
            const QString updated = QInputDialog::getText(this, tr("Edit Text"), tr("Text:"), QLineEdit::Normal, textObj->text, &ok);
            if (ok) {
                m_controller.setTextOverlayText(overlayIndex, updated);
            }
        } else if (chosen == deleteText) {
            if (m_controller.deleteOverlayAt(overlayIndex)) {
                m_activeOverlay = -1;
            }
        }
        return;
    }

    if (selected->kind() == overlay::OverlayObject::Kind::Annotation) {
        auto* annotation = static_cast<overlay::AnnotationObject*>(selected);
        const bool isArrow = isArrowAnnotation(annotation->text);
        const QString baseArrowText = annotation->text;

        QAction* pickColor = nullptr;
        QAction* editAnnotation = nullptr;

        ArrowStyle style;
        if (isArrow) {
            style = parseArrowStyle(annotation->text);

            auto* thicknessWidget = new QWidget(&menu);
            auto* thicknessLayout = new QHBoxLayout(thicknessWidget);
            thicknessLayout->setContentsMargins(8, 4, 8, 4);
            thicknessLayout->setSpacing(8);

            auto* thicknessLabel = new QLabel(tr("Arrow Thickness"), thicknessWidget);
            auto* thicknessSlider = new QSlider(Qt::Horizontal, thicknessWidget);
            thicknessSlider->setRange(1, 12);
            thicknessSlider->setValue(static_cast<int>(std::round(style.width)));
            thicknessSlider->setFixedWidth(130);

            auto* thicknessValueLabel = new QLabel(QString::number(static_cast<int>(std::round(style.width))), thicknessWidget);
            thicknessValueLabel->setMinimumWidth(20);

            thicknessLayout->addWidget(thicknessLabel);
            thicknessLayout->addWidget(thicknessSlider);
            thicknessLayout->addWidget(thicknessValueLabel);

            auto* thicknessAction = new QWidgetAction(&menu);
            thicknessAction->setDefaultWidget(thicknessWidget);
            menu.addAction(thicknessAction);

            connect(thicknessSlider, &QSlider::valueChanged, &menu, [&, thicknessValueLabel](int value) {
                style.width = static_cast<qreal>(value);
                thicknessValueLabel->setText(QString::number(value));
                m_controller.setAnnotationOverlayText(overlayIndex, composeArrowTag(baseArrowText, style));
            });

            menu.addSeparator();
            pickColor = menu.addAction(tr("Arrow Color..."));
            menu.addSeparator();
        }

        editAnnotation = menu.addAction(tr("Edit Annotation Text"));
        auto* deleteOverlay = menu.addAction(tr("Delete Overlay"));
        auto* chosen = menu.exec(globalPos);

        if (isArrow && chosen == pickColor) {
            const QColor chosenColor = QColorDialog::getColor(style.color, this, tr("Arrow Color"));
            if (!chosenColor.isValid()) {
                return;
            }
            style.color = chosenColor;
            m_controller.setAnnotationOverlayText(overlayIndex, composeArrowTag(baseArrowText, style));
            return;
        }

        if (chosen == editAnnotation) {
            bool ok = false;
            const QString updatedBody = QInputDialog::getMultiLineText(
                this,
                tr("Edit Annotation"),
                tr("Text:"),
                stripAnnotationTag(annotation->text),
                &ok);
            if (ok) {
                const QString merged = composeTaggedAnnotationText(annotation->text, updatedBody);
                m_controller.setAnnotationOverlayText(overlayIndex, merged);
            }
            return;
        }
        if (chosen == deleteOverlay) {
            if (m_controller.deleteOverlayAt(overlayIndex)) {
                m_activeOverlay = -1;
            }
        }
        return;
    }

    if (selected->kind() == overlay::OverlayObject::Kind::Shape) {
        auto* deleteOverlay = menu.addAction(tr("Delete Overlay"));
        auto* chosen = menu.exec(globalPos);
        if (chosen == deleteOverlay) {
            if (m_controller.deleteOverlayAt(overlayIndex)) {
                m_activeOverlay = -1;
            }
        }
    }
}

void PageView::mousePressEvent(QMouseEvent* event) {
    setFocus();
    const QPointF pagePos = widgetToPage(event->position());
    const int hit = overlayAtPagePoint(pagePos);

    if (event->button() == Qt::RightButton) {
        if (hit >= 0) {
            m_activeOverlay = hit;
            openOverlayContextMenu(event->globalPosition().toPoint(), hit);
            update();
            event->accept();
            return;
        }
    }

    if (event->button() != Qt::LeftButton) {
        QWidget::mousePressEvent(event);
        return;
    }

    if (hit < 0 && m_zoom > 1.0f) {
        m_dragMode = DragMode::Pan;
        m_lastMousePos = event->pos();
        m_activeOverlay = -1;
        update();
        event->accept();
        return;
    }

    m_activeOverlay = hit;
    if (m_activeOverlay >= 0) {
        auto* overlay = m_controller.currentPageOverlayAt(m_activeOverlay);
        const QRectF rect = overlayRect(overlay);
        m_dragStartPage = pagePos;
        m_dragStartRect = rect;
        m_dragMode = isPointOnResizeHandle(pagePos, rect) ? DragMode::Resize : DragMode::Move;
        update();
        event->accept();
        return;
    }

    m_dragMode = DragMode::None;
    update();
    QWidget::mousePressEvent(event);
}

void PageView::mouseMoveEvent(QMouseEvent* event) {
    if (m_activeOverlay < 0 || m_dragMode == DragMode::None || !(event->buttons() & Qt::LeftButton)) {
        if (m_dragMode == DragMode::Pan && (event->buttons() & Qt::LeftButton)) {
            const QPoint delta = event->pos() - m_lastMousePos;
            m_lastMousePos = event->pos();
            m_pan = clampPanForImageSize(m_lastPageImageSize, m_pan + QPointF(delta.x(), delta.y()));
            update();
            event->accept();
            return;
        }
        QWidget::mouseMoveEvent(event);
        return;
    }

    const QPointF pagePos = widgetToPage(event->position());
    if (pagePos.x() < 0.0 || pagePos.y() < 0.0) {
        return;
    }

    if (m_dragMode == DragMode::Move) {
        const QPointF delta = pagePos - m_dragStartPage;
        m_controller.moveOverlayBy(m_activeOverlay, delta);
        m_dragStartPage = pagePos;
        return;
    }

    QRectF resized = m_dragStartRect;
    resized.setBottomRight(pagePos);
    m_controller.setOverlayRect(m_activeOverlay, resized);
}

void PageView::mouseReleaseEvent(QMouseEvent* event) {
    if (event->button() == Qt::LeftButton) {
        m_dragMode = DragMode::None;
    }
    QWidget::mouseReleaseEvent(event);
}

void PageView::mouseDoubleClickEvent(QMouseEvent* event) {
    if (event->button() != Qt::LeftButton) {
        QWidget::mouseDoubleClickEvent(event);
        return;
    }

    const QPointF pagePos = widgetToPage(event->position());
    const int hit = overlayAtPagePoint(pagePos);
    if (hit < 0) {
        QWidget::mouseDoubleClickEvent(event);
        return;
    }

    auto* overlay = m_controller.currentPageOverlayAt(hit);
    if (!overlay) {
        QWidget::mouseDoubleClickEvent(event);
        return;
    }

    if (overlay->kind() == overlay::OverlayObject::Kind::TextEdit) {
        auto* textObj = static_cast<overlay::TextEditObject*>(overlay);
        bool ok = false;
        const QString updated = QInputDialog::getText(this, tr("Edit Text"), tr("Text:"), QLineEdit::Normal, textObj->text, &ok);
        if (ok) {
            m_activeOverlay = hit;
            m_controller.setTextOverlayText(hit, updated);
        }
        return;
    }

    if (overlay->kind() == overlay::OverlayObject::Kind::Annotation) {
        auto* annotation = static_cast<overlay::AnnotationObject*>(overlay);
        bool ok = false;
        const QString updatedBody = QInputDialog::getMultiLineText(
            this,
            tr("Edit Annotation"),
            tr("Text:"),
            stripAnnotationTag(annotation->text),
            &ok);
        if (ok) {
            m_activeOverlay = hit;
            m_controller.setAnnotationOverlayText(hit, composeTaggedAnnotationText(annotation->text, updatedBody));
        }
        return;
    }

    QWidget::mouseDoubleClickEvent(event);
}

void PageView::keyPressEvent(QKeyEvent* event) {
    if ((event->key() == Qt::Key_Delete || event->key() == Qt::Key_Backspace) && m_activeOverlay >= 0) {
        auto* overlay = m_controller.currentPageOverlayAt(m_activeOverlay);
        if (overlay) {
            if (m_controller.deleteOverlayAt(m_activeOverlay)) {
                m_activeOverlay = -1;
                event->accept();
                return;
            }
        }
    }
    QWidget::keyPressEvent(event);
}
