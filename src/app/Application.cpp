#include "app/Application.h"

#include "ui/MainWindow.h"

#include <QApplication>
#include <QTranslator>
#include <QLocale>

int Application::run(int argc, char* argv[]) {
    QCoreApplication::setAttribute(Qt::AA_DontShowIconsInMenus, false);
    QCoreApplication::setAttribute(Qt::AA_DontShowShortcutsInContextMenus, false);

    QApplication app(argc, argv);
    
    // Load translations based on system language.
    QTranslator translator;
    const QString locale = QLocale::system().name(); // e.g., "en_US", "de_DE", "fr_FR"

    const auto tryLoadTranslation = [&translator](const QString& baseName) {
        const QStringList candidates {
            QStringLiteral(":/translations/%1.qm").arg(baseName),
            QStringLiteral(":/translations/translations/%1.qm").arg(baseName)
        };
        for (const QString& candidate : candidates) {
            if (translator.load(candidate)) {
                return true;
            }
        }
        return false;
    };

    bool loaded = false;
    const QStringList stems {
        QStringLiteral("pdfditor_"),
        QStringLiteral("pdfeditor_")
    };
    for (const QString& stem : stems) {
        if (tryLoadTranslation(stem + locale)) {
            loaded = true;
            break;
        }
    }
    if (!loaded && locale.size() >= 2) {
        const QString langCode = locale.left(2);
        for (const QString& stem : stems) {
            if (tryLoadTranslation(stem + langCode)) {
                loaded = true;
                break;
            }
        }
    }
    if (loaded) {
        app.installTranslator(&translator);
    }
    // If no translation found, app continues in English (default)
    
    MainWindow window;
    window.show();
    return app.exec();
}

