#include "editor/commands/DeletePageCommand.h"

#include "document/Document.h"
#include "document/PageModel.h"

namespace editor::commands {

DeletePageCommand::DeletePageCommand(document::Document& document, int pageIndex)
    : m_document(document), m_pageIndex(pageIndex) {}

void DeletePageCommand::execute() {
    document::PageModel deleted;
    if (!m_document.deletePage(m_pageIndex, &deleted)) {
        return;
    }
    m_deletedPage = std::make_shared<document::PageModel>(deleted);
    m_executed = true;
}

void DeletePageCommand::undo() {
    if (!m_executed || !m_deletedPage) {
        return;
    }
    m_document.insertPage(m_pageIndex, *m_deletedPage);
}

} // namespace editor::commands
