//! A mostly usable binding to the Windows `ToastNotification` API.
//!
//! # Basic Example
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
//! # Rich Toast with Images and Actions
//! ```no_run
//! use winrt_toast_reborn::{Toast, Text, Image, Action, ToastManager, ToastDuration};
//! use winrt_toast_reborn::content::image::{ImagePlacement, ImageHintCrop};
//! use std::path::Path;
//!
//! fn main() -> winrt_toast_reborn::Result<()> {
//!     let manager = ToastManager::new(ToastManager::POWERSHELL_AUM_ID);
//!
//!     // Create images
//!     let hero_image = Image::new_local(Path::new("hero.jpg"))?
//!         .with_placement(ImagePlacement::Hero);
//!
//!     let logo_image = Image::new_local(Path::new("logo.png"))?
//!         .with_placement(ImagePlacement::AppLogoOverride)
//!         .with_hint_crop(ImageHintCrop::Circle);
//!
//!     let mut toast = Toast::new();
//!     toast
//!         .text1("Meeting Reminder")
//!         .text2("Team standup starts in 5 minutes")
//!         .text3("Conference Room A")
//!         .image(1, hero_image)
//!         .image(2, logo_image)
//!         .duration(ToastDuration::Long)
//!         .action(Action::new("Join", "join_meeting", "meeting_id=123"))
//!         .action(Action::new("Snooze", "snooze", ""));
//!
//!     manager.show(&toast)?;
//!     Ok(())
//! }
//!```
//! # Interactive Toast with Input Fields
//!```no_run
//! use winrt_toast_reborn::{Toast, Input, Selection, Action, ToastManager};
//! use winrt_toast_reborn::content::input::InputType;
//!
//! fn main() -> winrt_toast_reborn::Result<()> {
//!     let manager = ToastManager::new(ToastManager::POWERSHELL_AUM_ID);
//!
//!     let mut toast = Toast::new();
//!     toast
//!         .text1("Quick Reply")
//!         .text2("Choose your response:")
//!         .input(
//!             Input::new("response", InputType::Selection)
//!                 .with_title("Select option")
//!                 .with_default_input("yes")
//!         )
//!         .selection(Selection::new("yes", "Yes"))
//!         .selection(Selection::new("no", "No"))
//!         .selection(Selection::new("maybe", "Maybe later"))
//!         .action(Action::new("Send", "send_response", "").with_input_id("response"));
//!
//!     manager.show(&toast)?;
//!     Ok(())
//! }
//!```

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
pub use manager::{ActivatedAction, DismissalReason, ToastDismissed, ToastFailed, ToastManager};

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
