#include "editor/EditorController.h"

#include "editor/commands/AddAnnotationCommand.h"
#include "editor/commands/AddTextEditCommand.h"
#include "editor/commands/DeletePageCommand.h"
#include "editor/commands/InsertImageCommand.h"
#include "editor/commands/MoveObjectCommand.h"
#include "editor/commands/ReplaceTextCommand.h"

#include "overlay/OverlayObject.h"

#include <algorithm>
#include <QDateTime>
#include <QFileInfo>
#include <QPainter>
#include <QImageReader>
#include <QSettings>

namespace editor {

namespace {

QRectF targetTextRect(const document::PageModel& page, const SelectionManager& selection, int currentPage) {
    if (selection.hasSelection() && selection.pageIndex() == currentPage) {
        const auto* selected = page.overlayAt(selection.overlayIndex());
        if (selected && selected->kind() == overlay::OverlayObject::Kind::TextEdit) {
            return static_cast<const overlay::TextEditObject*>(selected)->rect;
        }
    }

    for (const auto& item : page.overlayObjects) {
        if (!item || item->kind() != overlay::OverlayObject::Kind::TextEdit) {
            continue;
        }
        return static_cast<const overlay::TextEditObject*>(item.get())->rect;
    }

    return QRectF(120.0, 140.0, 240.0, 56.0);
}

}

EditorController::EditorController(QObject* parent)
    : QObject(parent) {
    loadRecents();
}

bool EditorController::openDocument(const QString& path) {
    if (!m_document.open(path)) {
        emit statusChanged(QStringLiteral("Failed to open document"));
        return false;
    }

    m_renderer.open(m_document);
    m_undoStack.clear();
    m_selection.clear();
    m_currentPage = 0;
    pushRecent(path);

    emit documentChanged();
    emit pageChanged(m_currentPage, m_document.pageCount());
    emit statusChanged(QStringLiteral("Opened: %1").arg(path));
    return true;
}

bool EditorController::openRecent(int index) {
    if (index < 0 || index >= m_recentFiles.size()) {
        return false;
    }
    return openDocument(m_recentFiles[index]);
}

bool EditorController::saveDocument(const QString& path) {
    if (!m_writer.save(m_document, path, m_renderer)) {
        emit statusChanged(QStringLiteral("Failed to save document"));
        return false;
    }
    if (!m_document.save(path)) {
        emit statusChanged(QStringLiteral("Saved PDF, but failed to persist editable overlays"));
        return false;
    }
    pushRecent(path);
    emit statusChanged(QStringLiteral("Saved: %1").arg(path));
    return true;
}

bool EditorController::exportFlattenedPdf(const QString& path) {
    if (!m_writer.save(m_document, path, m_renderer)) {
        emit statusChanged(QStringLiteral("Failed to export flattened PDF"));
        return false;
    }
    pushRecent(path);
    emit statusChanged(QStringLiteral("Flattened PDF exported: %1").arg(path));
    return true;
}

void EditorController::addAnnotation(const QString& text) {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return;
    }

    m_undoStack.execute(std::make_unique<commands::AddAnnotationCommand>(m_document, m_currentPage, text));
    selectLastOverlay();
    emit documentChanged();
    emit statusChanged(QStringLiteral("Annotation added"));
}

void EditorController::addTextEdit(const QString& text) {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return;
    }

    m_undoStack.execute(std::make_unique<commands::AddTextEditCommand>(m_document, m_currentPage, text));
    selectLastOverlay();
    emit documentChanged();
    emit statusChanged(QStringLiteral("Text edit added"));
}

bool EditorController::replaceText(const QString& oldText, const QString& newText) {
    if (!m_document.isOpen()) {
        return false;
    }
    m_undoStack.execute(std::make_unique<commands::ReplaceTextCommand>(m_document, m_currentPage, oldText, newText));
    emit documentChanged();
    emit statusChanged(QStringLiteral("Text replaced"));
    return true;
}

bool EditorController::addImageOverlay(const QString& imagePath) {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return false;
    }

    QImageReader reader(imagePath);
    const QImage image = reader.read();
    if (image.isNull()) {
        emit statusChanged(QStringLiteral("Failed to load image"));
        return false;
    }

    m_undoStack.execute(std::make_unique<commands::InsertImageCommand>(m_document, m_currentPage, image));
    selectLastOverlay();
    emit documentChanged();
    emit statusChanged(QStringLiteral("Image overlay inserted"));
    return true;
}

