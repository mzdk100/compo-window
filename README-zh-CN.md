# Compo Window - Compo 跨平台窗口组件

[English Documentation](README.md)

---

Compo Window 是基于 [Compo](https://github.com/mzdk100/compo) 声明式响应式组件框架构建的跨平台窗口组件库。它为 Windows、macOS、iOS 和 Android 平台提供原生窗口创建和管理功能，让你能够使用 Compo 的响应式编程模型构建原生 GUI 应用程序。

## 特性

- **跨平台支持**：为 Windows、macOS、iOS 和 Android 提供原生窗口实现
- **响应式窗口管理**：声明式窗口属性，支持自动更新
- **原生性能**：使用平台特定的 API 以获得最佳性能
- **Compo 集成**：与 Compo 组件系统无缝集成
- **零依赖**：最少的外部依赖，充分利用平台原生 API
- **类型安全**：完整的 Rust 类型安全，编译时保证

## 平台支持

| 平台 | 状态 | 实现方式 |
|------|------|----------|
| Windows  | ✅ | Win32 API |
| macOS    | ✅ | Cocoa/AppKit |
| iOS      | ✅ | UIKit |
| Android  | ✅ | JNI + Android SDK |

## 快速开始

### 安装

在你的 `Cargo.toml` 中添加：

```shell
cargo add compo-window
```

### 基本用法

```rust
use compo_window::prelude::*;

#[component]
async fn app() {
    let mut title = "Hello World";
    let mut width = 800;
    let mut height = 600;
    let mut enabled = true;

    #[render]
    window {
        title,
        width,
        height,
        enabled,
    };

    // 等待 2 秒后更新窗口
    sleep(Duration::from_secs(2)).await;
    title = "更新后的标题";
    width = 1000;
    height = 800;
}

fn main(#[cfg(target_os = "android")] vm: jni::JavaVM) {
    #[cfg(target_os = "android")]
    run(vm, app);
    #[cfg(not(target_os = "android"))]
    run(app);
}
```

### 运行示例

```bash
# 桌面端 (Windows/macOS/Linux)
cd examples/desktop
cargo run

# Android
cd examples/android
./run.sh  # Windows 上使用 run.bat

# iOS
cd examples/ios
# 在 Xcode 中打开 CompoWindow.xcodeproj 并运行
```

## API 参考

### `window` 组件

主要的窗口组件，包含以下参数：

```rust
#[component]
pub async fn window(
    #[default = "Window"] title: &str,        // 窗口标题
    width: i32,                               // 窗口宽度（平台特定默认值）
    height: i32,                              // 窗口高度（平台特定默认值）
    #[default = CW_USEDEFAULT] left: i32,     // 窗口 X 位置
    #[default = CW_USEDEFAULT] top: i32,      // 窗口 Y 位置
    #[default = true] visible: bool,          // 窗口可见性
    #[default = true] enabled: bool,          // 窗口启用状态
)
```

#### 参数说明

- **`title`**：窗口标题文本（默认："Window"）
- **`width`**：窗口宽度，单位像素（默认：桌面端 800，Android 360，iOS 375）
- **`height`**：窗口高度，单位像素（默认：桌面端 600，Android 640，iOS 667）
- **`left`**：窗口 X 位置（默认：系统默认值）
- **`top`**：窗口 Y 位置（默认：系统默认值）
- **`visible`**：窗口是否可见（默认：true）
- **`enabled`**：窗口是否接受用户输入（默认：true）

#### 响应式更新

所有窗口属性都是响应式的 - 当你更改传递给窗口组件的变量时，窗口会自动更新：

```rust
#[component]
async fn dynamic_window() {
    let mut title = "初始标题";
    let mut width = 400;
    
    #[render]
    window { title, width };
    
    // 当这些值改变时，窗口会自动更新
    title = "新标题";
    width = 800;
}
```

## 示例

### 基本窗口

```rust
use compo_window::prelude::*;

#[component]
async fn basic_window() {
    #[render]
    window {
        title: "我的应用程序",
        width: 1024,
        height: 768,
    };
}

fn main() {
    run(basic_window);
}
```

### 动态窗口更新

```rust
use compo_window::prelude::*;

#[component]
async fn animated_window() {
    let mut size = 400;
    
    #[render]
    window {
        title: "动画窗口",
        width: size,
        height: size,
    };
    
    // 动画窗口大小
    for i in 0..10 {
        sleep(Duration::from_millis(500)).await;
        size += 50;
    }
}

fn main() {
    run(animated_window);
}
```

### 多窗口应用

```rust
use compo_window::prelude::*;

#[component]
async fn multi_window_app() {
    #[render]
    window {
        title: "主窗口",
        width: 800,
        height: 600,
        left: 100,
        top: 100,
    };
    
    #[render]
    window {
        title: "辅助窗口",
        width: 400,
        height: 300,
        left: 200,
        top: 200,
    };
}

fn main() {
    run(multi_window_app);
}
```

## 平台特定详情

### 默认窗口尺寸

窗口组件使用适合各平台的默认尺寸：

- **桌面端 (Windows/macOS)**：800×600 像素 - 适合桌面应用程序
- **Android**：360×640 像素 - 针对典型 Android 手机竖屏模式优化
- **iOS**：375×667 像素 - 针对 iPhone 竖屏模式优化（iPhone 6/7/8 尺寸）

这些默认值确保窗口在各个平台上都能以适合的尺寸显示，符合各平台的典型使用场景和屏幕方向。

### Windows
- 使用 Win32 API 进行原生窗口创建
- 支持所有标准 Windows 窗口功能
- 与 Windows 消息循环集成

### macOS
- 使用 Cocoa/AppKit 进行原生窗口管理
- 支持 macOS 特定的窗口行为
- 与 NSApplication 生命周期集成

### iOS
- 使用 UIKit 进行 iOS 原生窗口创建
- 支持 iOS 应用生命周期集成
- 针对触摸界面进行优化

### Android
- 使用 JNI 桥接到 Android SDK
- 与 Android Activity 生命周期集成
- 支持 Android 特定的窗口功能

## 不同平台的构建

### 桌面端
```bash
cargo build --release
```

### Android
需要 Android NDK 和 cargo-apk2：
```bash
cargo install cargo-apk2
cd examples/android
cargo apk2 build --release
```

### iOS
需要 Xcode 和 iOS 工具链：
```bash
cargo build --release --target aarch64-apple-ios
```

## 贡献

欢迎贡献！请随时提交问题和拉取请求。

### 开发环境设置

1. 克隆仓库
2. 安装 Rust 和平台特定的工具链
3. 运行测试：`cargo test`
4. 运行示例：`cargo run --example desktop`

## 许可证

Apache-2.0

## 相关项目

- [Compo](https://github.com/mzdk100/compo) - 核心声明式响应式组件框架
- [Compo Platform Loop](https://github.com/mzdk100/compo-platform-loop) - 跨平台事件循环实现
- [cargo-apk2](https://github.com/mzdk100/cargo-apk2) - 使用 Cargo 构建 Android 应用程序的工具