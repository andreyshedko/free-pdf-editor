#include "ocr/OCRProcessor.h"

#include <QDir>
#include <QFile>
#include <QFileInfo>
#include <QProcess>
#include <QStandardPaths>
#include <QTemporaryDir>

namespace ocr {

QString OCRProcessor::resolveExecutable() const {
    QString exe = QStandardPaths::findExecutable(QStringLiteral("tesseract"));
    if (!exe.isEmpty()) {
        return exe;
    }

    exe = QStandardPaths::findExecutable(QStringLiteral("tesseract.exe"));
    if (!exe.isEmpty()) {
        return exe;
    }

    const QString fromEnv = qEnvironmentVariable("TESSERACT_EXE");
    if (!fromEnv.isEmpty() && QFileInfo::exists(fromEnv)) {
        return QDir::toNativeSeparators(fromEnv);
    }

    const QString fromOcrEnv = qEnvironmentVariable("OCR_TESSERACT_EXE");
    if (!fromOcrEnv.isEmpty() && QFileInfo::exists(fromOcrEnv)) {
        return QDir::toNativeSeparators(fromOcrEnv);
    }

#ifdef Q_OS_WIN
    const QStringList candidates = {
        QStringLiteral("C:/Program Files/Tesseract-OCR/tesseract.exe"),
        QStringLiteral("C:/Program Files (x86)/Tesseract-OCR/tesseract.exe")
    };
    for (const QString& candidate : candidates) {
        if (QFileInfo::exists(candidate)) {
            return QDir::toNativeSeparators(candidate);
        }
    }
#endif

    return {};
}

bool OCRProcessor::isAvailable(QString* diagnostic) const {
    const QString exe = resolveExecutable();
    if (exe.isEmpty()) {
        if (diagnostic) {
            *diagnostic = QStringLiteral(
                "Tesseract OCR engine not found. Install Tesseract or set TESSERACT_EXE to tesseract.exe path.");
        }
        return false;
    }

    if (diagnostic) {
        *diagnostic = QStringLiteral("Using Tesseract: %1").arg(QDir::toNativeSeparators(exe));
    }
    return true;
}

QString OCRProcessor::recognize(const QImage& image, const QString& lang, QString* error) const {
    const QString exe = resolveExecutable();
    if (exe.isEmpty()) {
        if (error) {
            *error = QStringLiteral(
                "Tesseract OCR engine not found. Install Tesseract or set TESSERACT_EXE to tesseract.exe path.");
        }
        return {};
    }

    QTemporaryDir dir;
    if (!dir.isValid()) {
        if (error) {
            *error = QStringLiteral("OCR temp dir error");
        }
        return {};
    }

    const QString input = dir.path() + QStringLiteral("/input.png");
    const QString outputBase = dir.path() + QStringLiteral("/output");
    if (!image.save(input)) {
        if (error) {
            *error = QStringLiteral("OCR input save failed");
        }
        return {};
    }

    QProcess p;
    p.start(exe, {input, outputBase, QStringLiteral("-l"), lang});
    if (!p.waitForFinished(120000)) {
        if (error) {
            *error = QStringLiteral("OCR process timeout");
        }
        return {};
    }

    if (p.exitStatus() != QProcess::NormalExit || p.exitCode() != 0) {
        if (error) {
            const QString stderrText = QString::fromUtf8(p.readAllStandardError()).trimmed();
            if (!stderrText.isEmpty()) {
                *error = stderrText;
            } else {
                *error = QStringLiteral("OCR process failed with exit code %1").arg(p.exitCode());
            }
        }
        return {};
    }

    QFile txt(outputBase + QStringLiteral(".txt"));
    if (!txt.open(QIODevice::ReadOnly)) {
        if (error) {
            *error = QStringLiteral("OCR output read failed");
        }
        return {};
    }

    const QString extracted = QString::fromUtf8(txt.readAll());
    if (error) {
        error->clear();
    }
    return extracted;
}

} // namespace ocr