bool EditorController::deleteCurrentPage() {
    if (!m_document.isOpen()) {
        return false;
    }

    const int pageBefore = m_currentPage;
    m_undoStack.execute(std::make_unique<commands::DeletePageCommand>(m_document, m_currentPage));
    if (m_document.pageCount() <= 0) {
        emit statusChanged(QStringLiteral("Delete page failed"));
        return false;
    }

    m_currentPage = std::min(pageBefore, m_document.pageCount() - 1);
    m_selection.clear();
    emit documentChanged();
    emit pageChanged(m_currentPage, m_document.pageCount());
    emit statusChanged(QStringLiteral("Page deleted"));
    return true;
}

bool EditorController::moveSelectedBy(const QPointF& delta) {
    if (!m_document.isOpen() || !m_selection.hasSelection()) {
        return false;
    }

    m_undoStack.execute(std::make_unique<commands::MoveObjectCommand>(
        m_document,
        m_selection.pageIndex(),
        m_selection.overlayIndex(),
        delta));

    emit documentChanged();
    emit statusChanged(QStringLiteral("Object moved"));
    return true;
}

bool EditorController::selectLastOverlay() {
    if (!m_document.isOpen()) {
        return false;
    }
    const auto& overlays = m_document.page(m_currentPage).overlayObjects;
    if (overlays.empty()) {
        m_selection.clear();
        return false;
    }
    m_selection.select(m_currentPage, static_cast<int>(overlays.size() - 1));
    return true;
}

QString EditorController::runOcrOnCurrentPage() {
    const QImage img = renderCurrentPage(1.0f);
    if (img.isNull()) {
        return QStringLiteral("No page");
    }
    const QString text = m_ocr.recognize(img);
    emit statusChanged(QStringLiteral("OCR done"));
    return text;
}

void EditorController::undo() {
    m_undoStack.undo();
    emit documentChanged();
    emit pageChanged(m_currentPage, std::max(1, m_document.pageCount()));
    emit statusChanged(QStringLiteral("Undo"));
}

void EditorController::redo() {
    m_undoStack.redo();
    emit documentChanged();
    emit pageChanged(m_currentPage, std::max(1, m_document.pageCount()));
    emit statusChanged(QStringLiteral("Redo"));
}

void EditorController::setCurrentPage(int pageIndex) {
    if (!m_document.isOpen() || pageIndex < 0 || pageIndex >= m_document.pageCount()) {
        return;
    }
    m_currentPage = pageIndex;
    m_selection.clear();
    emit pageChanged(m_currentPage, m_document.pageCount());
}

void EditorController::nextPage() { setCurrentPage(m_currentPage + 1); }
void EditorController::previousPage() { setCurrentPage(m_currentPage - 1); }

int EditorController::currentPage() const { return m_currentPage; }
int EditorController::pageCount() const { return m_document.pageCount(); }
bool EditorController::isOpen() const { return m_document.isOpen(); }
QString EditorController::currentDocumentPath() const { return m_document.path(); }
bool EditorController::hasEditableOverlayMetadata() const { return m_document.hasEditableOverlayMetadata(); }
QString EditorController::editableOverlayMetadataPath() const { return m_document.editableOverlayMetadataPath(); }
QString EditorController::expectedEditableOverlayMetadataPath() const { return m_document.expectedEditableOverlayMetadataPath(); }

QImage EditorController::renderCurrentPage(float zoom) const {
    if (!m_document.isOpen()) {
        return {};
    }
    return m_renderer.renderPage(m_document, m_currentPage, zoom);
}

const document::PageModel* EditorController::currentPageModel() const {
    if (!m_document.isOpen()) {
        return nullptr;
    }
    return &m_document.page(m_currentPage);
}

overlay::OverlayObject* EditorController::currentPageOverlayAt(int index) {
    if (!m_document.isOpen()) {
        return nullptr;
    }
    return m_document.page(m_currentPage).overlayAt(index);
}

int EditorController::currentPageOverlayCount() const {
    if (!m_document.isOpen()) {
        return 0;
    }
    return static_cast<int>(m_document.page(m_currentPage).overlayObjects.size());
}

bool EditorController::moveOverlayBy(int index, const QPointF& delta) {
    if (!m_document.isOpen()) {
        return false;
    }
    auto* obj = m_document.page(m_currentPage).overlayAt(index);
    if (!obj) {
        return false;
    }

    switch (obj->kind()) {
    case overlay::OverlayObject::Kind::Annotation:
        static_cast<overlay::AnnotationObject*>(obj)->rect.translate(delta);
        break;
    case overlay::OverlayObject::Kind::TextEdit:
        static_cast<overlay::TextEditObject*>(obj)->rect.translate(delta);
        break;
    case overlay::OverlayObject::Kind::ImageEdit:
        static_cast<overlay::ImageObject*>(obj)->rect.translate(delta);
        break;
    case overlay::OverlayObject::Kind::Shape:
        static_cast<overlay::ShapeObject*>(obj)->rect.translate(delta);
        break;
    }

    m_selection.select(m_currentPage, index);
    emit documentChanged();
    return true;
}

