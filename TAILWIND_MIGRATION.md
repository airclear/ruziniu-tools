# Tailwind CSS Migration Notes

## 概述
本项目已成功迁移到使用 Tailwind CSS 进行样式设计，替换了原有的自定义CSS。

## 安装的依赖
- `tailwindcss@4.1.17`
- `@tailwindcss/postcss@4.1.17`
- `autoprefixer@10.4.22`
- `postcss@8.5.6`

## 配置文件
- `tailwind.config.js` - Tailwind CSS配置
- `postcss.config.js` - PostCSS配置
- `src/style.css` - 主CSS文件，包含Tailwind指令

## 主要改进

### 1. 响应式设计
- 使用Tailwind响应式前缀 (sm:, lg:, xl:)
- 移动优先的设计方法
- 灵活的布局系统

### 2. 暗色模式
- 使用Tailwind的 `dark:` 变体
- 自动检测系统暗色模式偏好
- 完整的颜色主题支持

### 3. 现代化UI组件
- 圆角边框 (rounded-lg, rounded-xl)
- 阴影效果 (shadow-lg)
- 渐变背景 (gradient-to-r)
- 过渡动画 (transition-all, duration-200)

### 4. 表单优化
- 统一的输入框样式
- 改进的焦点状态
- 更好的视觉反馈
- 禁用状态样式

### 5. 进度条增强
- 渐变进度条
- 平滑动画
- 居中文字显示

## 使用的Tailwind类示例

### 布局类
```css
min-h-screen bg-gray-50 dark:bg-gray-900 py-8 px-4 sm:px-6 lg:px-8
max-w-4xl mx-auto
flex justify-center items-center gap-8
```

### 表单类
```css
px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg
bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100
focus:ring-2 focus:ring-blue-500 focus:border-blue-500
```

### 按钮类
```css
px-6 py-2 bg-green-600 hover:bg-green-700 disabled:bg-gray-400
text-white rounded-lg font-medium transition-all duration-200
focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2
```

## 构建优化
- CSS文件大小: 6.96 kB (gzipped: 1.58 kB)
- 包含的Tailwind类都是实际使用的类
- 生产构建自动移除未使用的样式

## 功能保持
所有原有功能都完整保留：
- ✅ 批量短链输入
- ✅ 并发数量设置
- ✅ 实时进度显示
- ✅ CSV格式下载
- ✅ 暗色模式支持

## 启动应用
```bash
# 推荐方式（自动处理端口冲突）
./start.sh

# 或者直接启动
npm run tauri dev
```

## 构建应用
```bash
npm run build
```

## 已修复的问题
- ✅ 修复了Tauri 2.0事件监听器API兼容性问题
- ✅ 修复了 `window.__TAURI__.core` 未定义错误
- ✅ 正确导入和使用 `@tauri-apps/api/core` 和 `@tauri-apps/api/event`
- ✅ 添加了完善的错误处理，支持浏览器模式提示
- ✅ 创建了启动脚本避免端口冲突
- ✅ 使用现代的 `invoke()` 和 `listen()` API

## API使用方式
```javascript
// 正确的Tauri 2.0 API使用方式
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

// 调用命令
await invoke("command_name", { param: value });

// 监听事件
await listen("event-name", (event) => {
  console.log(event.payload);
});
```