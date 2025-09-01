use std::path::Path;

#[allow(unused_imports)]
use winrt_toast_reborn::{register, unregister};

fn main() {
    let aum_id = "test.id";
    let display_name = "Test App";
    let icon_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("images")
        .join("cat.jpg");

    register(aum_id, display_name, Some(icon_path.as_path())).expect("Failed to register");

    // unregister(aum_id).expect("Failed to unregister");
}
