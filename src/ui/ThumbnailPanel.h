#pragma once

#include <QListWidget>

class ThumbnailPanel : public QListWidget {
    Q_OBJECT

public:
    explicit ThumbnailPanel(QWidget* parent = nullptr);
};
