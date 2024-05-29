use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};
use winrt_toast::content::text::TextPlacement;
use winrt_toast::{Action, DismissalReason, Text, Toast, ToastDuration, ToastManager};

const AUM_ID: &str =
    "{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe";

fn main() {
    let manager = ToastManager::new(AUM_ID);

    let mut toast = Toast::new();

    toast
        .text1("Title")
        .text2(Text::new("Body"))
        .text3(Text::new("Via SMS").with_placement(TextPlacement::Attribution))
        .duration(ToastDuration::Short)
        .action(Action::new("Yes", "yes", ""))
        .action(Action::new("No", "no", ""));

    let action_take = Arc::new(AtomicBool::new(false));
    let action_clone = Arc::clone(&action_take);
    let dismiss_clone = Arc::clone(&action_take);

    manager
        .on_activated(move |action| {
            match action {
                Some(action) => println!("You've clicked {}!", action),
                None => println!("You've clicked me!"),
            }
            action_clone.store(true, Ordering::SeqCst);
        })
        .on_dismissed(move |reason| {
            match reason {
                Ok(DismissalReason::UserCanceled) => println!("UserCanceled"),
                Ok(DismissalReason::ApplicationHidden) => println!("ApplicationHidden"),
                Ok(DismissalReason::TimedOut) => println!("TimedOut"),
                Err(e) => eprintln!("Error: {:?}", e),
            }
            dismiss_clone.store(true, Ordering::SeqCst);
        })
        .show(&toast)
        .expect("Failed to show toast");

    let time_instant = Instant::now();
    while time_instant.elapsed() < Duration::from_secs(10) {
        if action_take.load(Ordering::SeqCst) {
            break;
        }
        sleep(Duration::from_millis(100));
    }
}
