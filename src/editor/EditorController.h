#pragma once

#include "document/Document.h"
#include "editor/SelectionManager.h"
#include "editor/UndoStack.h"
#include "ocr/OCRProcessor.h"
#include "pdf_engine/PdfRenderer.h"
#include "pdf_engine/PdfWriter.h"

#include <QObject>
#include <QImage>
#include <QStringList>
#include <vector>

namespace editor {

class EditorController : public QObject {
    Q_OBJECT

public:
    explicit EditorController(QObject* parent = nullptr);

    bool openDocument(const QString& path);
    bool openRecent(int index);
    bool saveDocument(const QString& path);
    bool exportFlattenedPdf(const QString& path);

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
    [[nodiscard]] bool isOpen() const;
    [[nodiscard]] QString currentDocumentPath() const;
    [[nodiscard]] bool hasEditableOverlayMetadata() const;
    [[nodiscard]] QString editableOverlayMetadataPath() const;
    [[nodiscard]] QString expectedEditableOverlayMetadataPath() const;
    [[nodiscard]] QImage renderCurrentPage(float zoom) const;
    [[nodiscard]] const document::PageModel* currentPageModel() const;
    [[nodiscard]] overlay::OverlayObject* currentPageOverlayAt(int index);
    [[nodiscard]] int currentPageOverlayCount() const;

    bool moveOverlayBy(int index, const QPointF& delta);
    bool setOverlayRect(int index, const QRectF& rect);
    bool setTextOverlayText(int index, const QString& text);
    bool setAnnotationOverlayText(int index, const QString& text);
    bool setTextOverlayFontSize(int index, qreal size);
    bool rotateImageOverlay(int index, bool clockwise = true);
    bool flipImageOverlay(int index, bool horizontal = true);
    bool deleteOverlayAt(int index);

    [[nodiscard]] QStringList recentFiles() const;

        // Advanced feature actions
        void highlightAnnotation();
        void underlineAnnotation();
        void strikeoutAnnotation();
        void stickyNoteAnnotation();
        void commentAnnotation();
        void drawArrow();
        void editText();
        void changeFont();
        void changeFontSize();
        void moveTextBlock();
        void moveImage();
        void replaceImage();
        void deleteImage();
        void resizeImage();
        void createFormField();
        void editFormField();
        void fillForm();
        void exportFormData();
        void drawSignature();
        void insertImageSignature();
        void saveSignature();
        void applySignature();
        void hideText();
        void hideImages();
        void applyPermanentRedaction();
        void passwordProtection();
        void restrictPrinting();
        void restrictCopying();
        void digitalSignature();
        void sharedAnnotations();
        void commentsCollab();

        bool insertBlankPage();
        int findInOverlays(const QString& needle) const;
        bool editTextValue(const QString& newValue);
        bool setTextFontTag(const QString& fontTag);
        bool setTextFontSize(qreal size);
        bool moveFirstTextBlock(const QPointF& delta);
        bool moveFirstImage(const QPointF& delta);
        bool resizeFirstImage(qreal scale);
        bool mergeWithDocument(const QString& path);
        bool mergeWithDocumentTo(const QString& sourcePath, const QString& outputPath);
        bool splitCurrentPageTo(const QString& path);
        bool splitPagesTo(const QString& path, const std::vector<int>& pageIndexes);

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
    QImage m_savedSignature;
    bool m_passwordProtected {false};
    bool m_restrictPrinting {false};
    bool m_restrictCopying {false};
};

} // namespace editor
