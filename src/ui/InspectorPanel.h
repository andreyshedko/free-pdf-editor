#pragma once

#include <QListWidget>

class InspectorPanel : public QListWidget {
    Q_OBJECT

public:
    explicit InspectorPanel(QWidget* parent = nullptr);
};
