#[cfg(target_os = "android")]
mod droid;
#[cfg(target_os = "ios")]
mod ios;
#[cfg(target_os = "macos")]
mod mac;
#[cfg(windows)]
mod win;

#[cfg(target_os = "android")]
pub use droid::*;
#[cfg(target_os = "ios")]
pub use ios::*;
#[cfg(target_os = "macos")]
pub use mac::*;
#[cfg(windows)]
pub use win::*;
