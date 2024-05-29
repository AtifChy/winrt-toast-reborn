# winrt-toast

A mostly usable binding to the Windows `ToastNotification` API.

## Example

```rust
use winrt_toast::{Toast, Text, Header, ToastManager};
use winrt_toast::content::text::TextPlacement;

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