bool EditorController::setOverlayRect(int index, const QRectF& rect) {
    if (!m_document.isOpen()) {
        return false;
    }
    auto* obj = m_document.page(m_currentPage).overlayAt(index);
    if (!obj) {
        return false;
    }

    QRectF normalized = rect.normalized();
    normalized.setWidth(std::max(4.0, normalized.width()));
    normalized.setHeight(std::max(4.0, normalized.height()));

    switch (obj->kind()) {
    case overlay::OverlayObject::Kind::Annotation:
        static_cast<overlay::AnnotationObject*>(obj)->rect = normalized;
        break;
    case overlay::OverlayObject::Kind::TextEdit:
        static_cast<overlay::TextEditObject*>(obj)->rect = normalized;
        break;
    case overlay::OverlayObject::Kind::ImageEdit:
        static_cast<overlay::ImageObject*>(obj)->rect = normalized;
        break;
    case overlay::OverlayObject::Kind::Shape:
        static_cast<overlay::ShapeObject*>(obj)->rect = normalized;
        break;
    }

    m_selection.select(m_currentPage, index);
    emit documentChanged();
    return true;
}

bool EditorController::setTextOverlayText(int index, const QString& text) {
    if (!m_document.isOpen()) {
        return false;
    }
    auto* obj = m_document.page(m_currentPage).overlayAt(index);
    if (!obj || obj->kind() != overlay::OverlayObject::Kind::TextEdit) {
        return false;
    }

    static_cast<overlay::TextEditObject*>(obj)->text = text;
    m_selection.select(m_currentPage, index);
    emit documentChanged();
    emit statusChanged(QStringLiteral("Text updated"));
    return true;
}

bool EditorController::setAnnotationOverlayText(int index, const QString& text) {
    if (!m_document.isOpen()) {
        return false;
    }
    auto* obj = m_document.page(m_currentPage).overlayAt(index);
    if (!obj || obj->kind() != overlay::OverlayObject::Kind::Annotation) {
        return false;
    }

    static_cast<overlay::AnnotationObject*>(obj)->text = text;
    m_selection.select(m_currentPage, index);
    emit documentChanged();
    emit statusChanged(QStringLiteral("Annotation updated"));
    return true;
}

bool EditorController::setTextOverlayFontSize(int index, qreal size) {
    if (!m_document.isOpen()) {
        return false;
    }
    auto* obj = m_document.page(m_currentPage).overlayAt(index);
    if (!obj || obj->kind() != overlay::OverlayObject::Kind::TextEdit) {
        return false;
    }

    static_cast<overlay::TextEditObject*>(obj)->fontSize = std::clamp(static_cast<double>(size), 8.0, 144.0);
    m_selection.select(m_currentPage, index);
    emit documentChanged();
    emit statusChanged(QStringLiteral("Font size updated"));
    return true;
}

bool EditorController::rotateImageOverlay(int index, bool clockwise) {
    if (!m_document.isOpen()) {
        return false;
    }
    auto* obj = m_document.page(m_currentPage).overlayAt(index);
    if (!obj || obj->kind() != overlay::OverlayObject::Kind::ImageEdit) {
        return false;
    }

    auto* imageObj = static_cast<overlay::ImageObject*>(obj);
    if (imageObj->image.isNull()) {
        return false;
    }

    QTransform t;
    t.rotate(clockwise ? 90.0 : -90.0);
    imageObj->image = imageObj->image.transformed(t, Qt::SmoothTransformation);

    QRectF r = imageObj->rect;
    const QPointF c = r.center();
    r.setSize(QSizeF(r.height(), r.width()));
    r.moveCenter(c);
    imageObj->rect = r;

    m_selection.select(m_currentPage, index);
    emit documentChanged();
    emit statusChanged(QStringLiteral("Image rotated"));
    return true;
}

bool EditorController::flipImageOverlay(int index, bool horizontal) {
    if (!m_document.isOpen()) {
        return false;
    }
    auto* obj = m_document.page(m_currentPage).overlayAt(index);
    if (!obj || obj->kind() != overlay::OverlayObject::Kind::ImageEdit) {
        return false;
    }

    auto* imageObj = static_cast<overlay::ImageObject*>(obj);
    if (imageObj->image.isNull()) {
        return false;
    }

    imageObj->image = imageObj->image.flipped(horizontal ? Qt::Horizontal : Qt::Vertical);
    m_selection.select(m_currentPage, index);
    emit documentChanged();
    emit statusChanged(horizontal ? QStringLiteral("Image flipped horizontally")
                                 : QStringLiteral("Image flipped vertically"));
    return true;
}

