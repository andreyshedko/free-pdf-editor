#include "pdf_engine/PdfiumBridge.h"

#include <QDir>

#ifdef _WIN32
#include <windows.h>
#endif

namespace pdf_engine {

#ifdef _WIN32

using FPDF_DOCUMENT = void*;
using FPDF_PAGE = void*;
using FPDF_BITMAP = void*;

using FnInit = void (*)();
using FnDestroy = void (*)();
using FnLoadMem = FPDF_DOCUMENT (*)(const void*, int, const char*);
using FnCloseDoc = void (*)(FPDF_DOCUMENT);
using FnPageCount = int (*)(FPDF_DOCUMENT);
using FnLoadPage = FPDF_PAGE (*)(FPDF_DOCUMENT, int);
using FnClosePage = void (*)(FPDF_PAGE);
using FnPageW = float (*)(FPDF_PAGE);
using FnPageH = float (*)(FPDF_PAGE);
using FnBmpCreate = FPDF_BITMAP (*)(int, int, int, void*, int);
using FnBmpDestroy = void (*)(FPDF_BITMAP);
using FnBmpBuffer = void* (*)(FPDF_BITMAP);
using FnBmpStride = int (*)(FPDF_BITMAP);
using FnBmpFill = void (*)(FPDF_BITMAP, int, int, int, int, unsigned int);
using FnRender = void (*)(FPDF_BITMAP, FPDF_PAGE, int, int, int, int, int, int);
using FPDF_TEXTPAGE = void*;
using FnTextLoadPage = FPDF_TEXTPAGE (*)(FPDF_PAGE);
using FnTextClosePage = void (*)(FPDF_TEXTPAGE);
using FnTextCountChars = int (*)(FPDF_TEXTPAGE);
using FnTextGetText = int (*)(FPDF_TEXTPAGE, int, int, unsigned short*);

#endif

struct PdfiumBridge::Impl {
    bool ok {false};
#ifdef _WIN32
    HMODULE dll {nullptr};
    FnInit initLib {nullptr};
    FnDestroy destroyLib {nullptr};
    FnLoadMem loadMem {nullptr};
    FnCloseDoc closeDoc {nullptr};
    FnPageCount pageCount {nullptr};
    FnLoadPage loadPage {nullptr};
    FnClosePage closePage {nullptr};
    FnPageW pageW {nullptr};
    FnPageH pageH {nullptr};
    FnBmpCreate bmpCreate {nullptr};
    FnBmpDestroy bmpDestroy {nullptr};
    FnBmpBuffer bmpBuffer {nullptr};
    FnBmpStride bmpStride {nullptr};
    FnBmpFill bmpFill {nullptr};
    FnRender renderPageBitmap {nullptr};
    FnTextLoadPage textLoadPage {nullptr};
    FnTextClosePage textClosePage {nullptr};
    FnTextCountChars textCountChars {nullptr};
    FnTextGetText textGetText {nullptr};
#endif
};

PdfiumBridge::PdfiumBridge()
    : m_impl(new Impl) {
#ifdef _WIN32
    QStringList candidates;
    const QString env = qEnvironmentVariable("PDFIUM_DLL");
    if (!env.isEmpty()) {
        candidates << env;
    }
    candidates << QStringLiteral("pdfium.dll");

    for (const QString& candidate : candidates) {
        m_impl->dll = LoadLibraryW(reinterpret_cast<LPCWSTR>(candidate.utf16()));
        if (m_impl->dll) {
            break;
        }
    }

    if (!m_impl->dll) {
        return;
    }

    auto sym = [this](const char* name) -> FARPROC { return GetProcAddress(m_impl->dll, name); };
    m_impl->initLib = reinterpret_cast<FnInit>(sym("FPDF_InitLibrary"));
    m_impl->destroyLib = reinterpret_cast<FnDestroy>(sym("FPDF_DestroyLibrary"));
    m_impl->loadMem = reinterpret_cast<FnLoadMem>(sym("FPDF_LoadMemDocument"));
    m_impl->closeDoc = reinterpret_cast<FnCloseDoc>(sym("FPDF_CloseDocument"));
    m_impl->pageCount = reinterpret_cast<FnPageCount>(sym("FPDF_GetPageCount"));
    m_impl->loadPage = reinterpret_cast<FnLoadPage>(sym("FPDF_LoadPage"));
    m_impl->closePage = reinterpret_cast<FnClosePage>(sym("FPDF_ClosePage"));
    m_impl->pageW = reinterpret_cast<FnPageW>(sym("FPDF_GetPageWidthF"));
    m_impl->pageH = reinterpret_cast<FnPageH>(sym("FPDF_GetPageHeightF"));
    m_impl->bmpCreate = reinterpret_cast<FnBmpCreate>(sym("FPDFBitmap_CreateEx"));
    m_impl->bmpDestroy = reinterpret_cast<FnBmpDestroy>(sym("FPDFBitmap_Destroy"));
    m_impl->bmpBuffer = reinterpret_cast<FnBmpBuffer>(sym("FPDFBitmap_GetBuffer"));
    m_impl->bmpStride = reinterpret_cast<FnBmpStride>(sym("FPDFBitmap_GetStride"));
    m_impl->bmpFill = reinterpret_cast<FnBmpFill>(sym("FPDFBitmap_FillRect"));
    m_impl->renderPageBitmap = reinterpret_cast<FnRender>(sym("FPDF_RenderPageBitmap"));
    m_impl->textLoadPage = reinterpret_cast<FnTextLoadPage>(sym("FPDFText_LoadPage"));
    m_impl->textClosePage = reinterpret_cast<FnTextClosePage>(sym("FPDFText_ClosePage"));
    m_impl->textCountChars = reinterpret_cast<FnTextCountChars>(sym("FPDFText_CountChars"));
    m_impl->textGetText = reinterpret_cast<FnTextGetText>(sym("FPDFText_GetText"));

    m_impl->ok = m_impl->initLib && m_impl->destroyLib && m_impl->loadMem && m_impl->closeDoc
        && m_impl->pageCount && m_impl->loadPage && m_impl->closePage && m_impl->pageW
        && m_impl->pageH && m_impl->bmpCreate && m_impl->bmpDestroy && m_impl->bmpBuffer
        && m_impl->bmpStride && m_impl->bmpFill && m_impl->renderPageBitmap
        && m_impl->textLoadPage && m_impl->textClosePage && m_impl->textCountChars && m_impl->textGetText;

    if (!m_impl->ok) {
        FreeLibrary(m_impl->dll);
        m_impl->dll = nullptr;
        return;
    }

    m_impl->initLib();
#endif
}

PdfiumBridge::~PdfiumBridge() {
#ifdef _WIN32
    if (m_impl && m_impl->ok && m_impl->destroyLib) {
        m_impl->destroyLib();
    }
    if (m_impl && m_impl->dll) {
        FreeLibrary(m_impl->dll);
    }
#endif
    delete m_impl;
}

bool PdfiumBridge::isAvailable() const {
    return m_impl && m_impl->ok;
}

int PdfiumBridge::pageCount(const QByteArray& pdfBytes) const {
#ifdef _WIN32
    if (!isAvailable() || pdfBytes.isEmpty()) {
        return 0;
    }
    FPDF_DOCUMENT doc = m_impl->loadMem(pdfBytes.constData(), pdfBytes.size(), nullptr);
    if (!doc) {
        return 0;
    }
    const int count = m_impl->pageCount(doc);
    m_impl->closeDoc(doc);
    return count;
#else
    Q_UNUSED(pdfBytes)
    return 0;
#endif
}

QImage PdfiumBridge::renderPage(const QByteArray& pdfBytes, int pageIndex, float scale) const {
#ifdef _WIN32
    if (!isAvailable() || pdfBytes.isEmpty()) {
        return {};
    }

    FPDF_DOCUMENT doc = m_impl->loadMem(pdfBytes.constData(), pdfBytes.size(), nullptr);
    if (!doc) {
        return {};
    }

    FPDF_PAGE page = m_impl->loadPage(doc, pageIndex);
    if (!page) {
        m_impl->closeDoc(doc);
        return {};
    }

    const int width = static_cast<int>(m_impl->pageW(page) * scale);
    const int height = static_cast<int>(m_impl->pageH(page) * scale);
    if (width <= 0 || height <= 0) {
        m_impl->closePage(page);
        m_impl->closeDoc(doc);
        return {};
    }

    FPDF_BITMAP bmp = m_impl->bmpCreate(width, height, 4, nullptr, 0);
    if (!bmp) {
        m_impl->closePage(page);
        m_impl->closeDoc(doc);
        return {};
    }

    m_impl->bmpFill(bmp, 0, 0, width, height, 0xFFFFFFFF);
    m_impl->renderPageBitmap(bmp, page, 0, 0, width, height, 0, 0);

    const auto* src = static_cast<const uchar*>(m_impl->bmpBuffer(bmp));
    const int stride = m_impl->bmpStride(bmp);
    QImage img(width, height, QImage::Format_ARGB32);
    for (int y = 0; y < height; ++y) {
        memcpy(img.scanLine(y), src + y * stride, static_cast<size_t>(width) * 4);
    }

    m_impl->bmpDestroy(bmp);
    m_impl->closePage(page);
    m_impl->closeDoc(doc);
    return img.rgbSwapped(); // PDFium BGRA -> ARGB
#else
    Q_UNUSED(pdfBytes)
    Q_UNUSED(pageIndex)
    Q_UNUSED(scale)
    return {};
#endif
}

QString PdfiumBridge::extractText(const QByteArray& pdfBytes, int pageIndex) const {
#ifdef _WIN32
    if (!isAvailable() || pdfBytes.isEmpty()) {
        return {};
    }

    FPDF_DOCUMENT doc = m_impl->loadMem(pdfBytes.constData(), pdfBytes.size(), nullptr);
    if (!doc) {
        return {};
    }

    FPDF_PAGE page = m_impl->loadPage(doc, pageIndex);
    if (!page) {
        m_impl->closeDoc(doc);
        return {};
    }

    FPDF_TEXTPAGE textPage = m_impl->textLoadPage(page);
    if (!textPage) {
        m_impl->closePage(page);
        m_impl->closeDoc(doc);
        return {};
    }

    const int charsCount = m_impl->textCountChars(textPage);
    if (charsCount <= 0) {
        m_impl->textClosePage(textPage);
        m_impl->closePage(page);
        m_impl->closeDoc(doc);
        return {};
    }

    // Allocate buffer for UTF-16LE text (including null terminator)
    std::vector<unsigned short> buffer(static_cast<size_t>(charsCount) + 1, 0);
    const int readCount = m_impl->textGetText(textPage, 0, charsCount, buffer.data());
    
    QString extracted;
    if (readCount > 0) {
        extracted = QString::fromUtf16(buffer.data());
    }

    m_impl->textClosePage(textPage);
    m_impl->closePage(page);
    m_impl->closeDoc(doc);
    return extracted;
#else
    Q_UNUSED(pdfBytes)
    Q_UNUSED(pageIndex)
    return {};
#endif
}

} // namespace pdf_engine
