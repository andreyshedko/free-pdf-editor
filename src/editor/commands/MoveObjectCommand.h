#pragma once

#include "editor/Command.h"

#include <QPointF>

namespace document { class Document; }

namespace editor::commands {

class MoveObjectCommand final : public editor::Command {
public:
    MoveObjectCommand(document::Document& document, int pageIndex, int overlayIndex, const QPointF& delta);

    void execute() override;
    void undo() override;

private:
    void apply(const QPointF& delta);

    document::Document& m_document;
    int m_pageIndex;
    int m_overlayIndex;
    QPointF m_delta;
};

} // namespace editor::commands