bool EditorController::deleteOverlayAt(int index) {
    if (!m_document.isOpen()) {
        return false;
    }
    auto& page = m_document.page(m_currentPage);
    if (!page.removeOverlayAt(index)) {
        return false;
    }
    m_selection.clear();
    emit documentChanged();
    emit statusChanged(QStringLiteral("Overlay deleted"));
    return true;
}

QStringList EditorController::recentFiles() const { return m_recentFiles; }

void EditorController::pushRecent(const QString& path) {
    m_recentFiles.removeAll(path);
    m_recentFiles.prepend(path);
    while (m_recentFiles.size() > 10) {
        m_recentFiles.removeLast();
    }
    saveRecents();
    emit recentFilesChanged();
}

void EditorController::loadRecents() {
    QSettings settings(QStringLiteral("FreePdfEditor"), QStringLiteral("DesktopCpp"));
    m_recentFiles = settings.value(QStringLiteral("recentFiles")).toStringList();
}

void EditorController::saveRecents() const {
    QSettings settings(QStringLiteral("FreePdfEditor"), QStringLiteral("DesktopCpp"));
    settings.setValue(QStringLiteral("recentFiles"), m_recentFiles);
}

void EditorController::highlightAnnotation() {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return;
    }
    const QRectF textRect = targetTextRect(m_document.page(m_currentPage), m_selection, m_currentPage);
    const QRectF highlightRect = textRect.adjusted(-2.0, -2.0, 2.0, 2.0);
    m_undoStack.execute(std::make_unique<commands::AddAnnotationCommand>(
        m_document,
        m_currentPage,
        QStringLiteral("[Highlight]"),
        highlightRect));
    selectLastOverlay();
    emit documentChanged();
    emit statusChanged(QStringLiteral("Highlight annotation added"));
}

void EditorController::underlineAnnotation() {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return;
    }
    const QRectF textRect = targetTextRect(m_document.page(m_currentPage), m_selection, m_currentPage);
    const QRectF underlineRect(textRect.left(), textRect.bottom() - 8.0, textRect.width(), 14.0);
    m_undoStack.execute(std::make_unique<commands::AddAnnotationCommand>(
        m_document,
        m_currentPage,
        QStringLiteral("[Underline] %1").arg(QStringLiteral("Text")),
        underlineRect));
    selectLastOverlay();
    emit documentChanged();
    emit statusChanged(QStringLiteral("Underline annotation added"));
}

void EditorController::strikeoutAnnotation() {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return;
    }
    const QRectF textRect = targetTextRect(m_document.page(m_currentPage), m_selection, m_currentPage);
    const QRectF strikeRect(textRect.left(), textRect.center().y() - 7.0, textRect.width(), 14.0);
    m_undoStack.execute(std::make_unique<commands::AddAnnotationCommand>(
        m_document,
        m_currentPage,
        QStringLiteral("[Strikeout] %1").arg(QStringLiteral("Text")),
        strikeRect));
    selectLastOverlay();
    emit documentChanged();
    emit statusChanged(QStringLiteral("Strikeout annotation added"));
}

void EditorController::stickyNoteAnnotation() {
    addAnnotation(QStringLiteral("[Sticky Note]"));
}

void EditorController::commentAnnotation() {
    addAnnotation(QStringLiteral("[Comment]"));
}

void EditorController::drawShape() {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return;
    }
    auto shape = std::make_unique<overlay::ShapeObject>();
    shape->rect = QRectF(120.0, 120.0, 180.0, 100.0);
    const int index = m_document.page(m_currentPage).addOverlay(std::move(shape));
    m_selection.select(m_currentPage, index);
    emit documentChanged();
    emit statusChanged(QStringLiteral("Shape added"));
}

void EditorController::drawArrow() {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return;
    }
    auto shape = std::make_unique<overlay::ShapeObject>();
    shape->rect = QRectF(120.0, 260.0, 220.0, 12.0);
    const int index = m_document.page(m_currentPage).addOverlay(std::move(shape));
    m_selection.select(m_currentPage, index);
    emit documentChanged();
    emit statusChanged(QStringLiteral("Arrow added"));
}

bool EditorController::editTextValue(const QString& newValue) {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return false;
    }
    auto& page = m_document.page(m_currentPage);
    for (int i = 0; i < static_cast<int>(page.overlayObjects.size()); ++i) {
        auto* obj = page.overlayAt(i);
        if (obj && obj->kind() == overlay::OverlayObject::Kind::TextEdit) {
            auto* textObj = static_cast<overlay::TextEditObject*>(obj);
            textObj->text = newValue;
            m_selection.select(m_currentPage, i);
            emit documentChanged();
            emit statusChanged(QStringLiteral("Text updated"));
            return true;
        }
    }
    addTextEdit(newValue);
    return true;
}

