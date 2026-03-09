#include "editor/UndoStack.h"

namespace editor {

void UndoStack::execute(std::unique_ptr<Command> command) {
    if (!command) {
        return;
    }
    command->execute();
    m_undo.push_back(std::move(command));
    m_redo.clear();
}

bool UndoStack::canUndo() const {
    return !m_undo.empty();
}

bool UndoStack::canRedo() const {
    return !m_redo.empty();
}

void UndoStack::undo() {
    if (m_undo.empty()) {
        return;
    }
    auto command = std::move(m_undo.back());
    m_undo.pop_back();
    command->undo();
    m_redo.push_back(std::move(command));
}

void UndoStack::redo() {
    if (m_redo.empty()) {
        return;
    }
    auto command = std::move(m_redo.back());
    m_redo.pop_back();
    command->execute();
    m_undo.push_back(std::move(command));
}

void UndoStack::clear() {
    m_undo.clear();
    m_redo.clear();
}

} // namespace editor
