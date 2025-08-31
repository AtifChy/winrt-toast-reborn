# winrt-toast-reborn

[![Crates.io](https://img.shields.io/crates/v/winrt-toast-reborn)](https://crates.io/crates/winrt-toast-reborn)
[![Docs.rs](https://docs.rs/winrt-toast-reborn/badge.svg)](https://docs.rs/winrt-toast-reborn)
[![License](https://img.shields.io/crates/l/winrt-toast-reborn)](LICENSE)

A mostly usable binding to the Windows `ToastNotification` API.

> **Note:** This project is a fork of [kdeconnect-rs/winrt-toast](https://github.com/kmod-midori/kdeconnect-rs/tree/main/winrt-toast).

## Basic Example

```rust
use winrt_toast_reborn::{Toast, Text, Header, ToastManager};
use winrt_toast_reborn::content::text::TextPlacement;

fn main() {
    let manager = ToastManager::new(ToastManager::POWERSHELL_AUM_ID);

    let mut toast = Toast::new();
    toast
        .text1("Title")
        .text2(Text::new("Body"))
        .text3(
            Text::new("Via SMS")
                .with_placement(TextPlacement::Attribution)
        );

    manager.show(&toast).expect("Failed to show toast");
}
```

## Advanced Examples

### Rich Toast with Images and Actions

```rust
use winrt_toast_reborn::{Toast, Text, Image, Action, ToastManager, ToastDuration};
use winrt_toast_reborn::content::image::{ImagePlacement, ImageHintCrop};
use std::path::Path;

fn main() -> winrt_toast_reborn::Result<()> {
    let manager = ToastManager::new(ToastManager::POWERSHELL_AUM_ID);

    // Create images
    let hero_image = Image::new_local(Path::new("hero.jpg"))?
        .with_placement(ImagePlacement::Hero);

    let logo_image = Image::new_local(Path::new("logo.png"))?
        .with_placement(ImagePlacement::AppLogoOverride)
        .with_hint_crop(ImageHintCrop::Circle);

    let mut toast = Toast::new();
    toast
        .text1("Meeting Reminder")
        .text2("Team standup starts in 5 minutes")
        .text3("Conference Room A")
        .image(1, hero_image)
        .image(2, logo_image)
        .duration(ToastDuration::Long)
        .action(Action::new("Join", "join_meeting", "meeting_id=123"))
        .action(Action::new("Snooze", "snooze", ""));

    manager.show(&toast)?;
    Ok(())
}
```

### Interactive Toast with Input Fields

```rust
use winrt_toast_reborn::{Toast, Input, Selection, Action, ToastManager};
use winrt_toast_reborn::content::input::InputType;

fn main() -> winrt_toast_reborn::Result<()> {
    let manager = ToastManager::new(ToastManager::POWERSHELL_AUM_ID);

    let mut toast = Toast::new();
    toast
        .text1("Quick Reply")
        .text2("Choose your response:")
        .input(
            Input::new("response", InputType::Selection)
                .with_title("Select option")
                .with_default_input("yes")
        )
        .selection(Selection::new("yes", "Yes"))
        .selection(Selection::new("no", "No"))
        .selection(Selection::new("maybe", "Maybe later"))
        .action(Action::new("Send", "send_response", "").with_input_id("response"));

    manager.show(&toast)?;
    Ok(())
}
```

### App Registration

```rust
use winrt_toast_reborn::register;
use std::path::Path;

fn main() -> winrt_toast_reborn::Result<()> {
    // Register your app with Windows
    register(
        "MyCompany.MyApp.SubApp", // Your unique App User Model ID
        "My Application",          // Display name
        Some(Path::new("C:\\path\\to\\icon.ico")), // Optional icon
    )?;

    // Now you can use your own AUM ID
    let manager = ToastManager::new("MyCompany.MyApp.SubApp");

    let toast = Toast::new()
        .text1("Registered App")
        .text2("This toast comes from your registered app!");

    manager.show(&toast)?;
    Ok(())
}
```

## Features

- [x] Rich toast content (text, images, actions)
- [x] Interactive elements (buttons, text input, selections)
- [x] Event handling (activation, dismissal, failure)
- [x] Audio support with various sounds
- [x] Different scenarios (alarm, reminder, incoming call, urgent)
- [x] Toast management (remove, clear by tag/group)
- [x] App registration for custom AUM IDs
- [ ] Adaptive content and data binding
- [ ] Groups and subgroups

## License

MIT License - see [LICENSE](LICENSE) file for details.