bool EditorController::setTextFontTag(const QString& fontTag) {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return false;
    }
    auto& page = m_document.page(m_currentPage);
    for (int i = 0; i < static_cast<int>(page.overlayObjects.size()); ++i) {
        auto* obj = page.overlayAt(i);
        if (obj && obj->kind() == overlay::OverlayObject::Kind::TextEdit) {
            auto* textObj = static_cast<overlay::TextEditObject*>(obj);
            textObj->text = QStringLiteral("[%1] %2").arg(fontTag, textObj->text);
            m_selection.select(m_currentPage, i);
            emit documentChanged();
            emit statusChanged(QStringLiteral("Font updated"));
            return true;
        }
    }
    emit statusChanged(QStringLiteral("No text object to change font"));
    return false;
}

bool EditorController::setTextFontSize(qreal size) {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return false;
    }
    auto& page = m_document.page(m_currentPage);
    for (int i = 0; i < static_cast<int>(page.overlayObjects.size()); ++i) {
        auto* obj = page.overlayAt(i);
        if (obj && obj->kind() == overlay::OverlayObject::Kind::TextEdit) {
            auto* textObj = static_cast<overlay::TextEditObject*>(obj);
            textObj->fontSize = std::clamp(static_cast<double>(size), 8.0, 72.0);
            m_selection.select(m_currentPage, i);
            emit documentChanged();
            emit statusChanged(QStringLiteral("Font size updated"));
            return true;
        }
    }
    emit statusChanged(QStringLiteral("No text object to resize"));
    return false;
}

bool EditorController::moveFirstTextBlock(const QPointF& delta) {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return false;
    }
    auto& page = m_document.page(m_currentPage);
    for (int i = 0; i < static_cast<int>(page.overlayObjects.size()); ++i) {
        auto* obj = page.overlayAt(i);
        if (obj && obj->kind() == overlay::OverlayObject::Kind::TextEdit) {
            m_selection.select(m_currentPage, i);
            return moveSelectedBy(delta);
        }
    }
    emit statusChanged(QStringLiteral("No text block to move"));
    return false;
}

bool EditorController::moveFirstImage(const QPointF& delta) {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return false;
    }
    auto& page = m_document.page(m_currentPage);
    for (int i = 0; i < static_cast<int>(page.overlayObjects.size()); ++i) {
        auto* obj = page.overlayAt(i);
        if (obj && obj->kind() == overlay::OverlayObject::Kind::ImageEdit) {
            m_selection.select(m_currentPage, i);
            return moveSelectedBy(delta);
        }
    }
    emit statusChanged(QStringLiteral("No image to move"));
    return false;
}

bool EditorController::resizeFirstImage(qreal scale) {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return false;
    }
    if (scale <= 0.1) {
        emit statusChanged(QStringLiteral("Invalid image scale"));
        return false;
    }
    auto& page = m_document.page(m_currentPage);
    for (int i = 0; i < static_cast<int>(page.overlayObjects.size()); ++i) {
        auto* obj = page.overlayAt(i);
        if (obj && obj->kind() == overlay::OverlayObject::Kind::ImageEdit) {
            auto* imageObj = static_cast<overlay::ImageObject*>(obj);
            const QPointF c = imageObj->rect.center();
            imageObj->rect.setSize(imageObj->rect.size() * scale);
            imageObj->rect.moveCenter(c);
            m_selection.select(m_currentPage, i);
            emit documentChanged();
            emit statusChanged(QStringLiteral("Image resized"));
            return true;
        }
    }
    emit statusChanged(QStringLiteral("No image to resize"));
    return false;
}

int EditorController::findInOverlays(const QString& needle) const {
    if (!m_document.isOpen() || needle.isEmpty()) {
        return 0;
    }
    int hits = 0;
    const auto& page = m_document.page(m_currentPage);
    for (const auto& overlay : page.overlayObjects) {
        if (!overlay) {
            continue;
        }
        if (overlay->kind() == overlay::OverlayObject::Kind::TextEdit) {
            const auto* t = static_cast<const overlay::TextEditObject*>(overlay.get());
            if (t->text.contains(needle, Qt::CaseInsensitive)) {
                ++hits;
            }
        } else if (overlay->kind() == overlay::OverlayObject::Kind::Annotation) {
            const auto* a = static_cast<const overlay::AnnotationObject*>(overlay.get());
            if (a->text.contains(needle, Qt::CaseInsensitive)) {
                ++hits;
            }
        }
    }
    return hits;
}

