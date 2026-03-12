#include "ui/MainWindow.h"

#include "overlay/OverlayObject.h"
#include "ui/InspectorPanel.h"
#include "ui/PageView.h"
#include "ui/ThumbnailPanel.h"
#include "ui/Toolbar.h"

#include <algorithm>
#include <QAction>
#include <QApplication>
#include <QFile>
#include <QFileDialog>
#include <QHBoxLayout>
#include <QInputDialog>
#include <QLabel>
#include <QLineEdit>
#include <QMenu>
#include <QMenuBar>
#include <QMessageBox>
#include <QPalette>
#include <QStatusBar>
#include <QStyle>
#include <QWidget>

MainWindow::MainWindow(QWidget* parent)
    : QMainWindow(parent) {
    setModernTheme();
    setupUi();
    setupActions();
    refreshPanels();

    connect(&m_controller, &editor::EditorController::statusChanged, this, [this](const QString& status) {
        m_statusLabel->setText(status);
    });
    connect(&m_controller, &editor::EditorController::documentChanged, this, [this]() {
        refreshPanels();
    });
    connect(&m_controller, &editor::EditorController::pageChanged, this, [this](int page, int count) {
        m_statusLabel->setText(tr("Page %1 / %2").arg(page + 1).arg(count));
        refreshPanels();
    });
    connect(&m_controller, &editor::EditorController::recentFilesChanged, this, [this]() {
        rebuildRecentMenu();
    });
}

void MainWindow::setModernTheme() {
    qApp->setStyle("Fusion");
    QPalette darkPalette;
    darkPalette.setColor(QPalette::Window, QColor(48, 50, 56));
    darkPalette.setColor(QPalette::WindowText, QColor(235, 239, 245));
    darkPalette.setColor(QPalette::Base, QColor(30, 32, 37));
    darkPalette.setColor(QPalette::AlternateBase, QColor(42, 44, 50));
    darkPalette.setColor(QPalette::ToolTipBase, QColor(30, 32, 37));
    darkPalette.setColor(QPalette::ToolTipText, QColor(235, 239, 245));
    darkPalette.setColor(QPalette::Text, QColor(235, 239, 245));
    darkPalette.setColor(QPalette::Button, QColor(56, 58, 65));
    darkPalette.setColor(QPalette::ButtonText, QColor(235, 239, 245));
    darkPalette.setColor(QPalette::Link, QColor(73, 147, 235));
    darkPalette.setColor(QPalette::Highlight, QColor(73, 147, 235));
    darkPalette.setColor(QPalette::HighlightedText, QColor(12, 16, 24));
    qApp->setPalette(darkPalette);
}

void MainWindow::setupUi() {
    resize(1440, 900);
    setWindowTitle(tr("PDF Editor Pro"));

    auto* central = new QWidget(this);
    auto* layout = new QHBoxLayout(central);
    layout->setContentsMargins(0, 0, 0, 0);
    layout->setSpacing(1);

    m_thumbnails = new ThumbnailPanel(central);
    m_pageView = new PageView(m_controller, central);
    m_properties = new InspectorPanel(central);

    m_thumbnails->setFixedWidth(220);
    m_properties->setFixedWidth(260);

    layout->addWidget(m_thumbnails);
    layout->addWidget(m_pageView, 1);
    layout->addWidget(m_properties);
    setCentralWidget(central);

    m_toolbar = new Toolbar(this);
    addToolBar(m_toolbar);

    m_statusLabel = new QLabel(tr("Ready"), this);
    statusBar()->addPermanentWidget(m_statusLabel, 1);
}

