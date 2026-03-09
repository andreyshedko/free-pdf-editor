#include "editor/commands/AddTextEditCommand.h"

#include "document/Document.h"
#include "overlay/OverlayObject.h"

namespace editor::commands {

AddTextEditCommand::AddTextEditCommand(document::Document& document, int pageIndex, QString text)
    : m_document(document), m_pageIndex(pageIndex), m_text(std::move(text)) {}

void AddTextEditCommand::execute() {
    if (!m_document.isOpen()) {
        return;
    }

    auto textEdit = std::make_unique<overlay::TextEditObject>();
    textEdit->text = m_text;
    textEdit->fontSize = 14.0;
    textEdit->rect = QRectF(140.0, 220.0, 280.0, 64.0);
    m_insertedIndex = m_document.page(m_pageIndex).addOverlay(std::move(textEdit));
}

void AddTextEditCommand::undo() {
    if (!m_document.isOpen() || m_insertedIndex < 0) {
        return;
    }
    m_document.page(m_pageIndex).removeOverlayAt(m_insertedIndex);
}

} // namespace editor::commands
