#pragma once

#include "editor/Command.h"

#include <QRectF>
#include <QString>

namespace document { class Document; }

namespace editor::commands {

class AddAnnotationCommand final : public editor::Command {
public:
    AddAnnotationCommand(document::Document& document, int pageIndex, QString text, QRectF rect = QRectF(120.0, 140.0, 240.0, 56.0));

    void execute() override;
    void undo() override;

private:
    document::Document& m_document;
    int m_pageIndex;
    QString m_text;
    QRectF m_rect;
    int m_insertedIndex {-1};
};

} // namespace editor::commands
