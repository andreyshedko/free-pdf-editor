#pragma once

#include <QByteArray>
#include <QImage>

namespace pdf_engine {

class PdfiumBridge {
public:
    PdfiumBridge();
    ~PdfiumBridge();

    bool isAvailable() const;
    int pageCount(const QByteArray& pdfBytes) const;
    QImage renderPage(const QByteArray& pdfBytes, int pageIndex, float scale) const;

private:
    struct Impl;
    Impl* m_impl;
};

} // namespace pdf_engine
