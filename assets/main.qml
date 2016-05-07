import QtQuick 2.3
import QtQuick.Controls 1.2
import QtQuick.Window 2.2

//window containing the application
ApplicationWindow {
    visible: true

    //title of the application
    title: qsTr("faiyels")
    width: 300
    height: Screen.desktopAvailableHeight

    //menu containing two menu items
    menuBar: MenuBar {
        Menu {
            title: qsTr("File")
            MenuItem {
                text: qsTr("Exit")
                onTriggered: Qt.quit();
            }
        }
    }

    //Content Area

    //a button in the middle of the content area
    Button {
        text: qsTr("Hello World")
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.verticalCenter: parent.verticalCenter
    }
}
