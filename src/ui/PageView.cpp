#include "ui/PageView.h"

#include "overlay/OverlayObject.h"

#include <algorithm>
#include <QInputDialog>
#include <QKeyEvent>
#include <QLineEdit>
#include <QMenu>
#include <QMouseEvent>
#include <QPainter>
#include <QWheelEvent>

PageView::PageView(editor::EditorController& controller, QWidget* parent)
    : QWidget(parent), m_controller(controller) {
    setMinimumSize(640, 480);
    setAutoFillBackground(true);
    setFocusPolicy(Qt::StrongFocus);

    connect(&m_controller, &editor::EditorController::documentChanged, this, [this]() { update(); });
    connect(&m_controller, &editor::EditorController::pageChanged, this, [this]() { update(); });
}

void PageView::setZoom(float zoom) {
    const float clamped = std::clamp(zoom, 0.25f, 4.0f);
    if (qFuzzyCompare(m_zoom, clamped)) {
        return;
    }
    m_zoom = clamped;
    emit zoomChanged(m_zoom);
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
    const QRect target((width() - targetSize.width()) / 2, (height() - targetSize.height()) / 2, targetSize.width(), targetSize.height());
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
            painter.setPen(QPen(QColor(220, 48, 48), 2));
            painter.setBrush(QColor(255, 220, 220, 96));
            painter.drawRect(annotation->rect);
            painter.drawText(annotation->rect.adjusted(4, 4, -4, -4), annotation->text);
            break;
        }
        case overlay::OverlayObject::Kind::TextEdit: {
            const auto* textEdit = static_cast<const overlay::TextEditObject*>(objPtr.get());
            painter.setPen(QPen(QColor(40, 70, 220), 2));
            painter.setBrush(QColor(220, 230, 255, 90));
            painter.drawRoundedRect(textEdit->rect, 4, 4);
            auto font = painter.font();
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

    if (m_activeOverlay >= 0) {
        const auto* active = m_controller.currentPageOverlayAt(m_activeOverlay);
        const QRectF selectedRect = overlayRect(active);
        if (active && !selectedRect.isEmpty()) {
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
    const QPointF pagePos = widgetToPage(event->position());
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

    Q_UNUSED(pagePos)

    if (event->angleDelta().y() > 0) {
        m_controller.previousPage();
    } else {
        m_controller.nextPage();
    }
    event->accept();
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

    if (selected->kind() == overlay::OverlayObject::Kind::Annotation
        || selected->kind() == overlay::OverlayObject::Kind::Shape) {
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
            openOverlayContextMenu(event->globalPos(), hit);
            update();
            event->accept();
            return;
        }
    }

    if (event->button() != Qt::LeftButton) {
        QWidget::mousePressEvent(event);
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
    if (!overlay || overlay->kind() != overlay::OverlayObject::Kind::TextEdit) {
        QWidget::mouseDoubleClickEvent(event);
        return;
    }

    auto* textObj = static_cast<overlay::TextEditObject*>(overlay);
    bool ok = false;
    const QString updated = QInputDialog::getText(this, tr("Edit Text"), tr("Text:"), QLineEdit::Normal, textObj->text, &ok);
    if (ok) {
        m_activeOverlay = hit;
        m_controller.setTextOverlayText(hit, updated);
    }
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