void MainWindow::setupActions() {
    // File Menu
    auto* fileMenu = menuBar()->addMenu(tr("&File"));
    const QIcon openIcon = style()->standardIcon(QStyle::SP_DirOpenIcon);
    const QIcon saveIcon = style()->standardIcon(QStyle::SP_DialogSaveButton);

    auto* openAction = new QAction(openIcon, tr("&Open..."), this);
    openAction->setShortcut(QKeySequence::Open);
    fileMenu->addAction(openAction);

    m_openRecentMenu = fileMenu->addMenu(tr("Open &Recent"));

    auto* saveAction = new QAction(saveIcon, tr("&Save"), this);
    saveAction->setShortcut(QKeySequence::Save);
    fileMenu->addAction(saveAction);

    auto* saveAsAction = new QAction(tr("Save &As..."), this);
    saveAsAction->setShortcut(QKeySequence::SaveAs);
    fileMenu->addAction(saveAsAction);

    fileMenu->addSeparator();

    auto* mergeAction = new QAction(tr("&Merge PDFs..."), this);
    auto* splitAction = new QAction(tr("S&plit PDF..."), this);
    fileMenu->addAction(mergeAction);
    fileMenu->addAction(splitAction);

    auto* exportMenu = fileMenu->addMenu(tr("&Export As..."));
    auto* exportAsImagesAction = new QAction(tr("Images..."), this);
    auto* exportAsTextAction = new QAction(tr("Text..."), this);
    exportMenu->addAction(exportAsImagesAction);
    exportMenu->addAction(exportAsTextAction);

    fileMenu->addSeparator();

    auto* printAction = new QAction(tr("&Print..."), this);
    printAction->setShortcut(QKeySequence::Print);
    fileMenu->addAction(printAction);

    fileMenu->addSeparator();

    auto* exitAction = new QAction(tr("E&xit"), this);
    exitAction->setShortcut(QKeySequence::Quit);
    fileMenu->addAction(exitAction);

    // Edit Menu
    auto* editMenu = menuBar()->addMenu(tr("&Edit"));
    auto* undoAction = new QAction(style()->standardIcon(QStyle::SP_ArrowBack), tr("&Undo"), this);
    undoAction->setShortcut(QKeySequence::Undo);
    editMenu->addAction(undoAction);

    auto* redoAction = new QAction(style()->standardIcon(QStyle::SP_ArrowForward), tr("&Redo"), this);
    redoAction->setShortcut(QKeySequence::Redo);
    editMenu->addAction(redoAction);

    editMenu->addSeparator();

    auto* findAction = new QAction(tr("&Find..."), this);
    findAction->setShortcut(QKeySequence::Find);
    editMenu->addAction(findAction);

    // View Menu
    auto* viewMenu = menuBar()->addMenu(tr("&View"));
    auto* zoomInAction = new QAction(tr("Zoom &In"), this);
    zoomInAction->setShortcut(QKeySequence::ZoomIn);
    viewMenu->addAction(zoomInAction);

    auto* zoomOutAction = new QAction(tr("Zoom &Out"), this);
    zoomOutAction->setShortcut(QKeySequence::ZoomOut);
    viewMenu->addAction(zoomOutAction);

    viewMenu->addSeparator();

    auto* showThumbnailsAction = new QAction(tr("Page &Thumbnails"), this);
    showThumbnailsAction->setCheckable(true);
    showThumbnailsAction->setChecked(true);
    viewMenu->addAction(showThumbnailsAction);

    auto* showInspectorAction = new QAction(tr("&Inspector"), this);
    showInspectorAction->setCheckable(true);
    showInspectorAction->setChecked(true);
    viewMenu->addAction(showInspectorAction);

    // Page Menu
    auto* pageMenu = menuBar()->addMenu(tr("&Page"));
    auto* insertBlankPageAction = new QAction(tr("&Insert Blank Page"), this);
    pageMenu->addAction(insertBlankPageAction);

    auto* deletePageAction = new QAction(tr("&Delete Pages..."), this);
    deletePageAction->setShortcut(QKeySequence::Delete);
    pageMenu->addAction(deletePageAction);

    // Tools Menu
    auto* toolsMenu = menuBar()->addMenu(tr("&Tools"));

    auto* annotationsMenu = toolsMenu->addMenu(tr("&Annotations"));
    auto* highlightAction = new QAction(tr("Highlight"), this);
    auto* underlineAction = new QAction(tr("Underline"), this);
    auto* strikeoutAction = new QAction(tr("Strikeout"), this);
    auto* stickyNoteAction = new QAction(tr("Sticky Note"), this);
    auto* commentAction = new QAction(tr("Comment"), this);
    auto* addAnnotAction = new QAction(tr("Add Note"), this);
    auto* drawShapeAction = new QAction(tr("Draw Shape"), this);
    auto* arrowAction = new QAction(tr("Arrow"), this);
    annotationsMenu->addAction(highlightAction);
    annotationsMenu->addAction(underlineAction);
    annotationsMenu->addAction(strikeoutAction);
    annotationsMenu->addAction(stickyNoteAction);
    annotationsMenu->addAction(commentAction);
    annotationsMenu->addAction(addAnnotAction);
    annotationsMenu->addAction(drawShapeAction);
    annotationsMenu->addAction(arrowAction);

    auto* textImageMenu = toolsMenu->addMenu(tr("&Text && Images"));
    auto* addTextAction = new QAction(tr("Add Text Box"), this);
    auto* replaceTextAction = new QAction(tr("Replace Text"), this);
    auto* editTextAction = new QAction(tr("Edit Text"), this);
    auto* changeFontAction = new QAction(tr("Change Font"), this);
    auto* changeFontSizeAction = new QAction(tr("Change Font Size"), this);
    auto* moveTextBlockAction = new QAction(tr("Move Text Block"), this);
    auto* addImageAction = new QAction(tr("Insert Image"), this);
    auto* moveImageAction = new QAction(tr("Move Image"), this);
    auto* replaceImageAction = new QAction(tr("Replace Image"), this);
    auto* deleteImageAction = new QAction(tr("Delete Image"), this);
    auto* resizeImageAction = new QAction(tr("Resize Image"), this);
    textImageMenu->addAction(addTextAction);
    textImageMenu->addAction(replaceTextAction);
    textImageMenu->addAction(editTextAction);
    textImageMenu->addAction(changeFontAction);
    textImageMenu->addAction(changeFontSizeAction);
    textImageMenu->addAction(moveTextBlockAction);
    textImageMenu->addSeparator();
    textImageMenu->addAction(addImageAction);
    textImageMenu->addAction(moveImageAction);
    textImageMenu->addAction(replaceImageAction);
    textImageMenu->addAction(deleteImageAction);
    textImageMenu->addAction(resizeImageAction);

    auto* formsMenu = toolsMenu->addMenu(tr("&Forms"));
    auto* createFormFieldAction = new QAction(tr("Create Form Field"), this);
    auto* editFormFieldAction = new QAction(tr("Edit Form Field"), this);
    auto* fillFormAction = new QAction(tr("Fill Form"), this);
    auto* exportFormDataAction = new QAction(tr("Export Form Data"), this);
    formsMenu->addAction(createFormFieldAction);
    formsMenu->addAction(editFormFieldAction);
    formsMenu->addAction(fillFormAction);
    formsMenu->addAction(exportFormDataAction);

    auto* signaturesMenu = toolsMenu->addMenu(tr("&Signatures"));
    auto* drawSignatureAction = new QAction(tr("Draw Signature"), this);
    auto* insertImageSignatureAction = new QAction(tr("Insert Image Signature"), this);
    auto* saveSignatureAction = new QAction(tr("Save Signature"), this);
    auto* applySignatureAction = new QAction(tr("Apply Signature"), this);
    signaturesMenu->addAction(drawSignatureAction);
    signaturesMenu->addAction(insertImageSignatureAction);
    signaturesMenu->addAction(saveSignatureAction);
    signaturesMenu->addAction(applySignatureAction);

    auto* redactionMenu = toolsMenu->addMenu(tr("&Redaction"));
    auto* hideTextAction = new QAction(tr("Hide Text"), this);
    auto* hideImagesAction = new QAction(tr("Hide Images"), this);
    auto* applyPermanentRedactionAction = new QAction(tr("Apply Permanent Redaction"), this);
    redactionMenu->addAction(hideTextAction);
    redactionMenu->addAction(hideImagesAction);
    redactionMenu->addAction(applyPermanentRedactionAction);

    auto* securityMenu = toolsMenu->addMenu(tr("&Security"));
    auto* passwordProtectionAction = new QAction(tr("Password Protection"), this);
    auto* restrictPrintingAction = new QAction(tr("Restrict Printing"), this);
    auto* restrictCopyingAction = new QAction(tr("Restrict Copying"), this);
    auto* digitalSignatureAction = new QAction(tr("Digital Signatures"), this);
    securityMenu->addAction(passwordProtectionAction);
    securityMenu->addAction(restrictPrintingAction);
    securityMenu->addAction(restrictCopyingAction);
    securityMenu->addAction(digitalSignatureAction);

    auto* collaborationMenu = toolsMenu->addMenu(tr("&Collaboration"));
    auto* sharedAnnotationsAction = new QAction(tr("Shared Annotations"), this);
    auto* commentsCollabAction = new QAction(tr("Comments"), this);
    collaborationMenu->addAction(sharedAnnotationsAction);
    collaborationMenu->addAction(commentsCollabAction);

    toolsMenu->addSeparator();
    auto* ocrAction = new QAction(tr("Recognize Text (OCR)"), this);
    toolsMenu->addAction(ocrAction);

    // Help Menu
    auto* helpMenu = menuBar()->addMenu(tr("&Help"));
    auto* aboutAction = new QAction(tr("&About"), this);
    helpMenu->addAction(aboutAction);

    // Connect File actions
    connect(openAction, &QAction::triggered, this, [this]() {
        const QString path = QFileDialog::getOpenFileName(this, tr("Open PDF"), {}, tr("PDF Files (*.pdf)"));
        if (!path.isEmpty()) {
            m_controller.openDocument(path);
        }
    });

    connect(saveAction, &QAction::triggered, this, [this]() {
        if (!m_controller.isOpen()) {
            m_statusLabel->setText(tr("Open a document first"));
            return;
        }
        QString target = m_controller.recentFiles().isEmpty() ? QString() : m_controller.recentFiles().first();
        if (target.isEmpty()) {
            target = QFileDialog::getSaveFileName(this, tr("Save PDF"), {}, tr("PDF Files (*.pdf)"));
        }
        if (!target.isEmpty()) {
            m_controller.saveDocument(target);
        }
    });

    connect(saveAsAction, &QAction::triggered, this, [this]() {
        const QString path = QFileDialog::getSaveFileName(this, tr("Save PDF"), {}, tr("PDF Files (*.pdf)"));
        if (!path.isEmpty()) {
            m_controller.saveDocument(path);
        }
    });

    connect(exitAction, &QAction::triggered, this, &MainWindow::close);

    // Connect edit/view/page actions
    connect(undoAction, &QAction::triggered, &m_controller, &editor::EditorController::undo);
    connect(redoAction, &QAction::triggered, &m_controller, &editor::EditorController::redo);
    connect(deletePageAction, &QAction::triggered, this, [this]() { m_controller.deleteCurrentPage(); });
    connect(findAction, &QAction::triggered, this, [this]() {
        bool ok = false;
        const QString query = QInputDialog::getText(this, tr("Find"), tr("Find text in overlays:"), QLineEdit::Normal, {}, &ok);
        if (!ok || query.isEmpty()) {
            return;
        }
        const int hits = m_controller.findInOverlays(query);
        m_statusLabel->setText(tr("Found %1 match(es) on current page").arg(hits));
    });
    connect(zoomInAction, &QAction::triggered, this, [this]() { m_pageView->setZoom(1.25f); });
    connect(zoomOutAction, &QAction::triggered, this, [this]() { m_pageView->setZoom(0.8f); });
    connect(insertBlankPageAction, &QAction::triggered, this, [this]() { m_controller.insertBlankPage(); });

    // Connect annotation actions
    connect(highlightAction, &QAction::triggered, this, [this]() { m_controller.addAnnotation(tr("[Highlight]")); });
    connect(underlineAction, &QAction::triggered, &m_controller, &editor::EditorController::underlineAnnotation);
    connect(strikeoutAction, &QAction::triggered, &m_controller, &editor::EditorController::strikeoutAnnotation);
    connect(stickyNoteAction, &QAction::triggered, &m_controller, &editor::EditorController::stickyNoteAnnotation);
    connect(commentAction, &QAction::triggered, &m_controller, &editor::EditorController::commentAnnotation);
    connect(drawShapeAction, &QAction::triggered, &m_controller, &editor::EditorController::drawShape);
    connect(arrowAction, &QAction::triggered, &m_controller, &editor::EditorController::drawArrow);

    connect(addAnnotAction, &QAction::triggered, this, [this]() {
        bool ok = false;
        const QString text = QInputDialog::getText(this, tr("Annotation"), tr("Text:"), QLineEdit::Normal, tr("Note"), &ok);
        if (ok && !text.isEmpty()) {
            m_controller.addAnnotation(text);
        }
    });

    // Connect text/image actions
    connect(addTextAction, &QAction::triggered, this, [this]() {
        bool ok = false;
        const QString text = QInputDialog::getText(this, tr("Text Edit"), tr("Text:"), QLineEdit::Normal, tr("Text"), &ok);
        if (ok && !text.isEmpty()) {
            m_controller.addTextEdit(text);
        }
    });

    connect(replaceTextAction, &QAction::triggered, this, [this]() {
        bool okOld = false;
        const QString oldText = QInputDialog::getText(this, tr("Replace Text"), tr("Find:"), QLineEdit::Normal, {}, &okOld);
        if (!okOld || oldText.isEmpty()) {
            return;
        }
        bool okNew = false;
        const QString newText = QInputDialog::getText(this, tr("Replace Text"), tr("Replace with:"), QLineEdit::Normal, {}, &okNew);
        if (okNew) {
            m_controller.replaceText(oldText, newText);
        }
    });

    connect(editTextAction, &QAction::triggered, this, [this]() {
        bool ok = false;
        const QString text = QInputDialog::getText(this, tr("Edit Text"), tr("New text value:"), QLineEdit::Normal, {}, &ok);
        if (ok && !text.isEmpty()) {
            m_controller.editTextValue(text);
        }
    });
    connect(changeFontAction, &QAction::triggered, this, [this]() {
        bool ok = false;
        const QString font = QInputDialog::getText(this, tr("Change Font"), tr("Font tag (e.g. Helvetica):"), QLineEdit::Normal, tr("Helvetica"), &ok);
        if (ok && !font.isEmpty()) {
            m_controller.setTextFontTag(font);
        }
    });
    connect(changeFontSizeAction, &QAction::triggered, this, [this]() {
        bool ok = false;
        const double size = QInputDialog::getDouble(this, tr("Change Font Size"), tr("Font size:"), 14.0, 8.0, 72.0, 1, &ok);
        if (ok) {
            m_controller.setTextFontSize(size);
        }
    });
    connect(moveTextBlockAction, &QAction::triggered, this, [this]() {
        bool okX = false;
        const double dx = QInputDialog::getDouble(this, tr("Move Text"), tr("Delta X:"), 12.0, -2000.0, 2000.0, 1, &okX);
        if (!okX) {
            return;
        }
        bool okY = false;
        const double dy = QInputDialog::getDouble(this, tr("Move Text"), tr("Delta Y:"), 8.0, -2000.0, 2000.0, 1, &okY);
        if (okY) {
            m_controller.moveFirstTextBlock(QPointF(dx, dy));
        }
    });

    connect(addImageAction, &QAction::triggered, this, [this]() {
        const QString path = QFileDialog::getOpenFileName(this, tr("Insert Image"), {}, tr("Images (*.png *.jpg *.jpeg *.bmp)"));
        if (!path.isEmpty()) {
            m_controller.addImageOverlay(path);
        }
    });
    connect(moveImageAction, &QAction::triggered, this, [this]() {
        bool okX = false;
        const double dx = QInputDialog::getDouble(this, tr("Move Image"), tr("Delta X:"), 14.0, -2000.0, 2000.0, 1, &okX);
        if (!okX) {
            return;
        }
        bool okY = false;
        const double dy = QInputDialog::getDouble(this, tr("Move Image"), tr("Delta Y:"), 10.0, -2000.0, 2000.0, 1, &okY);
        if (okY) {
            m_controller.moveFirstImage(QPointF(dx, dy));
        }
    });
    connect(replaceImageAction, &QAction::triggered, &m_controller, &editor::EditorController::replaceImage);
    connect(deleteImageAction, &QAction::triggered, &m_controller, &editor::EditorController::deleteImage);
    connect(resizeImageAction, &QAction::triggered, this, [this]() {
        bool ok = false;
        const double scale = QInputDialog::getDouble(this, tr("Resize Image"), tr("Scale factor:"), 1.15, 0.1, 10.0, 2, &ok);
        if (ok) {
            m_controller.resizeFirstImage(scale);
        }
    });

    // Connect advanced features
    connect(createFormFieldAction, &QAction::triggered, &m_controller, &editor::EditorController::createFormField);
    connect(editFormFieldAction, &QAction::triggered, &m_controller, &editor::EditorController::editFormField);
    connect(fillFormAction, &QAction::triggered, &m_controller, &editor::EditorController::fillForm);
    connect(exportFormDataAction, &QAction::triggered, &m_controller, &editor::EditorController::exportFormData);
    connect(drawSignatureAction, &QAction::triggered, &m_controller, &editor::EditorController::drawSignature);
    connect(insertImageSignatureAction, &QAction::triggered, &m_controller, &editor::EditorController::insertImageSignature);
    connect(saveSignatureAction, &QAction::triggered, &m_controller, &editor::EditorController::saveSignature);
    connect(applySignatureAction, &QAction::triggered, &m_controller, &editor::EditorController::applySignature);
    connect(hideTextAction, &QAction::triggered, &m_controller, &editor::EditorController::hideText);
    connect(hideImagesAction, &QAction::triggered, &m_controller, &editor::EditorController::hideImages);
    connect(applyPermanentRedactionAction, &QAction::triggered, &m_controller, &editor::EditorController::applyPermanentRedaction);
    connect(passwordProtectionAction, &QAction::triggered, &m_controller, &editor::EditorController::passwordProtection);
    connect(restrictPrintingAction, &QAction::triggered, &m_controller, &editor::EditorController::restrictPrinting);
    connect(restrictCopyingAction, &QAction::triggered, &m_controller, &editor::EditorController::restrictCopying);
    connect(digitalSignatureAction, &QAction::triggered, &m_controller, &editor::EditorController::digitalSignature);
    connect(sharedAnnotationsAction, &QAction::triggered, &m_controller, &editor::EditorController::sharedAnnotations);
    connect(commentsCollabAction, &QAction::triggered, &m_controller, &editor::EditorController::commentsCollab);

    connect(ocrAction, &QAction::triggered, this, [this]() {
        const QString text = m_controller.runOcrOnCurrentPage();
        m_statusLabel->setText(text.left(240));
    });

    connect(mergeAction, &QAction::triggered, this, [this]() {
        if (!m_controller.isOpen()) {
            m_statusLabel->setText(tr("Open a base PDF first"));
            return;
        }
        const QString mergePath = QFileDialog::getOpenFileName(this, tr("Select PDF To Merge"), {}, tr("PDF Files (*.pdf)"));
        if (mergePath.isEmpty()) {
            return;
        }
        const QString outputPath = QFileDialog::getSaveFileName(this,
            tr("Save Merged PDF"),
            tr("merged-output.pdf"),
            tr("PDF Files (*.pdf)"));
        if (outputPath.isEmpty()) {
            return;
        }
        m_controller.mergeWithDocumentTo(mergePath, outputPath);
    });
    connect(splitAction, &QAction::triggered, this, [this]() {
        if (!m_controller.isOpen()) {
            m_statusLabel->setText(tr("Open a document first"));
            return;
        }

        bool ok = false;
        const QString defaultRange = QString::number(m_controller.currentPage() + 1);
        const QString rangeInput = QInputDialog::getText(
            this,
            tr("Split PDF"),
            tr("Page ranges (e.g. 1-3,5,8):"),
            QLineEdit::Normal,
            defaultRange,
            &ok);
        if (!ok || rangeInput.trimmed().isEmpty()) {
            return;
        }

        std::vector<int> pageIndexes;
        const QStringList parts = rangeInput.split(',', Qt::SkipEmptyParts);
        for (const QString& rawPart : parts) {
            const QString part = rawPart.trimmed();
            if (part.isEmpty()) {
                continue;
            }
            if (part.contains('-')) {
                const QStringList bounds = part.split('-', Qt::SkipEmptyParts);
                if (bounds.size() != 2) {
                    continue;
                }
                bool okStart = false;
                bool okEnd = false;
                const int start = bounds[0].trimmed().toInt(&okStart);
                const int end = bounds[1].trimmed().toInt(&okEnd);
                if (!okStart || !okEnd) {
                    continue;
                }
                const int from = std::min(start, end);
                const int to = std::max(start, end);
                for (int page = from; page <= to; ++page) {
                    const int index = page - 1;
                    if (index < 0 || index >= m_controller.pageCount()) {
                        continue;
                    }
                    if (std::find(pageIndexes.begin(), pageIndexes.end(), index) == pageIndexes.end()) {
                        pageIndexes.push_back(index);
                    }
                }
                continue;
            }

            bool okPage = false;
            const int page = part.toInt(&okPage);
            if (!okPage) {
                continue;
            }
            const int index = page - 1;
            if (index < 0 || index >= m_controller.pageCount()) {
                continue;
            }
            if (std::find(pageIndexes.begin(), pageIndexes.end(), index) == pageIndexes.end()) {
                pageIndexes.push_back(index);
            }
        }

        if (pageIndexes.empty()) {
            m_statusLabel->setText(tr("No valid pages in range input"));
            return;
        }

        std::sort(pageIndexes.begin(), pageIndexes.end());
        QStringList previewPages;
        for (const int idx : pageIndexes) {
            previewPages << QString::number(idx + 1);
        }
        const auto confirm = QMessageBox::question(
            this,
            tr("Confirm Split"),
            tr("Export %1 page(s): %2")
                .arg(pageIndexes.size())
                .arg(previewPages.join(tr(", "))),
            QMessageBox::Yes | QMessageBox::No,
            QMessageBox::Yes);
        if (confirm != QMessageBox::Yes) {
            return;
        }

        const QString path = QFileDialog::getSaveFileName(this, tr("Save Split Output"), tr("split-output.pdf"), tr("PDF Files (*.pdf)"));
        if (!path.isEmpty()) {
            m_controller.splitPagesTo(path, pageIndexes);
        }
    });
    connect(exportAsImagesAction, &QAction::triggered, this, [this]() {
        const QImage img = m_controller.renderCurrentPage(1.0f);
        if (img.isNull()) {
            m_statusLabel->setText(tr("No page to export"));
            return;
        }
        const QString path = QFileDialog::getSaveFileName(this, tr("Export Page As Image"), tr("page.png"), tr("PNG Image (*.png)"));
        if (!path.isEmpty() && img.save(path)) {
            m_statusLabel->setText(tr("Exported image: %1").arg(path));
        }
    });
    connect(exportAsTextAction, &QAction::triggered, this, [this]() {
        const QString text = m_controller.runOcrOnCurrentPage();
        if (text.isEmpty()) {
            m_statusLabel->setText(tr("No text extracted"));
            return;
        }
        const QString path = QFileDialog::getSaveFileName(this, tr("Export Page As Text"), tr("page.txt"), tr("Text File (*.txt)"));
        if (path.isEmpty()) {
            return;
        }
        QFile out(path);
        if (out.open(QIODevice::WriteOnly | QIODevice::Text)) {
            out.write(text.toUtf8());
            out.close();
            m_statusLabel->setText(tr("Exported text: %1").arg(path));
        }
    });
    connect(printAction, &QAction::triggered, this, [this]() {
        if (!m_controller.isOpen()) {
            m_statusLabel->setText(tr("Open a document first"));
            return;
        }
        const QString path = QFileDialog::getSaveFileName(this, tr("Print To PDF"), tr("print-output.pdf"), tr("PDF Files (*.pdf)"));
        if (!path.isEmpty()) {
            m_controller.saveDocument(path);
            m_statusLabel->setText(tr("Print-ready PDF saved: %1").arg(path));
        }
    });

    connect(aboutAction, &QAction::triggered, this, [this]() {
        QMessageBox::about(this,
                           tr("About PDF Editor Pro"),
                           tr("C++20 + Qt6 PDF editor prototype with overlay-based editing and command stack."));
    });

    connect(showThumbnailsAction, &QAction::toggled, m_thumbnails, &ThumbnailPanel::setVisible);
    connect(showInspectorAction, &QAction::toggled, m_properties, &InspectorPanel::setVisible);

    // Toolbar
    m_toolbar->addAction(openAction);
    m_toolbar->addAction(saveAction);
    m_toolbar->addSeparator();
    m_toolbar->addAction(undoAction);
    m_toolbar->addAction(redoAction);
    m_toolbar->addSeparator();
    m_toolbar->addAction(highlightAction);
    m_toolbar->addAction(addAnnotAction);
    m_toolbar->addAction(addTextAction);
    m_toolbar->addAction(addImageAction);
    m_toolbar->addSeparator();
    m_toolbar->addAction(drawShapeAction);
    m_toolbar->addAction(createFormFieldAction);
    m_toolbar->addAction(applySignatureAction);
    m_toolbar->addAction(hideTextAction);
    m_toolbar->addAction(ocrAction);

    rebuildRecentMenu();
}

