#pragma once

#include <QImage>
#include <QRectF>
#include <QString>
#include <memory>

namespace overlay {

class OverlayObject {
public:
    enum class Kind {
        Annotation,
        TextEdit,
        ImageEdit,
        Shape
    };

    virtual ~OverlayObject() = default;
    virtual Kind kind() const = 0;
    virtual std::unique_ptr<OverlayObject> clone() const = 0;
};

class AnnotationObject final : public OverlayObject {
public:
    QRectF rect;
    QString text;

    Kind kind() const override { return Kind::Annotation; }
    std::unique_ptr<OverlayObject> clone() const override;
};

class TextEditObject final : public OverlayObject {
public:
    QRectF rect;
    QString text;
    QString fontFamily;
    qreal fontSize {12.0};

    Kind kind() const override { return Kind::TextEdit; }
    std::unique_ptr<OverlayObject> clone() const override;
};

class ImageObject final : public OverlayObject {
public:
    QRectF rect;
    QImage image;

    Kind kind() const override { return Kind::ImageEdit; }
    std::unique_ptr<OverlayObject> clone() const override;
};

class ShapeObject final : public OverlayObject {
public:
    QRectF rect;

    Kind kind() const override { return Kind::Shape; }
    std::unique_ptr<OverlayObject> clone() const override;
};

} // namespace overlay