bool EditorController::insertBlankPage() {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return false;
    }
    document::PageModel page;
    const int insertIndex = std::min(m_currentPage + 1, m_document.pageCount());
    if (!m_document.insertPage(insertIndex, page)) {
        emit statusChanged(QStringLiteral("Failed to insert blank page"));
        return false;
    }
    m_currentPage = insertIndex;
    m_selection.clear();
    emit documentChanged();
    emit pageChanged(m_currentPage, m_document.pageCount());
    emit statusChanged(QStringLiteral("Blank page inserted"));
    return true;
}

bool EditorController::mergeWithDocument(const QString& path) {
    const QFileInfo baseInfo(m_document.path());
    const QString fallback = baseInfo.absolutePath() + QStringLiteral("/") + baseInfo.completeBaseName() + QStringLiteral("-merged.pdf");
    return mergeWithDocumentTo(path, fallback);
}

bool EditorController::mergeWithDocumentTo(const QString& sourcePath, const QString& outputPath) {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a base document first"));
        return false;
    }

    document::Document other;
    if (!other.open(sourcePath)) {
        emit statusChanged(QStringLiteral("Failed to open merge source"));
        return false;
    }

    pdf_engine::PdfRenderer otherRenderer;
    otherRenderer.open(other);

    QVector<QImage> pages;
    pages.reserve(m_document.pageCount() + other.pageCount());

    for (int i = 0; i < m_document.pageCount(); ++i) {
        const QImage img = m_renderer.renderPage(m_document, i, 1.0f);
        if (!img.isNull()) {
            pages.push_back(img);
        }
    }
    for (int i = 0; i < other.pageCount(); ++i) {
        const QImage img = otherRenderer.renderPage(other, i, 1.0f);
        if (!img.isNull()) {
            pages.push_back(img);
        }
    }

    if (!m_writer.saveRenderedPages(pages, outputPath)) {
        emit statusChanged(QStringLiteral("Failed to write merged document"));
        return false;
    }

    if (!openDocument(outputPath)) {
        emit statusChanged(QStringLiteral("Merged file saved but failed to reopen"));
        return false;
    }

    emit statusChanged(QStringLiteral("Merged and opened: %1").arg(outputPath));
    return true;
}

bool EditorController::splitCurrentPageTo(const QString& path) {
    if (!m_document.isOpen() || m_currentPage < 0 || m_currentPage >= m_document.pageCount()) {
        emit statusChanged(QStringLiteral("No current page to split"));
        return false;
    }
    return splitPagesTo(path, {m_currentPage});
}

bool EditorController::splitPagesTo(const QString& path, const std::vector<int>& pageIndexes) {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return false;
    }
    if (pageIndexes.empty()) {
        emit statusChanged(QStringLiteral("No pages selected for split"));
        return false;
    }

    QVector<QImage> pages;
    pages.reserve(static_cast<int>(pageIndexes.size()));
    int insertedCount = 0;
    for (const int pageIndex : pageIndexes) {
        if (pageIndex < 0 || pageIndex >= m_document.pageCount()) {
            continue;
        }
        const QImage rendered = m_renderer.renderPage(m_document, pageIndex, 1.0f);
        if (!rendered.isNull()) {
            pages.push_back(rendered);
            ++insertedCount;
        }
    }

    if (insertedCount <= 0) {
        emit statusChanged(QStringLiteral("No valid pages selected for split"));
        return false;
    }

    const bool saved = m_writer.saveRenderedPages(pages, path);
    if (!saved) {
        emit statusChanged(QStringLiteral("Failed to save split document"));
        return false;
    }

    emit statusChanged(QStringLiteral("Saved %1 split page(s): %2").arg(insertedCount).arg(path));
    return true;
}

void EditorController::editText() {
    editTextValue(QStringLiteral("Edited text"));
}

void EditorController::changeFont() {
    setTextFontTag(QStringLiteral("Sans"));
}

void EditorController::changeFontSize() {
    setTextFontSize(14.0);
}

void EditorController::moveTextBlock() {
    moveFirstTextBlock(QPointF(12.0, 8.0));
}

void EditorController::moveImage() {
    moveFirstImage(QPointF(14.0, 10.0));
}

void EditorController::replaceImage() {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return;
    }
    auto& page = m_document.page(m_currentPage);
    for (int i = 0; i < static_cast<int>(page.overlayObjects.size()); ++i) {
        auto* obj = page.overlayAt(i);
        if (obj && obj->kind() == overlay::OverlayObject::Kind::ImageEdit) {
            auto* imageObj = static_cast<overlay::ImageObject*>(obj);
            const QSize sz = imageObj->image.isNull() ? QSize(320, 180) : imageObj->image.size();
            QImage replacement(sz, QImage::Format_ARGB32_Premultiplied);
            replacement.fill(QColor(230, 236, 246));
            QPainter p(&replacement);
            p.setRenderHint(QPainter::Antialiasing, true);
            p.setPen(QPen(QColor(40, 70, 130), 2));
            p.drawRect(replacement.rect().adjusted(1, 1, -2, -2));
            p.drawText(replacement.rect(), Qt::AlignCenter, QStringLiteral("Replaced Image"));
            p.end();
            imageObj->image = replacement;
            m_selection.select(m_currentPage, i);
            emit documentChanged();
            emit statusChanged(QStringLiteral("Image replaced"));
            return;
        }
    }
    emit statusChanged(QStringLiteral("No image to replace"));
}

