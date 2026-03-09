#include "ui/MainWindow.h"

#include "overlay/OverlayObject.h"
#include "ui/InspectorPanel.h"
#include "ui/PageView.h"
#include "ui/ThumbnailPanel.h"
#include "ui/Toolbar.h"

#include <algorithm>
#include <QAction>
#include <QFileDialog>
#include <QHBoxLayout>
#include <QInputDialog>
#include <QLabel>
#include <QLineEdit>
#include <QMenu>
#include <QMenuBar>
#include <QStatusBar>
#include <QWidget>

MainWindow::MainWindow(QWidget* parent)
    : QMainWindow(parent) {
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
        m_statusLabel->setText(QStringLiteral("Page %1 / %2").arg(page + 1).arg(count));
        refreshPanels();
    });
    connect(&m_controller, &editor::EditorController::recentFilesChanged, this, [this]() {
        rebuildRecentMenu();
        refreshPanels();
    });
}

void MainWindow::setupUi() {
    resize(1360, 900);
    setWindowTitle(QStringLiteral("Free PDF Editor (C++/Qt)"));

    auto* central = new QWidget(this);
    auto* layout = new QHBoxLayout(central);
    layout->setContentsMargins(8, 8, 8, 8);
    layout->setSpacing(8);

    m_thumbnails = new ThumbnailPanel(central);
    m_pageView = new PageView(m_controller, central);
    m_properties = new InspectorPanel(central);

    layout->addWidget(m_thumbnails);
    layout->addWidget(m_pageView, 1);
    layout->addWidget(m_properties);
    setCentralWidget(central);

    m_toolbar = new Toolbar(this);
    addToolBar(m_toolbar);

    m_statusLabel = new QLabel(QStringLiteral("Ready"), this);
    statusBar()->addPermanentWidget(m_statusLabel, 1);
}

void MainWindow::setupActions() {
    auto* fileMenu = menuBar()->addMenu(QStringLiteral("File"));
    auto* editMenu = menuBar()->addMenu(QStringLiteral("Edit"));
    auto* pageMenu = menuBar()->addMenu(QStringLiteral("Page"));
    auto* toolsMenu = menuBar()->addMenu(QStringLiteral("Tools"));

    m_openRecentMenu = fileMenu->addMenu(QStringLiteral("Open Recent"));

    auto* openAction = new QAction(QStringLiteral("Open"), this);
    auto* saveAction = new QAction(QStringLiteral("Save As"), this);
    auto* addAnnotAction = new QAction(QStringLiteral("Add Annotation"), this);
    auto* addTextAction = new QAction(QStringLiteral("Add Text Edit"), this);
    auto* replaceTextAction = new QAction(QStringLiteral("Replace Text"), this);
    auto* addImageAction = new QAction(QStringLiteral("Insert Image"), this);
    auto* moveOverlayAction = new QAction(QStringLiteral("Move Selected"), this);
    auto* deletePageAction = new QAction(QStringLiteral("Delete Page"), this);
    auto* ocrAction = new QAction(QStringLiteral("Run OCR"), this);
    auto* undoAction = new QAction(QStringLiteral("Undo"), this);
    auto* redoAction = new QAction(QStringLiteral("Redo"), this);

    connect(openAction, &QAction::triggered, this, [this]() {
        const QString path = QFileDialog::getOpenFileName(this, QStringLiteral("Open PDF"), {}, QStringLiteral("PDF Files (*.pdf)"));
        if (!path.isEmpty()) {
            m_controller.openDocument(path);
        }
    });

    connect(saveAction, &QAction::triggered, this, [this]() {
        const QString path = QFileDialog::getSaveFileName(this, QStringLiteral("Save PDF"), {}, QStringLiteral("PDF Files (*.pdf)"));
        if (!path.isEmpty()) {
            m_controller.saveDocument(path);
        }
    });

    connect(addAnnotAction, &QAction::triggered, this, [this]() {
        bool ok = false;
        const QString text = QInputDialog::getText(this, QStringLiteral("Annotation"), QStringLiteral("Text:"), QLineEdit::Normal, QStringLiteral("Note"), &ok);
        if (ok && !text.isEmpty()) {
            m_controller.addAnnotation(text);
        }
    });

    connect(addTextAction, &QAction::triggered, this, [this]() {
        bool ok = false;
        const QString text = QInputDialog::getText(this, QStringLiteral("Text Edit"), QStringLiteral("Text:"), QLineEdit::Normal, QStringLiteral("Text"), &ok);
        if (ok && !text.isEmpty()) {
            m_controller.addTextEdit(text);
        }
    });

    connect(replaceTextAction, &QAction::triggered, this, [this]() {
        bool okOld = false;
        const QString oldText = QInputDialog::getText(this, QStringLiteral("Replace Text"), QStringLiteral("Find:"), QLineEdit::Normal, {}, &okOld);
        if (!okOld || oldText.isEmpty()) {
            return;
        }
        bool okNew = false;
        const QString newText = QInputDialog::getText(this, QStringLiteral("Replace Text"), QStringLiteral("Replace with:"), QLineEdit::Normal, {}, &okNew);
        if (okNew) {
            m_controller.replaceText(oldText, newText);
        }
    });

    connect(addImageAction, &QAction::triggered, this, [this]() {
        const QString path = QFileDialog::getOpenFileName(this, QStringLiteral("Insert Image"), {}, QStringLiteral("Images (*.png *.jpg *.jpeg *.bmp)"));
        if (!path.isEmpty()) {
            m_controller.addImageOverlay(path);
        }
    });

    connect(moveOverlayAction, &QAction::triggered, this, [this]() {
        m_controller.selectLastOverlay();
        m_controller.moveSelectedBy(QPointF(20.0, 12.0));
    });

    connect(deletePageAction, &QAction::triggered, this, [this]() { m_controller.deleteCurrentPage(); });
    connect(ocrAction, &QAction::triggered, this, [this]() {
        const QString text = m_controller.runOcrOnCurrentPage();
        m_statusLabel->setText(text.left(240));
    });
    connect(undoAction, &QAction::triggered, &m_controller, &editor::EditorController::undo);
    connect(redoAction, &QAction::triggered, &m_controller, &editor::EditorController::redo);

    fileMenu->addAction(openAction);
    fileMenu->addAction(saveAction);
    editMenu->addAction(undoAction);
    editMenu->addAction(redoAction);
    pageMenu->addAction(deletePageAction);
    toolsMenu->addAction(addAnnotAction);
    toolsMenu->addAction(addTextAction);
    toolsMenu->addAction(replaceTextAction);
    toolsMenu->addAction(addImageAction);
    toolsMenu->addAction(moveOverlayAction);
    toolsMenu->addAction(ocrAction);

    m_toolbar->addAction(openAction);
    m_toolbar->addAction(saveAction);
    m_toolbar->addSeparator();
    m_toolbar->addAction(addAnnotAction);
    m_toolbar->addAction(addTextAction);
    m_toolbar->addAction(replaceTextAction);
    m_toolbar->addAction(addImageAction);
    m_toolbar->addAction(moveOverlayAction);
    m_toolbar->addAction(deletePageAction);
    m_toolbar->addAction(ocrAction);
    m_toolbar->addSeparator();
    m_toolbar->addAction(undoAction);
    m_toolbar->addAction(redoAction);

    rebuildRecentMenu();
}

