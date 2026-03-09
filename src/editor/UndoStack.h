#pragma once

#include "editor/Command.h"

#include <memory>
#include <vector>

namespace editor {

class UndoStack {
public:
    void execute(std::unique_ptr<Command> command);
    bool canUndo() const;
    bool canRedo() const;
    void undo();
    void redo();
    void clear();

private:
    std::vector<std::unique_ptr<Command>> m_undo;
    std::vector<std::unique_ptr<Command>> m_redo;
};

} // namespace editor
