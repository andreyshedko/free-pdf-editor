#pragma once

#include "editor/EditorController.h"

#include <QWidget>

class QMouseEvent;
class QKeyEvent;

class PageView : public QWidget {
    Q_OBJECT

public:
    explicit PageView(editor::EditorController& controller, QWidget* parent = nullptr);

    void setZoom(float zoom);
    void setActiveOverlay(int overlayIndex);
    [[nodiscard]] float zoom() const { return m_zoom; }

signals:
    void zoomChanged(float zoom);

protected:
    void paintEvent(QPaintEvent* event) override;
    void wheelEvent(QWheelEvent* event) override;
    void mousePressEvent(QMouseEvent* event) override;
    void mouseMoveEvent(QMouseEvent* event) override;
    void mouseReleaseEvent(QMouseEvent* event) override;
    void mouseDoubleClickEvent(QMouseEvent* event) override;
    void keyPressEvent(QKeyEvent* event) override;

private:
    enum class DragMode {
        None,
        Move,
        Resize,
        Pan
    };

    [[nodiscard]] QRectF overlayRect(const overlay::OverlayObject* overlay) const;
    [[nodiscard]] QPointF widgetToPage(const QPointF& pos) const;
    [[nodiscard]] bool isPointOnResizeHandle(const QPointF& pagePos, const QRectF& rect) const;
    [[nodiscard]] int overlayAtPagePoint(const QPointF& pagePos) const;
    [[nodiscard]] QPointF clampPanForImageSize(const QSize& imageSize, const QPointF& pan) const;
    [[nodiscard]] qreal wheelScrollStep(const QWheelEvent* event) const;
    void openOverlayContextMenu(const QPoint& globalPos, int overlayIndex);

    editor::EditorController& m_controller;
    float m_zoom {1.0f};
    QRect m_lastTarget;
    QSize m_lastPageImageSize;
    int m_activeOverlay {-1};
    DragMode m_dragMode {DragMode::None};
    QPointF m_dragStartPage;
    QRectF m_dragStartRect;
    QPoint m_lastMousePos;
    QPointF m_pan;
};
