#pragma once

#include "editor/EditorController.h"

#include <QWidget>

class PageView : public QWidget {
    Q_OBJECT

public:
    explicit PageView(editor::EditorController& controller, QWidget* parent = nullptr);

    void setZoom(float zoom);

protected:
    void paintEvent(QPaintEvent* event) override;
    void wheelEvent(QWheelEvent* event) override;

private:
    editor::EditorController& m_controller;
    float m_zoom {1.0f};
};
