#include "editor/commands/InsertImageCommand.h"

#include "document/Document.h"
#include "overlay/OverlayObject.h"

#include <QImage>

namespace editor::commands {

InsertImageCommand::InsertImageCommand(document::Document& document, int pageIndex, const QImage& image)
    : m_document(document), m_pageIndex(pageIndex), m_image(image) {}

void InsertImageCommand::execute() {
    if (!m_document.isOpen() || m_image.isNull()) {
        return;
    }

    auto imageOverlay = std::make_unique<overlay::ImageObject>();
    imageOverlay->image = m_image;
    imageOverlay->rect = QRectF(160.0, 320.0, 220.0, 180.0);
    m_insertedIndex = m_document.page(m_pageIndex).addOverlay(std::move(imageOverlay));
}

void InsertImageCommand::undo() {
    if (!m_document.isOpen() || m_insertedIndex < 0) {
        return;
    }
    m_document.page(m_pageIndex).removeOverlayAt(m_insertedIndex);
}

} // namespace editor::commands
