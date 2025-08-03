use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};
use winrt_toast_reborn::content::audio::{LoopingSound, Sound};
use winrt_toast_reborn::content::image::{ImageHintCrop, ImagePlacement};
use winrt_toast_reborn::content::input::InputType;
use winrt_toast_reborn::content::text::TextPlacement;
use winrt_toast_reborn::{
    Action, ActivatedAction, Audio, DismissalReason, Image, Input, Result, Selection, Text, Toast,
    ToastDuration, ToastManager,
};

fn main() -> Result<()> {
    let manager = ToastManager::new(ToastManager::POWERSHELL_AUM_ID);

    let mut toast = Toast::new();

    let hero_image =
        Image::new_local(Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/images/flower.jpg"))?
            .with_placement(ImagePlacement::Hero);

    let icon_image =
        Image::new_local(Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/images/cat.jpg"))?
            .with_placement(ImagePlacement::AppLogoOverride)
            .with_hint_crop(ImageHintCrop::Circle);

    toast
        .tag("example")
        .text1("Title")
        .text2(Text::new("Body"))
        .text3(Text::new("Via SMS").with_placement(TextPlacement::Attribution))
        .image(1, hero_image)
        .image(2, icon_image)
        .duration(ToastDuration::Long)
        .audio(Audio::new(Sound::Looping(LoopingSound::Alarm5)).with_looping())
        .input(
            Input::new("box", InputType::Selection)
                .with_title("Select an option")
                .with_default_input("breakfast"),
        )
        .selection(Selection::new("breakfast", "Breakfast"))
        .selection(Selection::new("lunch", "Lunch"))
        .selection(Selection::new("dinner", "Dinner"))
        .action(Action::new("Send", "send", "").with_input_id("box"))
        .action(Action::new("Dismiss", "dismiss", ""));

    // Clone the action take atomic bool for the closures
    // This is necessary because the closures are Fn/FnMut,
    // and we need to be able to modify the action_take bool
    // from within the closures
    let action_take = Arc::new(AtomicBool::new(false));
    let action_clone = Arc::clone(&action_take);
    let dismiss_clone = Arc::clone(&action_take);

    fn handle_activated_action(action: Option<ActivatedAction>) {
        match action {
            Some(action) => {
                let message = format!(
                    "You clicked on {}{}!",
                    action.arg,
                    action
                        .values
                        .get(&action.input_id.unwrap_or_default())
                        .map_or(String::new(), |value| format!(", input = {value}"))
                );
                println!("{message}");
            }
            None => println!("You clicked me!"),
        }
    }

    manager
        .on_activated(Some("box"), move |action| {
            handle_activated_action(action);
            action_clone.store(true, Ordering::SeqCst);
        })
        .on_dismissed(move |reason| {
            match reason {
                Ok(r) if r.reason == DismissalReason::UserCanceled => println!("UserCanceled"),
                Ok(r) if r.reason == DismissalReason::ApplicationHidden => {
                    println!("ApplicationHidden")
                }
                Ok(r) if r.reason == DismissalReason::TimedOut => println!("TimedOut"),
                Ok(_r) => println!("Unknown dismissal reason"),
                Err(e) => eprintln!("Error: {e:?}"),
            }
            dismiss_clone.store(true, Ordering::SeqCst);
        })
        .on_failed(|e| eprintln!("Error: {e:?}"))
        .show(&toast)
        .expect("Failed to show toast");

    // Wait for the user to interact with the toast
    let time_instant = Instant::now();
    while time_instant.elapsed() < Duration::from_secs(25) {
        if action_take.load(Ordering::SeqCst) {
            break;
        }
        sleep(Duration::from_millis(100));
    }

    Ok(())
}
