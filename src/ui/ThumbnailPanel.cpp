#include "ui/ThumbnailPanel.h"

ThumbnailPanel::ThumbnailPanel(QWidget* parent)
    : QListWidget(parent) {
    setMinimumWidth(240);
}
