#include "document/Document.h"

#include "overlay/OverlayObject.h"

#include <QBuffer>
#include <QFile>
#include <QFileInfo>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QRegularExpression>

#include <algorithm>

namespace {

QString sidecarPathFor(const QString& pdfPath) {
    const QFileInfo info(pdfPath);
    return info.absolutePath() + QStringLiteral("/") + info.completeBaseName() + QStringLiteral(".fpe.json");
}

QString legacySidecarPathFor(const QString& pdfPath) {
    return pdfPath + QStringLiteral(".fpe.json");
}

int detectPageCountHeuristic(const QByteArray& bytes) {
    const QByteArray marker("/Type /Page");
    int count = 0;
    int from = 0;
    while (true) {
        const int idx = bytes.indexOf(marker, from);
        if (idx < 0) {
            break;
        }
        ++count;
        from = idx + marker.size();
    }
    return std::max(1, count);
}

QJsonObject rectToJson(const QRectF& rect) {
    return {
        {QStringLiteral("x"), rect.x()},
        {QStringLiteral("y"), rect.y()},
        {QStringLiteral("w"), rect.width()},
        {QStringLiteral("h"), rect.height()}
    };
}

QRectF rectFromJson(const QJsonObject& obj) {
    return QRectF(
        obj.value(QStringLiteral("x")).toDouble(),
        obj.value(QStringLiteral("y")).toDouble(),
        obj.value(QStringLiteral("w")).toDouble(),
        obj.value(QStringLiteral("h")).toDouble());
}

struct LegacyFontParse {
    QString fontFamily;
    QString text;
};

LegacyFontParse parseLegacyTextFontTag(const QString& rawText) {
    static const QRegularExpression pattern(QStringLiteral("^\\[([^\\]\\n]{1,80})\\]\\s+(.+)$"));
    const QRegularExpressionMatch match = pattern.match(rawText);
    if (!match.hasMatch()) {
        return {QString{}, rawText};
    }

    const QString tag = match.captured(1).trimmed();
    const QString text = match.captured(2);
    if (tag.isEmpty() || tag.contains('=') || tag.contains(';') || !tag.contains(QRegularExpression(QStringLiteral("[A-Za-z]")))) {
        return {QString{}, rawText};
    }
    return {tag, text};
}

QJsonObject overlayToJson(const overlay::OverlayObject& overlayObject) {
    QJsonObject obj;
    switch (overlayObject.kind()) {
    case overlay::OverlayObject::Kind::Annotation: {
        const auto& annotation = static_cast<const overlay::AnnotationObject&>(overlayObject);
        obj[QStringLiteral("kind")] = QStringLiteral("annotation");
        obj[QStringLiteral("rect")] = rectToJson(annotation.rect);
        obj[QStringLiteral("text")] = annotation.text;
        break;
    }
    case overlay::OverlayObject::Kind::TextEdit: {
        const auto& textEdit = static_cast<const overlay::TextEditObject&>(overlayObject);
        obj[QStringLiteral("kind")] = QStringLiteral("text");
        obj[QStringLiteral("rect")] = rectToJson(textEdit.rect);
        obj[QStringLiteral("text")] = textEdit.text;
        obj[QStringLiteral("fontFamily")] = textEdit.fontFamily;
        obj[QStringLiteral("fontSize")] = textEdit.fontSize;
        break;
    }
    case overlay::OverlayObject::Kind::ImageEdit: {
        const auto& imageEdit = static_cast<const overlay::ImageObject&>(overlayObject);
        obj[QStringLiteral("kind")] = QStringLiteral("image");
        obj[QStringLiteral("rect")] = rectToJson(imageEdit.rect);
        QByteArray pngBytes;
        QBuffer buffer(&pngBytes);
        buffer.open(QIODevice::WriteOnly);
        imageEdit.image.save(&buffer, "PNG");
        obj[QStringLiteral("imagePngBase64")] = QString::fromLatin1(pngBytes.toBase64());
        break;
    }
    case overlay::OverlayObject::Kind::Shape: {
        const auto& shape = static_cast<const overlay::ShapeObject&>(overlayObject);
        obj[QStringLiteral("kind")] = QStringLiteral("shape");
        obj[QStringLiteral("rect")] = rectToJson(shape.rect);
        break;
    }
    }
    return obj;
}

std::unique_ptr<overlay::OverlayObject> overlayFromJson(const QJsonObject& obj) {
    const QString kind = obj.value(QStringLiteral("kind")).toString();
    const QRectF rect = rectFromJson(obj.value(QStringLiteral("rect")).toObject());

    if (kind == QStringLiteral("annotation")) {
        auto out = std::make_unique<overlay::AnnotationObject>();
        out->rect = rect;
        out->text = obj.value(QStringLiteral("text")).toString();
        return out;
    }
    if (kind == QStringLiteral("text")) {
        auto out = std::make_unique<overlay::TextEditObject>();
        out->rect = rect;
        out->text = obj.value(QStringLiteral("text")).toString();
        out->fontFamily = obj.value(QStringLiteral("fontFamily")).toString();
        if (out->fontFamily.isEmpty()) {
            const LegacyFontParse parsed = parseLegacyTextFontTag(out->text);
            if (!parsed.fontFamily.isEmpty()) {
                out->fontFamily = parsed.fontFamily;
                out->text = parsed.text;
            }
        }
        out->fontSize = obj.value(QStringLiteral("fontSize")).toDouble(12.0);
        return out;
    }
    if (kind == QStringLiteral("image")) {
        auto out = std::make_unique<overlay::ImageObject>();
        out->rect = rect;
        const QByteArray pngBytes = QByteArray::fromBase64(obj.value(QStringLiteral("imagePngBase64")).toString().toLatin1());
        out->image.loadFromData(pngBytes, "PNG");
        return out;
    }
    if (kind == QStringLiteral("shape")) {
        auto out = std::make_unique<overlay::ShapeObject>();
        out->rect = rect;
        return out;
    }
    return nullptr;
}

bool writeOverlaySidecar(const QString& pdfPath, const std::vector<document::PageModel>& pages) {
    QJsonArray pagesJson;
    for (const auto& page : pages) {
        QJsonObject pageObj;
        pageObj[QStringLiteral("pageNumber")] = page.pageNumber;
        QJsonArray overlays;
        for (const auto& overlay : page.overlayObjects) {
            if (!overlay) {
                continue;
            }
            overlays.append(overlayToJson(*overlay));
        }
        pageObj[QStringLiteral("overlays")] = overlays;
        pagesJson.append(pageObj);
    }

    QFile sidecar(sidecarPathFor(pdfPath));
    if (!sidecar.open(QIODevice::WriteOnly | QIODevice::Truncate)) {
        return false;
    }
    const QJsonObject root{{QStringLiteral("pages"), pagesJson}};
    sidecar.write(QJsonDocument(root).toJson(QJsonDocument::Indented));
    return true;
}

bool loadOverlaySidecar(const QString& pdfPath, std::vector<document::PageModel>& pages, QString* loadedPath = nullptr) {
    QFile sidecar(sidecarPathFor(pdfPath));
    if (!sidecar.exists()) {
        sidecar.setFileName(legacySidecarPathFor(pdfPath));
    }
    if (!sidecar.exists() || !sidecar.open(QIODevice::ReadOnly)) {
        return false;
    }

    if (loadedPath) {
        *loadedPath = sidecar.fileName();
    }

    const QJsonDocument doc = QJsonDocument::fromJson(sidecar.readAll());
    const QJsonArray pagesJson = doc.object().value(QStringLiteral("pages")).toArray();
    for (int i = 0; i < pagesJson.size() && i < static_cast<int>(pages.size()); ++i) {
        const QJsonObject pageObj = pagesJson.at(i).toObject();
        const QJsonArray overlays = pageObj.value(QStringLiteral("overlays")).toArray();
        pages[static_cast<size_t>(i)].overlayObjects.clear();
        for (const auto& overlayValue : overlays) {
            auto overlay = overlayFromJson(overlayValue.toObject());
            if (overlay) {
                pages[static_cast<size_t>(i)].overlayObjects.push_back(std::move(overlay));
            }
        }
    }
    return true;
}

} // namespace

