//! Native hosts retain GPUI for the lifetime of their platform callbacks.
//!
//! Call [`run`] from the native launch entrypoint. Initialize Kit in that
//! callback and create content with [`crate::open_window`]; it mounts one Root.

#[cfg(target_os = "android")]
pub use gpui_android as android;
#[cfg(target_os = "ios")]
pub use gpui_ios as ios;

#[cfg(target_os = "android")]
pub use gpui_android::{application, run, run_with};
#[cfg(target_os = "ios")]
pub use gpui_ios::{application, run, run_with};
