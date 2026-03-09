#pragma once

#include <QImage>
#include <QString>

namespace ocr {

class OCRProcessor {
public:
    bool isAvailable() const;
    QString recognize(const QImage& image, const QString& lang = QStringLiteral("eng")) const;
};

} // namespace ocr
