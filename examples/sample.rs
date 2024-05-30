use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};
use winrt_toast::content::audio::{Audio, LoopingSound, Sound};
use winrt_toast::content::image::{ImageHintCrop, ImagePlacement};
use winrt_toast::content::input::InputType;
use winrt_toast::content::text::TextPlacement;
use winrt_toast::{
    Action, DismissalReason, Image, Input, Result, Text, Toast, ToastDuration, ToastManager,
};

fn main() -> Result<()> {
    let manager = ToastManager::new(ToastManager::POWERSHELL_AUM_ID);

    let mut toast = Toast::new();

    let hero_image =
        Image::new_local(Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/flower.jpeg"))?
            .with_placement(ImagePlacement::Hero);

    let icon_image =
        Image::new_local(Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/chick.jpeg"))?
            .with_placement(ImagePlacement::AppLogoOverride)
            .with_hint_crop(ImageHintCrop::Circle);

    toast
        .text1("Title")
        .text2(Text::new("Body"))
        .text3(Text::new("Via SMS").with_placement(TextPlacement::Attribution))
        .image(1, hero_image)
        .image(2, icon_image)
        .duration(ToastDuration::Long)
        .audio(Audio::new(Sound::Looping(LoopingSound::Alarm5)).with_looping())
        .input(Input::new("box", InputType::Text).with_placeholder("Type here..."))
        .action(Action::new("Send", "send", "").with_input_id("box"));

    let action_take = Arc::new(AtomicBool::new(false));
    let action_clone = Arc::clone(&action_take);
    let dismiss_clone = Arc::clone(&action_take);

    manager
        .on_activated(
            move |action| {
                match action {
                    Some(action) => println!("You've clicked {}!", action),
                    None => println!("You've clicked me!"),
                }
                action_clone.store(true, Ordering::SeqCst);
            },
            Some("box"),
        )
        .on_dismissed(move |reason| {
            match reason {
                Ok(DismissalReason::UserCanceled) => println!("UserCanceled"),
                Ok(DismissalReason::ApplicationHidden) => println!("ApplicationHidden"),
                Ok(DismissalReason::TimedOut) => println!("TimedOut"),
                Err(e) => eprintln!("Error: {:?}", e),
            }
            dismiss_clone.store(true, Ordering::SeqCst);
        })
        .on_failed(|e| eprintln!("Error: {:?}", e))
        .show(&toast)
        .expect("Failed to show toast");

    let time_instant = Instant::now();
    while time_instant.elapsed() < Duration::from_secs(25) {
        if action_take.load(Ordering::SeqCst) {
            break;
        }
        sleep(Duration::from_millis(100));
    }

    Ok(())
}