void MainWindow::rebuildRecentMenu() {
    if (!m_openRecentMenu) {
        return;
    }

    m_openRecentMenu->clear();
    const QStringList recents = m_controller.recentFiles();
    if (recents.isEmpty()) {
        auto* emptyAction = m_openRecentMenu->addAction(tr("No recent files"));
        emptyAction->setEnabled(false);
        return;
    }

    for (int i = 0; i < recents.size(); ++i) {
        const QString path = recents.at(i);
        auto* action = m_openRecentMenu->addAction(path);
        connect(action, &QAction::triggered, this, [this, i]() { m_controller.openRecent(i); });
    }
}

void MainWindow::refreshPanels() {
    m_thumbnails->clear();
    m_properties->clear();

    if (!m_controller.isOpen()) {
        m_thumbnails->addItem(tr("No document open"));
        m_properties->addItem(tr("No document open"));
        return;
    }

    for (int i = 0; i < m_controller.pageCount(); ++i) {
        m_thumbnails->addItem(tr("Page %1").arg(i + 1));
    }

    const auto* pageModel = m_controller.currentPageModel();
    m_properties->addItem(tr("Current page: %1").arg(m_controller.currentPage() + 1));
    m_properties->addItem(tr("Page count: %1").arg(std::max(0, m_controller.pageCount())));

    if (!pageModel) {
        m_properties->addItem(tr("Open a document to inspect overlays"));
        return;
    }

    m_properties->addItem(tr("Overlays: %1").arg(pageModel->overlayObjects.size()));
    int idx = 1;
    for (const auto& overlay : pageModel->overlayObjects) {
        if (!overlay) {
            continue;
        }
        QString type;
        switch (overlay->kind()) {
        case overlay::OverlayObject::Kind::Annotation: type = tr("Annotation"); break;
        case overlay::OverlayObject::Kind::TextEdit: type = tr("TextEdit"); break;
        case overlay::OverlayObject::Kind::ImageEdit: type = tr("Image"); break;
        case overlay::OverlayObject::Kind::Shape: type = tr("Shape"); break;
        }
        m_properties->addItem(tr("%1. %2").arg(idx++).arg(type));
    }
}
