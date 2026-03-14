#pragma once

#include <QImage>
#include <QString>

namespace ocr {

class OCRProcessor {
public:
    bool isAvailable(QString* diagnostic = nullptr) const;
    QString recognize(const QImage& image,
                      const QString& lang = QStringLiteral("eng"),
                      QString* error = nullptr) const;

private:
    QString resolveExecutable() const;
};

} // namespace ocr
