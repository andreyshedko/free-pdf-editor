#pragma once

#include "editor/Command.h"

#include <memory>

namespace document {
class Document;
class PageModel;
}

namespace editor::commands {

class DeletePageCommand final : public editor::Command {
public:
    DeletePageCommand(document::Document& document, int pageIndex);

    void execute() override;
    void undo() override;

private:
    document::Document& m_document;
    int m_pageIndex {0};
    bool m_executed {false};
    std::shared_ptr<document::PageModel> m_deletedPage;
};

} // namespace editor::commands