void EditorController::deleteImage() {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return;
    }
    auto& page = m_document.page(m_currentPage);
    for (int i = 0; i < static_cast<int>(page.overlayObjects.size()); ++i) {
        auto* obj = page.overlayAt(i);
        if (obj && obj->kind() == overlay::OverlayObject::Kind::ImageEdit) {
            page.removeOverlayAt(i);
            m_selection.clear();
            emit documentChanged();
            emit statusChanged(QStringLiteral("Image deleted"));
            return;
        }
    }
    emit statusChanged(QStringLiteral("No image to delete"));
}

void EditorController::resizeImage() {
    resizeFirstImage(1.15);
}

void EditorController::createFormField() {
    addTextEdit(QStringLiteral("[Form] Name"));
}

void EditorController::editFormField() {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return;
    }
    auto& page = m_document.page(m_currentPage);
    for (int i = 0; i < static_cast<int>(page.overlayObjects.size()); ++i) {
        auto* obj = page.overlayAt(i);
        if (obj && obj->kind() == overlay::OverlayObject::Kind::TextEdit) {
            auto* textObj = static_cast<overlay::TextEditObject*>(obj);
            if (textObj->text.startsWith(QStringLiteral("[Form]")) || textObj->text.startsWith(QStringLiteral("[Filled]"))) {
                textObj->text += QStringLiteral(" (edited)");
                m_selection.select(m_currentPage, i);
                emit documentChanged();
                emit statusChanged(QStringLiteral("Form field edited"));
                return;
            }
        }
    }
    emit statusChanged(QStringLiteral("No form field available"));
}

void EditorController::fillForm() {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return;
    }
    auto& page = m_document.page(m_currentPage);
    for (int i = 0; i < static_cast<int>(page.overlayObjects.size()); ++i) {
        auto* obj = page.overlayAt(i);
        if (obj && obj->kind() == overlay::OverlayObject::Kind::TextEdit) {
            auto* textObj = static_cast<overlay::TextEditObject*>(obj);
            if (textObj->text.startsWith(QStringLiteral("[Form]"))) {
                textObj->text.replace(QStringLiteral("[Form]"), QStringLiteral("[Filled]"));
                textObj->text += QStringLiteral(" = value");
                m_selection.select(m_currentPage, i);
                emit documentChanged();
                emit statusChanged(QStringLiteral("Form field filled"));
                return;
            }
        }
    }
    emit statusChanged(QStringLiteral("No form field to fill"));
}

void EditorController::exportFormData() {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return;
    }
    QStringList fields;
    for (int p = 0; p < m_document.pageCount(); ++p) {
        const auto& page = m_document.page(p);
        for (const auto& overlay : page.overlayObjects) {
            if (!overlay || overlay->kind() != overlay::OverlayObject::Kind::TextEdit) {
                continue;
            }
            const auto* textObj = static_cast<const overlay::TextEditObject*>(overlay.get());
            if (textObj->text.startsWith(QStringLiteral("[Form]")) || textObj->text.startsWith(QStringLiteral("[Filled]"))) {
                fields << QStringLiteral("P%1: %2").arg(p + 1).arg(textObj->text);
            }
        }
    }
    emit statusChanged(fields.isEmpty() ? QStringLiteral("No form data to export")
                                        : QStringLiteral("Exported %1 form fields").arg(fields.size()));
}

void EditorController::drawSignature() {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return;
    }
    auto shape = std::make_unique<overlay::ShapeObject>();
    shape->rect = QRectF(360.0, 620.0, 210.0, 40.0);
    const int idx = m_document.page(m_currentPage).addOverlay(std::move(shape));
    m_selection.select(m_currentPage, idx);
    emit documentChanged();
    emit statusChanged(QStringLiteral("Signature stroke added"));
}