void MainWindow::rebuildRecentMenu() {
    if (!m_openRecentMenu) {
        return;
    }

    m_openRecentMenu->clear();
    const QStringList recents = m_controller.recentFiles();
    if (recents.isEmpty()) {
        auto* emptyAction = m_openRecentMenu->addAction(QStringLiteral("No recent files"));
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

    m_thumbnails->addItem(QStringLiteral("Recent files"));
    const QStringList recents = m_controller.recentFiles();
    for (const QString& path : recents) {
        m_thumbnails->addItem(path);
    }
    if (recents.isEmpty()) {
        m_thumbnails->addItem(QStringLiteral("No recent files"));
    }

    const auto* pageModel = m_controller.currentPageModel();
    m_properties->addItem(QStringLiteral("Current page: %1").arg(m_controller.currentPage() + 1));
    m_properties->addItem(QStringLiteral("Page count: %1").arg(std::max(0, m_controller.pageCount())));

    if (!pageModel) {
        m_properties->addItem(QStringLiteral("Open a document to inspect overlays"));
        return;
    }

    m_properties->addItem(QStringLiteral("Overlays: %1").arg(pageModel->overlayObjects.size()));
    int idx = 1;
    for (const auto& overlay : pageModel->overlayObjects) {
        if (!overlay) {
            continue;
        }
        QString type;
        switch (overlay->kind()) {
        case overlay::OverlayObject::Kind::Annotation: type = QStringLiteral("Annotation"); break;
        case overlay::OverlayObject::Kind::TextEdit: type = QStringLiteral("TextEdit"); break;
        case overlay::OverlayObject::Kind::ImageEdit: type = QStringLiteral("Image"); break;
        case overlay::OverlayObject::Kind::Shape: type = QStringLiteral("Shape"); break;
        }
        m_properties->addItem(QStringLiteral("%1. %2").arg(idx++).arg(type));
    }
}
