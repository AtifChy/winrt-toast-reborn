//! A mostly usable binding to the Windows `ToastNotification` API.
//!
//! # Example
//! ```no_run
//! use winrt_toast_reborn::{Toast, Text, Header, ToastManager};
//! use winrt_toast_reborn::content::text::TextPlacement;
//!
//! let manager = ToastManager::new(ToastManager::POWERSHELL_AUM_ID);
//!
//! let mut toast = Toast::new();
//! toast
//!     .text1("Title")
//!     .text2(Text::new("Body"))
//!     .text3(
//!         Text::new("Via SMS")
//!             .with_placement(TextPlacement::Attribution)
//!     );
//!
//! manager.show(&toast).expect("Failed to show toast");
//! ```

#![warn(missing_docs)]

/// Contents in a toast notification.
pub mod content;

pub use content::action::Action;
pub use content::audio::Audio;
pub use content::header::Header;
pub use content::image::Image;
pub use content::input::Input;
pub use content::input::Selection;
pub use content::text::Text;
use thiserror::Error;

mod manager;
pub use manager::{ActivatedAction, DismissalReason, ToastManager};

mod toast;
pub use toast::{Scenario, Toast, ToastDuration};

mod register;
pub use register::register;

/// Re-export of the `url` crate.
pub use url;
use windows::core::HSTRING;

/// Convert a string to a HSTRING
pub(crate) fn hs(s: impl AsRef<str>) -> HSTRING {
    let s = s.as_ref();
    HSTRING::from(s)
}

/// The error type used in this crate.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum WinToastError {
    /// External error from the Windows API.
    #[error("Windows API error: {0}")]
    Os(#[from] windows::core::Error),
    /// Error from the Windows Runtime.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// The given path is not absolute, and therefore cannot be converted to a URL.
    #[error("The given path is not absolute")]
    InvalidPath,
    /// The dismissal reason from OS is unknown
    #[error("The dismissal reason from OS is unknown")]
    InvalidDismissalReason,
    /// The toast is not initialized properly.
    #[error("Unknown error")]
    Unknown,
}

/// The result type used in this crate.
pub type Result<T> = std::result::Result<T, WinToastError>;
