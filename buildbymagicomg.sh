#!/bin/bash
RUSTFLAGS="-C link-arg=--target=aarch64-linux-android26" cargo apk build --target aarch64-linux-android
