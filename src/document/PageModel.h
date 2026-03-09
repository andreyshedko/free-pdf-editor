#pragma once

#include "overlay/OverlayObject.h"

#include <vector>

namespace document {

class PageModel {
public:
    int pageNumber {0};
    std::vector<std::unique_ptr<overlay::OverlayObject>> overlayObjects;

    PageModel() = default;
    PageModel(const PageModel& other);
    PageModel& operator=(const PageModel& other);
    PageModel(PageModel&&) noexcept = default;
    PageModel& operator=(PageModel&&) noexcept = default;

    int addOverlay(std::unique_ptr<overlay::OverlayObject> overlay);
    bool removeOverlayAt(int index);
    overlay::OverlayObject* overlayAt(int index);
    const overlay::OverlayObject* overlayAt(int index) const;
};

} // namespace document
