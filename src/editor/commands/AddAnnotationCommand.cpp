#include "editor/commands/AddAnnotationCommand.h"

#include "document/Document.h"
#include "overlay/OverlayObject.h"

namespace editor::commands {

AddAnnotationCommand::AddAnnotationCommand(document::Document& document, int pageIndex, QString text, QRectF rect)
    : m_document(document), m_pageIndex(pageIndex), m_text(std::move(text)), m_rect(std::move(rect)) {}

void AddAnnotationCommand::execute() {
    if (!m_document.isOpen()) {
        return;
    }

    auto annotation = std::make_unique<overlay::AnnotationObject>();
    annotation->text = m_text;
    annotation->rect = m_rect;

    m_insertedIndex = m_document.page(m_pageIndex).addOverlay(std::move(annotation));
}

void AddAnnotationCommand::undo() {
    if (!m_document.isOpen() || m_insertedIndex < 0) {
        return;
    }
    m_document.page(m_pageIndex).removeOverlayAt(m_insertedIndex);
}

} // namespace editor::commands
