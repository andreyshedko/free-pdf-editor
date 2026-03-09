#include "editor/commands/ReplaceTextCommand.h"

#include "document/Document.h"
#include "overlay/OverlayObject.h"

namespace editor::commands {

ReplaceTextCommand::ReplaceTextCommand(document::Document& document, int pageIndex, QString oldText, QString newText)
    : m_document(document), m_pageIndex(pageIndex), m_oldText(std::move(oldText)), m_newText(std::move(newText)) {}

void ReplaceTextCommand::execute() {
    if (!m_document.isOpen() || m_pageIndex < 0 || m_pageIndex >= m_document.pageCount()) {
        return;
    }

    auto& page = m_document.page(m_pageIndex);
    for (int i = 0; i < static_cast<int>(page.overlayObjects.size()); ++i) {
        auto* obj = page.overlayAt(i);
        if (!obj || obj->kind() != overlay::OverlayObject::Kind::TextEdit) {
            continue;
        }
        auto* textObj = static_cast<overlay::TextEditObject*>(obj);
        if (textObj->text.contains(m_oldText)) {
            m_overlayIndex = i;
            m_prevText = textObj->text;
            textObj->text.replace(m_oldText, m_newText);
            return;
        }
    }
}

void ReplaceTextCommand::undo() {
    if (!m_document.isOpen() || m_overlayIndex < 0 || m_pageIndex < 0 || m_pageIndex >= m_document.pageCount()) {
        return;
    }
    auto* obj = m_document.page(m_pageIndex).overlayAt(m_overlayIndex);
    if (!obj || obj->kind() != overlay::OverlayObject::Kind::TextEdit) {
        return;
    }
    auto* textObj = static_cast<overlay::TextEditObject*>(obj);
    textObj->text = m_prevText;
}

} // namespace editor::commands