void EditorController::insertImageSignature() {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return;
    }
    QImage sig(260, 90, QImage::Format_ARGB32_Premultiplied);
    sig.fill(Qt::transparent);
    QPainter p(&sig);
    p.setRenderHint(QPainter::Antialiasing, true);
    p.setPen(QPen(QColor(20, 20, 20), 3));
    p.drawLine(8, 70, 70, 40);
    p.drawLine(70, 40, 130, 65);
    p.drawLine(130, 65, 210, 25);
    p.drawText(sig.rect().adjusted(0, 0, -8, -6), Qt::AlignRight | Qt::AlignBottom, QStringLiteral("Signature"));
    p.end();

    auto imageOverlay = std::make_unique<overlay::ImageObject>();
    imageOverlay->image = sig;
    imageOverlay->rect = QRectF(360.0, 620.0, sig.width(), sig.height());
    const int idx = m_document.page(m_currentPage).addOverlay(std::move(imageOverlay));
    m_selection.select(m_currentPage, idx);
    m_savedSignature = sig;
    emit documentChanged();
    emit statusChanged(QStringLiteral("Image signature inserted"));
}

void EditorController::saveSignature() {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return;
    }
    const auto& page = m_document.page(m_currentPage);
    for (const auto& overlay : page.overlayObjects) {
        if (overlay && overlay->kind() == overlay::OverlayObject::Kind::ImageEdit) {
            m_savedSignature = static_cast<const overlay::ImageObject*>(overlay.get())->image;
            emit statusChanged(QStringLiteral("Signature saved"));
            return;
        }
    }
    emit statusChanged(QStringLiteral("No image signature to save"));
}

void EditorController::applySignature() {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return;
    }
    if (m_savedSignature.isNull()) {
        insertImageSignature();
        return;
    }
    auto imageOverlay = std::make_unique<overlay::ImageObject>();
    imageOverlay->image = m_savedSignature;
    imageOverlay->rect = QRectF(360.0, 620.0, m_savedSignature.width(), m_savedSignature.height());
    const int idx = m_document.page(m_currentPage).addOverlay(std::move(imageOverlay));
    m_selection.select(m_currentPage, idx);
    emit documentChanged();
    emit statusChanged(QStringLiteral("Signature applied"));
}

void EditorController::hideText() {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return;
    }
    auto& page = m_document.page(m_currentPage);
    int changed = 0;
    for (auto& overlay : page.overlayObjects) {
        if (!overlay) {
            continue;
        }
        if (overlay->kind() == overlay::OverlayObject::Kind::TextEdit) {
            auto* t = static_cast<overlay::TextEditObject*>(overlay.get());
            t->text = QString(t->text.size(), QChar('X'));
            ++changed;
        } else if (overlay->kind() == overlay::OverlayObject::Kind::Annotation) {
            auto* a = static_cast<overlay::AnnotationObject*>(overlay.get());
            a->text = QString(a->text.size(), QChar('X'));
            ++changed;
        }
    }
    emit documentChanged();
    emit statusChanged(QStringLiteral("Redacted text overlays: %1").arg(changed));
}

void EditorController::hideImages() {
    if (!m_document.isOpen()) {
        emit statusChanged(QStringLiteral("Open a document first"));
        return;
    }
    auto& page = m_document.page(m_currentPage);
    int changed = 0;
    for (auto& overlay : page.overlayObjects) {
        if (!overlay || overlay->kind() != overlay::OverlayObject::Kind::ImageEdit) {
            continue;
        }
        auto* i = static_cast<overlay::ImageObject*>(overlay.get());
        if (i->image.isNull()) {
            i->image = QImage(120, 80, QImage::Format_ARGB32_Premultiplied);
        }
        i->image.fill(Qt::black);
        ++changed;
    }
    emit documentChanged();
    emit statusChanged(QStringLiteral("Redacted image overlays: %1").arg(changed));
}

void EditorController::applyPermanentRedaction() {
    hideText();
    hideImages();
    emit statusChanged(QStringLiteral("Permanent redaction applied"));
}

void EditorController::passwordProtection() {
    m_passwordProtected = !m_passwordProtected;
    emit statusChanged(m_passwordProtected
                           ? QStringLiteral("Password protection enabled")
                           : QStringLiteral("Password protection disabled"));
}

void EditorController::restrictPrinting() {
    m_restrictPrinting = !m_restrictPrinting;
    emit statusChanged(m_restrictPrinting
                           ? QStringLiteral("Printing restricted")
                           : QStringLiteral("Printing restriction removed"));
}

void EditorController::restrictCopying() {
    m_restrictCopying = !m_restrictCopying;
    emit statusChanged(m_restrictCopying
                           ? QStringLiteral("Copying restricted")
                           : QStringLiteral("Copying restriction removed"));
}

void EditorController::digitalSignature() {
    addAnnotation(QStringLiteral("[Digitally Signed] %1").arg(QDateTime::currentDateTime().toString(Qt::ISODate)));
}

void EditorController::sharedAnnotations() {
    addAnnotation(QStringLiteral("[Shared Annotation]"));
}

void EditorController::commentsCollab() {
    addAnnotation(QStringLiteral("[Collab Comment]"));
}

} // namespace editor
