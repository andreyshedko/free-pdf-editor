#include "editor/SelectionManager.h"

namespace editor {

void SelectionManager::select(int pageIndex, int overlayIndex) {
    m_pageIndex = pageIndex;
    m_overlayIndex = overlayIndex;
}

void SelectionManager::clear() {
    m_pageIndex.reset();
    m_overlayIndex.reset();
}

bool SelectionManager::hasSelection() const {
    return m_pageIndex.has_value() && m_overlayIndex.has_value();
}

int SelectionManager::pageIndex() const {
    return m_pageIndex.value_or(-1);
}

int SelectionManager::overlayIndex() const {
    return m_overlayIndex.value_or(-1);
}

} // namespace editor
