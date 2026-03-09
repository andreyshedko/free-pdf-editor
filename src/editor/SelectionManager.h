#pragma once

#include <optional>

namespace editor {

class SelectionManager {
public:
    void select(int pageIndex, int overlayIndex);
    void clear();

    [[nodiscard]] bool hasSelection() const;
    [[nodiscard]] int pageIndex() const;
    [[nodiscard]] int overlayIndex() const;

private:
    std::optional<int> m_pageIndex;
    std::optional<int> m_overlayIndex;
};

} // namespace editor