namespace document {

bool Document::open(const QString& path) {
    QFileInfo info(path);
    if (!info.exists() || !info.isFile()) {
        return false;
    }

    QFile file(info.absoluteFilePath());
    if (!file.open(QIODevice::ReadOnly)) {
        return false;
    }

    m_hasEditableOverlayMetadata = false;
    m_editableOverlayMetadataPath.clear();
    m_sourceBytes = file.readAll();
    m_path = info.absoluteFilePath();
    m_metadata.title = info.completeBaseName();

    const int pages = detectPageCountHeuristic(m_sourceBytes);
    m_pages.clear();
    m_pages.reserve(static_cast<size_t>(pages));

    for (int i = 0; i < pages; ++i) {
        PageModel page;
        page.pageNumber = i;
        m_pages.push_back(std::move(page));
    }

    m_hasEditableOverlayMetadata = loadOverlaySidecar(m_path, m_pages, &m_editableOverlayMetadataPath);

    return true;
}

bool Document::save(const QString& path) {
    if (!isOpen()) {
        return false;
    }
    QFile file(path);
    if (file.open(QIODevice::ReadOnly)) {
        m_sourceBytes = file.readAll();
        file.close();
    }
    m_path = QFileInfo(path).absoluteFilePath();
    m_editableOverlayMetadataPath = sidecarPathFor(m_path);
    m_hasEditableOverlayMetadata = writeOverlaySidecar(m_path, m_pages);
    if (!m_hasEditableOverlayMetadata) {
        m_editableOverlayMetadataPath.clear();
    }
    return m_hasEditableOverlayMetadata;
}

bool Document::isOpen() const {
    return !m_pages.empty();
}

const QString& Document::path() const {
    return m_path;
}

const QByteArray& Document::sourceBytes() const {
    return m_sourceBytes;
}

const DocumentMetadata& Document::metadata() const {
    return m_metadata;
}

bool Document::hasEditableOverlayMetadata() const {
    return m_hasEditableOverlayMetadata;
}

const QString& Document::editableOverlayMetadataPath() const {
    return m_editableOverlayMetadataPath;
}

QString Document::expectedEditableOverlayMetadataPath() const {
    if (m_path.isEmpty()) {
        return {};
    }
    return sidecarPathFor(m_path);
}

int Document::pageCount() const {
    return static_cast<int>(m_pages.size());
}

PageModel& Document::page(int index) {
    return m_pages.at(static_cast<size_t>(index));
}

const PageModel& Document::page(int index) const {
    return m_pages.at(static_cast<size_t>(index));
}

bool Document::deletePage(int index, PageModel* removedPage) {
    if (!isOpen() || index < 0 || index >= pageCount() || pageCount() <= 1) {
        return false;
    }

    const auto it = m_pages.begin() + index;
    if (removedPage) {
        *removedPage = *it;
    }
    m_pages.erase(it);
    renumberPages();
    return true;
}

bool Document::insertPage(int index, const PageModel& pageModel) {
    if (index < 0 || index > pageCount()) {
        return false;
    }
    m_pages.insert(m_pages.begin() + index, pageModel);
    renumberPages();
    return true;
}

void Document::reset() {
    m_path.clear();
    m_sourceBytes.clear();
    m_pages.clear();
    m_metadata = {};
    m_hasEditableOverlayMetadata = false;
    m_editableOverlayMetadataPath.clear();
}

void Document::renumberPages() {
    for (int i = 0; i < static_cast<int>(m_pages.size()); ++i) {
        m_pages[static_cast<size_t>(i)].pageNumber = i;
    }
}

} // namespace document
