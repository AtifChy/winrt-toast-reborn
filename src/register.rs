use std::path::Path;

use winreg::{enums::HKEY_CURRENT_USER, RegKey};

use crate::WinToastError;

/// Register the application to Windows registry.
///
/// `icon_path` should be an absolute path to the icon file, otherwise [`WinToastError::InvalidPath`] will be returned.
///
/// For more information on AUM_ID and registration, see this
/// [Windows documentation](https://docs.microsoft.com/en-us/windows/apps/design/shell/tiles-and-notifications/send-local-toast-desktop-cpp-wrl#step-5-register-with-notification-platform).
pub fn register(aum_id: &str, display_name: &str, icon_path: Option<&Path>) -> crate::Result<()> {
    if let Some(path) = icon_path {
        if !path.is_absolute() {
            return Err(WinToastError::InvalidPath);
        }
    }

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey(format!(r"SOFTWARE\Classes\AppUserModelId\{aum_id}"))?;

    key.set_value("DisplayName", &display_name)?;

    if let Some(path) = icon_path {
        key.set_value("IconUri", &path.to_string_lossy().to_string())?;
    } else {
        let _ = key.delete_value("IconUri");
    }

    Ok(())
}

/// Unregister the application from Windows registry.
///
/// Removes the registry key created by [`register`].
pub fn unregister(aum_id: &str) -> crate::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    hkcu.delete_subkey_all(format!(r"SOFTWARE\Classes\AppUserModelId\{aum_id}"))?;
    Ok(())
}
