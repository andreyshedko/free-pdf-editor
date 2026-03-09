#pragma once

#include "editor/Command.h"

#include <QImage>

namespace document { class Document; }

namespace editor::commands {

class InsertImageCommand final : public editor::Command {
public:
    InsertImageCommand(document::Document& document, int pageIndex, const QImage& image);

    void execute() override;
    void undo() override;

private:
    document::Document& m_document;
    int m_pageIndex;
    QImage m_image;
    int m_insertedIndex {-1};
};

} // namespace editor::commands
