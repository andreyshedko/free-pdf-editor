#include "ocr/OCRProcessor.h"

#include <QFile>
#include <QProcess>
#include <QStandardPaths>
#include <QTemporaryDir>

namespace ocr {

bool OCRProcessor::isAvailable() const {
    return !QStandardPaths::findExecutable(QStringLiteral("tesseract")).isEmpty();
}

QString OCRProcessor::recognize(const QImage& image, const QString& lang) const {
    const QString exe = QStandardPaths::findExecutable(QStringLiteral("tesseract"));
    if (exe.isEmpty()) {
        return QStringLiteral("Tesseract not found");
    }

    QTemporaryDir dir;
    if (!dir.isValid()) {
        return QStringLiteral("OCR temp dir error");
    }

    const QString input = dir.path() + QStringLiteral("/input.png");
    const QString outputBase = dir.path() + QStringLiteral("/output");
    if (!image.save(input)) {
        return QStringLiteral("OCR input save failed");
    }

    QProcess p;
    p.start(exe, {input, outputBase, QStringLiteral("-l"), lang});
    p.waitForFinished(120000);
    if (p.exitStatus() != QProcess::NormalExit || p.exitCode() != 0) {
        return QString::fromUtf8(p.readAllStandardError());
    }

    QFile txt(outputBase + QStringLiteral(".txt"));
    if (!txt.open(QIODevice::ReadOnly)) {
        return QStringLiteral("OCR output read failed");
    }
    return QString::fromUtf8(txt.readAll());
}

} // namespace ocr
