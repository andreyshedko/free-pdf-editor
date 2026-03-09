#include "ui/Toolbar.h"

Toolbar::Toolbar(QWidget* parent)
    : QToolBar(parent) {
    setObjectName(QStringLiteral("mainToolbar"));
}
