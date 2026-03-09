#include "editor/commands/MoveObjectCommand.h"

#include "document/Document.h"
#include "overlay/OverlayObject.h"

namespace editor::commands {

MoveObjectCommand::MoveObjectCommand(document::Document& document, int pageIndex, int overlayIndex, const QPointF& delta)
    : m_document(document), m_pageIndex(pageIndex), m_overlayIndex(overlayIndex), m_delta(delta) {}

void MoveObjectCommand::execute() {
    apply(m_delta);
}

void MoveObjectCommand::undo() {
    apply(-m_delta);
}

void MoveObjectCommand::apply(const QPointF& delta) {
    if (!m_document.isOpen()) {
        return;
    }

    auto* obj = m_document.page(m_pageIndex).overlayAt(m_overlayIndex);
    if (!obj) {
        return;
    }

    switch (obj->kind()) {
    case overlay::OverlayObject::Kind::Annotation:
        static_cast<overlay::AnnotationObject*>(obj)->rect.translate(delta);
        break;
    case overlay::OverlayObject::Kind::TextEdit:
        static_cast<overlay::TextEditObject*>(obj)->rect.translate(delta);
        break;
    case overlay::OverlayObject::Kind::ImageEdit:
        static_cast<overlay::ImageObject*>(obj)->rect.translate(delta);
        break;
    case overlay::OverlayObject::Kind::Shape:
        static_cast<overlay::ShapeObject*>(obj)->rect.translate(delta);
        break;
    }
}

} // namespace editor::commands
