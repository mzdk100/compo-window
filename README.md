# Compo Window - Cross-Platform Window Component for Compo

[中文文档](README-zh-CN.md)

---

Compo Window is a cross-platform window component library built on top of the [Compo](https://github.com/mzdk100/compo) declarative and reactive component framework. It provides native window creation and management capabilities for Windows, macOS, iOS, and Android platforms, enabling you to build native GUI applications with Compo's reactive programming model.

## Features

- **Cross-Platform Support**: Native window implementation for Windows, macOS, iOS, and Android
- **Reactive Window Management**: Declarative window properties with automatic updates
- **Native Performance**: Uses platform-specific APIs for optimal performance
- **Compo Integration**: Seamlessly integrates with Compo's component system
- **Zero Dependencies**: Minimal external dependencies, leveraging platform-native APIs
- **Type Safety**: Full Rust type safety with compile-time guarantees

## Platform Support

| Platform | Status | Implementation |
|----------|--------|----------------|
| Windows  | ✅ | Win32 API |
| macOS    | ✅ | Cocoa/AppKit |
| iOS      | ✅ | UIKit |
| Android  | ✅ | JNI + Android SDK |

## Quick Start

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
compo-window = "0.1.0"
```

### Basic Usage

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

    // Wait 2 seconds then update the window
    sleep(Duration::from_secs(2)).await;
    title = "Updated Title";
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

### Running the Example

```bash
# Desktop (Windows/macOS/Linux)
cd examples/desktop
cargo run

# Android
cd examples/android
./run.sh  # or run.bat on Windows

# iOS
cd examples/ios
# Open CompoWindow.xcodeproj in Xcode and run
```

## API Reference

### `window` Component

The main window component with the following parameters:

```rust
#[component]
pub async fn window(
    #[default = "Window"] title: &str,        // Window title
    width: i32,                               // Window width (platform-specific defaults)
    height: i32,                              // Window height (platform-specific defaults)
    #[default = CW_USEDEFAULT] left: i32,     // Window X position
    #[default = CW_USEDEFAULT] top: i32,      // Window Y position
    #[default = true] visible: bool,          // Window visibility
    #[default = true] enabled: bool,          // Window enabled state
)
```

#### Parameters

- **`title`**: The window title text (default: "Window")
- **`width`**: Window width in pixels (default: 800 on desktop, 360 on Android, 375 on iOS)
- **`height`**: Window height in pixels (default: 600 on desktop, 640 on Android, 667 on iOS)
- **`left`**: Window X position (default: system default)
- **`top`**: Window Y position (default: system default)
- **`visible`**: Whether the window is visible (default: true)
- **`enabled`**: Whether the window accepts user input (default: true)

#### Reactive Updates

All window properties are reactive - when you change a variable that's passed to the window component, the window will automatically update:

```rust
#[component]
async fn dynamic_window() {
    let mut title = "Initial Title";
    let mut width = 400;
    
    #[render]
    window { title, width };
    
    // Window will automatically update when these change
    title = "New Title";
    width = 800;
}
```

## Examples

### Basic Window

```rust
use compo_window::prelude::*;

#[component]
async fn basic_window() {
    #[render]
    window {
        title: "My Application",
        width: 1024,
        height: 768,
    };
}

fn main() {
    run(basic_window);
}
```

### Dynamic Window Updates

```rust
use compo_window::prelude::*;

#[component]
async fn animated_window() {
    let mut size = 400;
    
    #[render]
    window {
        title: "Animated Window",
        width: size,
        height: size,
    };
    
    // Animate window size
    for i in 0..10 {
        sleep(Duration::from_millis(500)).await;
        size += 50;
    }
}

fn main() {
    run(animated_window);
}
```

### Multiple Windows

```rust
use compo_window::prelude::*;

#[component]
async fn multi_window_app() {
    #[render]
    window {
        title: "Main Window",
        width: 800,
        height: 600,
        left: 100,
        top: 100,
    };
    
    #[render]
    window {
        title: "Secondary Window",
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

## Platform-Specific Details

### Default Window Sizes

The window component uses platform-appropriate default sizes:

- **Desktop (Windows/macOS)**: 800×600 pixels - suitable for desktop applications
- **Android**: 360×640 pixels - optimized for typical Android phone screens in portrait mode
- **iOS**: 375×667 pixels - optimized for iPhone screens in portrait mode (iPhone 6/7/8 size)

These defaults ensure that windows appear with appropriate sizes for each platform's typical use cases and screen orientations.

### Windows
- Uses Win32 API for native window creation
- Supports all standard Windows window features
- Integrates with Windows message loop

### macOS
- Uses Cocoa/AppKit for native window management
- Supports macOS-specific window behaviors
- Integrates with NSApplication lifecycle

### iOS
- Uses UIKit for iOS-native window creation
- Supports iOS app lifecycle integration
- Optimized for touch interfaces

### Android
- Uses JNI bridge to Android SDK
- Integrates with Android Activity lifecycle
- Supports Android-specific window features

## Building for Different Platforms

### Desktop
```bash
cargo build --release
```

### Android
Requires Android NDK and cargo-apk2:
```bash
cargo install cargo-apk2
cd examples/android
cargo apk2 build --release
```

### iOS
Requires Xcode and iOS toolchain:
```bash
cargo build --release --target aarch64-apple-ios
```

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

### Development Setup

1. Clone the repository
2. Install Rust and platform-specific toolchains
3. Run tests: `cargo test`
4. Run examples: `cargo run --example desktop`

## License

Apache-2.0

## Related Projects

- [Compo](https://github.com/mzdk100/compo) - The core declarative and reactive component framework
- [Compo Platform Loop](https://github.com/mzdk100/compo-platform-loop) - Cross-platform event loop implementation
- [cargo-apk2](https://github.com/mzdk100/cargo-apk2) - Tool for building Android applications with Cargo