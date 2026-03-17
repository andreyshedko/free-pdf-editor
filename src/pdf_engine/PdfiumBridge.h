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
    QString extractText(const QByteArray& pdfBytes, int pageIndex) const;
    std::vector<QRectF> findTextRects(const QByteArray& pdfBytes, int pageIndex, const QString& needle) const;

private:
    struct Impl;
    Impl* m_impl;
};

} // namespace pdf_engine
