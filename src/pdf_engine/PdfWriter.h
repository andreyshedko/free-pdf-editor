#pragma once

#include <QString>
#include <QVector>

class QImage;

namespace document { class Document; }
namespace pdf_engine { class PdfRenderer; }

namespace pdf_engine {

class PdfWriter {
public:
    bool save(const document::Document& document, const QString& path, PdfRenderer& renderer) const;
    bool saveRenderedPages(const QVector<QImage>& pages, const QString& path) const;
};

} // namespace pdf_engine
