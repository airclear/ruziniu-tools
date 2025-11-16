#!/bin/bash

# 本地交叉编译 Windows 版本的构建脚本

set -e

echo "🚀 开始交叉编译 Windows 版本..."

# 检查是否已安装 Windows 目标
if ! rustup target list --installed | grep -q "x86_64-pc-windows-msvc"; then
    echo "❌ 未找到 Windows 目标平台，请先运行: rustup target add x86_64-pc-windows-msvc"
    exit 1
fi

# 清理之前的构建
echo "🧹 清理之前的构建..."
npm run build

# 设置环境变量
export CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER="lld-link"

# 构建 Windows 版本
echo "🔨 构建 Windows 版本..."
cd src-tauri
cargo tauri build --target x86_64-pc-windows-msvc

echo "✅ Windows 版本构建完成！"
echo "📦 输出位置: src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis/"
echo "📝 可执行文件: src-tauri/target/x86_64-pc-windows-msvc/release/ruziniu-tools.exe"