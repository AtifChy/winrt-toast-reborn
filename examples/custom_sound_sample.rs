use url::Url;

use winrt_toast_reborn::{Audio, Result, Text, Toast, ToastDuration, ToastManager};

fn main() -> Result<()> {
    let manager = ToastManager::new(ToastManager::POWERSHELL_AUM_ID);

    let mut toast = Toast::new();

    toast
        .tag("example")
        .text1("Title")
        .text2(Text::new("Body"))
        .duration(ToastDuration::Long)
        .audio(Audio::new_local(Url::from_file_path("path/to/sound/file").unwrap()).with_looping());

    manager.show(&toast).expect("Failed to show toast");

    Ok(())
}
