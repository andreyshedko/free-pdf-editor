#include "app/Application.h"

#include "ui/MainWindow.h"

#include <QApplication>

int Application::run(int argc, char* argv[]) {
    QApplication app(argc, argv);
    MainWindow window;
    window.show();
    return app.exec();
}
