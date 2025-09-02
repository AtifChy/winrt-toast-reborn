use std::sync::mpsc;
use std::time::Duration;
use winrt_toast_reborn::{Result, Toast, ToastManager};

fn main() -> Result<()> {
    let manager = ToastManager::new(ToastManager::POWERSHELL_AUM_ID);

    let mut toast = Toast::new();

    toast
        .text1("Action Sample")
        .text2("This is a sample toast with actions.")
        .action(winrt_toast_reborn::Action::new("Action1", "action1", ""))
        .action(winrt_toast_reborn::Action::new("Action2", "action2", ""));

    let (tx, rx) = mpsc::channel::<()>();

    manager
        .on_activated(None, move |action| {
            match action {
                Some(act) => {
                    println!("Activated with action: {}", act.arg);
                }
                None => {
                    println!("Toast activated without action.");
                }
            }
            let _ = tx.send(());
        })
        .show(&toast)?;

    // We will wait up to 25 seconds for an action to be taken
    if rx.recv_timeout(Duration::from_secs(25)).is_ok() {
        println!("Action received.");
    } else {
        println!("No action received within timeout.");
    }

    Ok(())
}
