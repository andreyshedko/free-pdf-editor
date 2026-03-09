#include "utils/Logger.h"

#include <QDebug>

namespace utils {

void logInfo(const QString& message) {
    qInfo().noquote() << message;
}

void logError(const QString& message) {
    qWarning().noquote() << message;
}

} // namespace utils
