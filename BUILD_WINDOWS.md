# Windows 跨平台构建指南

## 🎯 方案一：GitHub Actions（推荐）

这是最简单、最可靠的方法，完全自动化。

### 使用步骤：

1. **完成开发** - 确保您的应用在本地正常运行
2. **更新版本号** - 在 `tauri.conf.json` 中更新版本号
3. **提交并推送代码**
   ```bash
   git add .
   git commit -m "准备发布 v0.1.0"
   git push origin main
   ```
4. **创建版本标签**
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```
5. **等待构建完成** - 访问 GitHub 仓库的 "Releases" 页面下载构建结果

### 输出文件：
- **Windows**: `.exe` 可执行文件 + `.msi` 安装包 + NSIS 安装程序
- **macOS**: `.app` 应用 + `.dmg` 安装包（支持 Intel 和 Apple Silicon）

**注意**: GitHub Actions 会在最新的 macOS runner 上构建，自动生成适用于 Intel 和 Apple Silicon Mac 的通用二进制文件。

## 🛠️ 方案二：本地交叉编译（实验性）

⚠️ **注意**: 这是实验性功能，可能会遇到依赖问题。

### 前置要求：

1. **安装 Windows 目标平台**
   ```bash
   rustup target add x86_64-pc-windows-msvc
   ```

2. **安装链接器（如果需要）**
   ```bash
   # macOS 使用 Homebrew
   brew install llvm

   # 或者安装 mingw-w64
   brew install mingw-w64
   ```

### 构建命令：

1. **快速构建**
   ```bash
   npm run build:windows
   ```

2. **完整构建（前端 + Windows）**
   ```bash
   npm run build:all
   ```

3. **手动构建**
   ```bash
   # 构建前端
   npm run build

   # 构建 Windows 版本
   cd src-tauri
   cargo tauri build --target x86_64-pc-windows-msvc
   ```

### 输出位置：
- **可执行文件**: `src-tauri/target/x86_64-pc-windows-msvc/release/ruziniu-tools.exe`
- **安装包**: `src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis/`

### 常见问题解决：

1. **链接器错误**
   ```bash
   export CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER="lld-link"
   ```

2. **依赖缺失**
   ```bash
   cargo clean
   cargo tauri build --target x86_64-pc-windows-msvc
   ```

## 📋 构建脚本说明

- `build-windows.sh`: 自动化交叉编译脚本
- 包含错误检查和详细的输出信息
- 自动清理之前的构建结果

## 🚀 推荐工作流

1. **开发阶段**: 使用 `npm run tauri dev` 进行本地开发
2. **测试构建**: 使用本地交叉编译验证构建流程
3. **正式发布**: 使用 GitHub Actions 进行自动化构建和发布

## 💡 提示

- 确保在 `tauri.conf.json` 中设置了正确的图标文件
- 版本号格式遵循语义化版本控制 (Semantic Versioning)
- GitHub Actions 会自动为每个平台生成适当的安装包格式