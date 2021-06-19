#[cfg_attr(unix, path = "linux_api.rs")]
#[cfg_attr(windows, path = "windows_api.rs")]
mod platform_api;
pub use self::platform_api::*;
