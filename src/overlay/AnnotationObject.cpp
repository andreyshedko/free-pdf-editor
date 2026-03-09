#include "overlay/OverlayObject.h"

namespace overlay {

std::unique_ptr<OverlayObject> AnnotationObject::clone() const {
    auto out = std::make_unique<AnnotationObject>();
    out->rect = rect;
    out->text = text;
    return out;
}

std::unique_ptr<OverlayObject> TextEditObject::clone() const {
    auto out = std::make_unique<TextEditObject>();
    out->rect = rect;
    out->text = text;
    out->fontSize = fontSize;
    return out;
}

std::unique_ptr<OverlayObject> ImageObject::clone() const {
    auto out = std::make_unique<ImageObject>();
    out->rect = rect;
    out->image = image;
    return out;
}

std::unique_ptr<OverlayObject> ShapeObject::clone() const {
    auto out = std::make_unique<ShapeObject>();
    out->rect = rect;
    return out;
}

} // namespace overlay
