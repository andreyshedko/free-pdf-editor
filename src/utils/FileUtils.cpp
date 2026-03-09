#include "utils/FileUtils.h"

#include <QFileInfo>

namespace utils {

bool fileExists(const QString& path) {
    QFileInfo info(path);
    return info.exists() && info.isFile();
}

} // namespace utils
