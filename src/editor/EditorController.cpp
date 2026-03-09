#include "editor/EditorController.h"

#include "editor/commands/AddAnnotationCommand.h"
#include "editor/commands/AddTextEditCommand.h"
#include "editor/commands/DeletePageCommand.h"
#include "editor/commands/InsertImageCommand.h"
#include "editor/commands/MoveObjectCommand.h"
#include "editor/commands/ReplaceTextCommand.h"

#include <algorithm>
#include <QImageReader>
#include <QSettings>

namespace editor {

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
    pushRecent(path);
    emit statusChanged(QStringLiteral("Saved: %1").arg(path));
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

} // namespace editor
