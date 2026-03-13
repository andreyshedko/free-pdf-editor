#pragma once

#include "document/DocumentMetadata.h"
#include "document/PageModel.h"

#include <QByteArray>
#include <QString>
#include <vector>

namespace document {

class Document {
public:
    bool open(const QString& path);
    bool save(const QString& path);

    [[nodiscard]] bool isOpen() const;
    [[nodiscard]] const QString& path() const;
    [[nodiscard]] const QByteArray& sourceBytes() const;
    [[nodiscard]] const DocumentMetadata& metadata() const;
    [[nodiscard]] bool hasEditableOverlayMetadata() const;
    [[nodiscard]] const QString& editableOverlayMetadataPath() const;
    [[nodiscard]] QString expectedEditableOverlayMetadataPath() const;

    [[nodiscard]] int pageCount() const;
    [[nodiscard]] PageModel& page(int index);
    [[nodiscard]] const PageModel& page(int index) const;

    bool deletePage(int index, PageModel* removedPage = nullptr);
    bool insertPage(int index, const PageModel& page);

    void reset();

private:
    void renumberPages();

    QString m_path;
    QByteArray m_sourceBytes;
    std::vector<PageModel> m_pages;
    DocumentMetadata m_metadata;
    bool m_hasEditableOverlayMetadata {false};
    QString m_editableOverlayMetadataPath;
};

} // namespace document
