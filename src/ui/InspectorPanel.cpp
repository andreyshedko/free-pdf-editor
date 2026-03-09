#include "ui/InspectorPanel.h"

InspectorPanel::InspectorPanel(QWidget* parent)
    : QListWidget(parent) {
    setMinimumWidth(280);
}
