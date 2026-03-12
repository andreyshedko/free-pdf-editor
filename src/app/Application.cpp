#include "app/Application.h"

#include "ui/MainWindow.h"

#include <QApplication>
#include <QTranslator>
#include <QLocale>

int Application::run(int argc, char* argv[]) {
    QApplication app(argc, argv);
    
    // Load translations based on system language
    QTranslator translator;
    const QString locale = QLocale::system().name(); // e.g., "en_US", "de_DE", "fr_FR"
    
    // Try to load translations from the app resources (compiled .qm files)
    if (translator.load(":/translations/pdfditor_" + locale + ".qm")) {
        app.installTranslator(&translator);
    } else if (!locale.isEmpty()) {
        // Fallback to language code if full locale wasn't found (e.g., "de" instead of "de_DE")
        QString langCode = locale.left(2);
        if (translator.load(":/translations/pdfditor_" + langCode + ".qm")) {
            app.installTranslator(&translator);
        }
    }
    // If no translation found, app continues in English (default)
    
    MainWindow window;
    window.show();
    return app.exec();
}

