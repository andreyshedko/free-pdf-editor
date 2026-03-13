#pragma once

#include "editor/EditorController.h"

#include <QMainWindow>

class Toolbar;
class ThumbnailPanel;
class InspectorPanel;
class PageView;
class QLabel;
class QMenu;

class MainWindow : public QMainWindow {
    Q_OBJECT

public:
    explicit MainWindow(QWidget* parent = nullptr);

private:
    void setModernTheme();
    void setupUi();
    void setupActions();
    void rebuildRecentMenu();
    void refreshPanels();
    void updateEditableStateIndicator();

    editor::EditorController m_controller;
    Toolbar* m_toolbar {nullptr};
    ThumbnailPanel* m_thumbnails {nullptr};
    InspectorPanel* m_properties {nullptr};
    PageView* m_pageView {nullptr};
    QLabel* m_statusLabel {nullptr};
    QLabel* m_editableStateLabel {nullptr};
    QMenu* m_openRecentMenu {nullptr};
};
