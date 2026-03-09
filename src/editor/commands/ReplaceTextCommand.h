#pragma once

#include "editor/Command.h"

#include <QString>

namespace document { class Document; }

namespace editor::commands {

class ReplaceTextCommand final : public editor::Command {
public:
    ReplaceTextCommand(document::Document& document, int pageIndex, QString oldText, QString newText);

    void execute() override;
    void undo() override;

private:
    document::Document& m_document;
    int m_pageIndex;
    QString m_oldText;
    QString m_newText;
    int m_overlayIndex {-1};
    QString m_prevText;
};

} // namespace editor::commands
