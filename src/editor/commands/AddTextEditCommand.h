#pragma once

#include "editor/Command.h"

#include <QString>

namespace document { class Document; }

namespace editor::commands {

class AddTextEditCommand final : public editor::Command {
public:
    AddTextEditCommand(document::Document& document, int pageIndex, QString text);

    void execute() override;
    void undo() override;

private:
    document::Document& m_document;
    int m_pageIndex;
    QString m_text;
    int m_insertedIndex {-1};
};

} // namespace editor::commands
