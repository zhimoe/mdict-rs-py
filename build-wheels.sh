#!/bin/bash

# 多平台构建脚本
# 使用此脚本可以在本地构建多平台wheel文件

set -e

echo "Building mdict-rs wheels for multiple platforms..."

# 清理之前的构建
rm -rf dist build target

# 构建当前平台的wheel
echo "Building for current platform..."
maturin build --release --out dist --find-interpreter

# 构建源码分发包
echo "Building source distribution..."
maturin sdist --out dist

echo "Build completed!"
echo "Wheels and sdist are in the 'dist' directory:"
ls -la dist/