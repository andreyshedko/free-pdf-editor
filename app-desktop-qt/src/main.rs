use qmetaobject::QmlEngine;
use tracing::info;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("app_desktop_qt=debug".parse().expect("valid directive")),
        )
        .init();

    info!("Free PDF Editor Qt frontend starting");

    let mut engine = QmlEngine::new();
    engine.load_data(
        r##"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

ApplicationWindow {
    id: root
    width: 1220
    height: 780
    visible: true
    title: "Free PDF Editor (Qt)"

    ColumnLayout {
        anchors.fill: parent
        spacing: 8
        anchors.margins: 10

        RowLayout {
            Layout.fillWidth: true
            spacing: 8
            Button { text: "Open" }
            Button { text: "Save" }
            Button { text: "Insert Text" }
            Item { Layout.fillWidth: true }
            Label { text: "Qt migration shell" }
        }

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            border.color: "#c7c7c7"
            border.width: 1
            color: "#f9fafb"

            Text {
                anchors.centerIn: parent
                text: "Canvas placeholder (Qt)\nController migration in progress"
                color: "#334155"
                horizontalAlignment: Text.AlignHCenter
            }
        }

        Rectangle {
            Layout.fillWidth: true
            height: 34
            color: "#111827"
            Text {
                anchors.verticalCenter: parent.verticalCenter
                anchors.left: parent.left
                anchors.leftMargin: 10
                text: "Status: ready"
                color: "white"
            }
        }
    }
}
"##
        .into(),
    );
    engine.exec();
}

