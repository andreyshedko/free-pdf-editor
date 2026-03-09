#include "ui/PageView.h"

#include "overlay/OverlayObject.h"

#include <algorithm>
#include <QPainter>
#include <QWheelEvent>

PageView::PageView(editor::EditorController& controller, QWidget* parent)
    : QWidget(parent), m_controller(controller) {
    setMinimumSize(640, 480);
    setAutoFillBackground(true);

    connect(&m_controller, &editor::EditorController::documentChanged, this, [this]() { update(); });
    connect(&m_controller, &editor::EditorController::pageChanged, this, [this]() { update(); });
}

void PageView::setZoom(float zoom) {
    m_zoom = std::clamp(zoom, 0.25f, 4.0f);
    update();
}

void PageView::paintEvent(QPaintEvent*) {
    QPainter painter(this);
    painter.fillRect(rect(), QColor(235, 238, 243));

    const QImage pageImage = m_controller.renderCurrentPage(m_zoom);
    if (pageImage.isNull()) {
        painter.setPen(QColor(80, 80, 80));
        painter.drawText(rect(), Qt::AlignCenter, QStringLiteral("Open PDF to start"));
        return;
    }

    const QSize fitted = pageImage.size().scaled(size(), Qt::KeepAspectRatio);
    const QRect target((width() - fitted.width()) / 2, (height() - fitted.height()) / 2, fitted.width(), fitted.height());
    painter.drawImage(target, pageImage);

    const auto* pageModel = m_controller.currentPageModel();
    if (!pageModel) {
        return;
    }

    const qreal sx = static_cast<qreal>(target.width()) / static_cast<qreal>(pageImage.width());
    const qreal sy = static_cast<qreal>(target.height()) / static_cast<qreal>(pageImage.height());

    painter.save();
    painter.translate(target.topLeft());
    painter.scale(sx, sy);

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

    painter.restore();
}

void PageView::wheelEvent(QWheelEvent* event) {
    if (event->modifiers().testFlag(Qt::ControlModifier)) {
        const float factor = event->angleDelta().y() > 0 ? 1.1f : 0.9f;
        setZoom(m_zoom * factor);
        event->accept();
        return;
    }

    if (event->angleDelta().y() > 0) {
        m_controller.previousPage();
    } else {
        m_controller.nextPage();
    }
    event->accept();
}
