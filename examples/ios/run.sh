#!/bin/sh

# 构建Rust库
cargo build --release --target aarch64-apple-ios

# 打开Xcode项目
open YongguangLegend.xcodeproj