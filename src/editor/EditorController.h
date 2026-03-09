#pragma once

#include "document/Document.h"
#include "editor/SelectionManager.h"
#include "editor/UndoStack.h"
#include "ocr/OCRProcessor.h"
#include "pdf_engine/PdfRenderer.h"
#include "pdf_engine/PdfWriter.h"

#include <QObject>
#include <QStringList>

namespace editor {

class EditorController : public QObject {
    Q_OBJECT

public:
    explicit EditorController(QObject* parent = nullptr);

    bool openDocument(const QString& path);
    bool openRecent(int index);
    bool saveDocument(const QString& path);

    void addAnnotation(const QString& text);
    void addTextEdit(const QString& text);
    bool replaceText(const QString& oldText, const QString& newText);
    bool addImageOverlay(const QString& imagePath);
    bool deleteCurrentPage();
    bool moveSelectedBy(const QPointF& delta);
    bool selectLastOverlay();
    QString runOcrOnCurrentPage();

    void undo();
    void redo();

    void setCurrentPage(int pageIndex);
    void nextPage();
    void previousPage();

    [[nodiscard]] int currentPage() const;
    [[nodiscard]] int pageCount() const;
    [[nodiscard]] QImage renderCurrentPage(float zoom) const;
    [[nodiscard]] const document::PageModel* currentPageModel() const;

    [[nodiscard]] QStringList recentFiles() const;

signals:
    void documentChanged();
    void pageChanged(int pageIndex, int pageCount);
    void recentFilesChanged();
    void statusChanged(const QString& status);

private:
    void pushRecent(const QString& path);
    void loadRecents();
    void saveRecents() const;

    document::Document m_document;
    pdf_engine::PdfRenderer m_renderer;
    pdf_engine::PdfWriter m_writer;
    ocr::OCRProcessor m_ocr;
    UndoStack m_undoStack;
    SelectionManager m_selection;
    int m_currentPage {0};
    QStringList m_recentFiles;
};

} // namespace editor
