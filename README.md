# winrt-toast-reborn

[![Crates.io](https://img.shields.io/crates/v/winrt-toast-reborn)](https://crates.io/crates/winrt-toast-reborn)
[![Docs.rs](https://docs.rs/winrt-toast-reborn/badge.svg)](https://docs.rs/winrt-toast-reborn)
[![License](https://img.shields.io/crates/l/winrt-toast-reborn)](LICENSE)

A mostly usable binding to the Windows `ToastNotification` API.

## Example

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

## To-Do Features

* [x] Button style and tooltips in actions
* [x] Better callbacks
* [x] Sound
* [ ] Adaptive contents and data binding
* [ ] Groups and subgroups