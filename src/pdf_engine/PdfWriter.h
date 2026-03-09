#pragma once

#include <QString>

namespace document { class Document; }
namespace pdf_engine { class PdfRenderer; }

namespace pdf_engine {

class PdfWriter {
public:
    bool save(const document::Document& document, const QString& path, PdfRenderer& renderer) const;
};

} // namespace pdf_engine
