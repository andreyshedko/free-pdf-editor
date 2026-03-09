#include "document/PageModel.h"

namespace document {

PageModel::PageModel(const PageModel& other)
    : pageNumber(other.pageNumber) {
    overlayObjects.reserve(other.overlayObjects.size());
    for (const auto& o : other.overlayObjects) {
        overlayObjects.push_back(o ? o->clone() : nullptr);
    }
}

PageModel& PageModel::operator=(const PageModel& other) {
    if (this == &other) {
        return *this;
    }
    pageNumber = other.pageNumber;
    overlayObjects.clear();
    overlayObjects.reserve(other.overlayObjects.size());
    for (const auto& o : other.overlayObjects) {
        overlayObjects.push_back(o ? o->clone() : nullptr);
    }
    return *this;
}

int PageModel::addOverlay(std::unique_ptr<overlay::OverlayObject> overlay) {
    overlayObjects.push_back(std::move(overlay));
    return static_cast<int>(overlayObjects.size() - 1);
}

bool PageModel::removeOverlayAt(int index) {
    if (index < 0 || index >= static_cast<int>(overlayObjects.size())) {
        return false;
    }
    overlayObjects.erase(overlayObjects.begin() + index);
    return true;
}

overlay::OverlayObject* PageModel::overlayAt(int index) {
    if (index < 0 || index >= static_cast<int>(overlayObjects.size())) {
        return nullptr;
    }
    return overlayObjects[static_cast<size_t>(index)].get();
}

const overlay::OverlayObject* PageModel::overlayAt(int index) const {
    if (index < 0 || index >= static_cast<int>(overlayObjects.size())) {
        return nullptr;
    }
    return overlayObjects[static_cast<size_t>(index)].get();
}

} // namespace document
