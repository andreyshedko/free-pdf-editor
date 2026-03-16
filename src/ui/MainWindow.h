#pragma once

#include "editor/EditorController.h"

#include <QMainWindow>
#include <QString>
#include <utility>
#include <vector>

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
    void runFind();
    void applyFindMatch(int index);
    void clearFindState(bool clearQuery);
    void updateFindStatusLabel();
    void rebuildRecentMenu();
    void refreshPanels();
    void updateEditableStateIndicator();

    editor::EditorController m_controller;
    Toolbar* m_toolbar {nullptr};
    ThumbnailPanel* m_thumbnails {nullptr};
    InspectorPanel* m_properties {nullptr};
    PageView* m_pageView {nullptr};
    QLabel* m_statusLabel {nullptr};
    QLabel* m_findStatusLabel {nullptr};
    QLabel* m_editableStateLabel {nullptr};
    QMenu* m_openRecentMenu {nullptr};
    QString m_lastFindQuery;
    std::vector<std::pair<int, int>> m_findMatches;
    int m_findMatchIndex {-1};
    bool m_findWrapEnabled {true};
};
