#!/bin/bash

# Docker 交叉编译 Windows 版本

set -e

echo "🐳 使用 Docker 构建 Windows 版本..."

# 构建 Docker 镜像
echo "🔨 构建 Docker 镜像..."
docker build -f Dockerfile.windows -t ruziniu-tools-windows .

# 运行构建容器
echo "🚀 开始构建..."
docker run --rm -v "$(pwd)":/app ruziniu-tools-windows

echo "✅ Docker 构建完成！"
echo "📦 输出位置: src-tauri/target/x86_64-pc-windows-gnu/release/bundle/nsis/"