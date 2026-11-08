#!/bin/bash

adb -s 192.168.0.101:37313 shell mkdir -p /data/data/com.niviid99.empetri/files/Music
adb push "$PWD/${1}" /data/local/tmp/track.mp3
adb shell run-as com.niviid99.empetri cp /data/local/tmp/track.mp3 files/Music/${1}
