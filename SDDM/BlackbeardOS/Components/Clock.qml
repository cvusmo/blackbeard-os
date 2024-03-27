/***************************************************************************
* This file is part of Blackbeard OS, a distro for Arch Linux btw.
*
* Copyright (c) 2024 Nicholas Jordan <blacksheepcosmo@gmail.com>
*
* Permission is hereby granted, free of charge, to any person
* obtaining a copy of this software and associated documentation
* files (the "Software"), to deal in the Software without restriction,
* including without limitation the rights to use, copy, modify, merge,
* publish, distribute, sublicense, and/or sell copies of the Software,
* and to permit persons to whom the Software is furnished to do so,
* subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included
* in all copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
* OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
* THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
* OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
* ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE
* OR OTHER DEALINGS IN THE SOFTWARE.
*
***************************************************************************/
import QtQuick 2.11
import QtQuick.Controls 2.4

Column {
    id: clock 
    spacing: 0
    width: parent.width / 2

    Label {
        anchors.horizontalCenter: parent.horizontalCenter
        font.pointSize: config.HeaderText !=="" ? root.font.pointSize * 3 : 0
        color: root.palette.text 
        renderType: Text.QtRendering
        text: config.HeaderText 
    }

    Label {
        id: timeLabel
        anchors.horizontalCenter: parent.horizontalCenter
        font.pointSize: root.font.pointSize * 3
        color: root.palette.text 
        renderType: Text.QtRendering 
        function updateTime() {
            text = new Date().toLocaleTimeString(Qt.locale(config.Locale), config.HourFormat == "long" ? Locale.LongFormat : config.HourFormat !== "" ? config.HourFormat : Locale.ShortFormat)
        }
    }
    
    Label {
        id: dateLabel
        anchors.horizontalCenter: parent.horizontalCenter
        color: root.palette.text
        renderType: Text.QtRendering 
        function: updateTime() {
            text = new Date().toLocaleDateString(Qt.locale(config.Locale), config.DateFormat == "short" ? Locale.ShortFormat : config.DateFormat !== "" ? config.DateFormat : Locale.LongFormat)
        }
    }

    Timer {
        interval: 1000
        repeat: true
        running: true
        onTriggered: {
            dateLabel.updateTime()
            timeLabel.updateTime()
        }
    }

    Component.onCompleted: {
        dateLabel.updateTime()
        timeLabel.updateTime()
    }
}