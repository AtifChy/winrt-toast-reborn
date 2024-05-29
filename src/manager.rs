use windows::{
    core::{IInspectable, Interface, HSTRING},
    Data::Xml::Dom::XmlDocument,
    Foundation::{DateTime, IReference, PropertyValue, TypedEventHandler},
    Globalization::Calendar,
    UI::Notifications::{
        ToastActivatedEventArgs, ToastDismissalReason, ToastDismissedEventArgs,
        ToastFailedEventArgs, ToastNotification, ToastNotificationManager,
    },
};

use crate::{hs, Result, Toast, WinToastError};

/// Specifies the reason that a toast notification is no longer being shown
///
/// See <https://docs.microsoft.com/en-us/uwp/api/windows.ui.notifications.toastdismissalreason>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DismissalReason {
    /// The user dismissed the toast notification.
    UserCanceled,
    /// The app explicitly hid the toast notification by calling the `ToastNotifier.hide` method.
    ApplicationHidden,
    /// The toast notification had been shown for the maximum allowed time and was faded out.
    /// The maximum time to show a toast notification is 7 seconds except in the case of long-duration toasts,
    /// in which case it is 25 seconds.
    TimedOut,
}

impl DismissalReason {
    fn from_winrt(reason: ToastDismissalReason) -> Result<Self> {
        match reason {
            ToastDismissalReason::UserCanceled => Ok(DismissalReason::UserCanceled),
            ToastDismissalReason::ApplicationHidden => Ok(DismissalReason::ApplicationHidden),
            ToastDismissalReason::TimedOut => Ok(DismissalReason::TimedOut),
            _ => Err(WinToastError::InvalidDismissalReason),
        }
    }
}

/// An interface that provides access to the toast notification manager.
///
/// This does not actually hold any Windows resource, but is used to
/// store the Application User Model ID (AUM_ID)  that is required to access the toast notification manager.
///
/// You may register your own AUM_ID with this crate's `register` function, or
/// use any method described in the [Windows documentation](https://docs.microsoft.com/en-us/windows/apps/design/shell/tiles-and-notifications/send-local-toast-desktop-cpp-wrl#step-5-register-with-notification-platform).
///
/// Alternatively, you may use `{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\WindowsPowerShell\v1.0\powershell.exe` as an experimental AUM_ID.
#[derive(Clone)]
pub struct ToastManager {
    app_id: HSTRING,
    on_activated: Option<TypedEventHandler<ToastNotification, IInspectable>>,
    on_dismissed: Option<TypedEventHandler<ToastNotification, ToastDismissedEventArgs>>,
    on_failed: Option<TypedEventHandler<ToastNotification, ToastFailedEventArgs>>,
}

impl std::fmt::Debug for ToastManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ToastManager({})", self.app_id)
    }
}

impl ToastManager {
    /// Create a new manager with
    pub fn new(aum_id: impl AsRef<str>) -> Self {
        Self {
            app_id: hs(aum_id.as_ref()),
            on_activated: None,
            on_dismissed: None,
            on_failed: None,
        }
    }

    /// Remove all notifications in `group`.
    pub fn remove_group(&self, group: &str) -> Result<()> {
        let history = ToastNotificationManager::History()?;

        history.RemoveGroupWithId(&hs(group), &self.app_id)?;

        Ok(())
    }

    /// Remove a notification in `group` with `tag`.
    pub fn remove_grouped_tag(&self, group: &str, tag: &str) -> Result<()> {
        let history = ToastNotificationManager::History()?;

        history.RemoveGroupedTagWithId(&hs(tag), &hs(group), &self.app_id)?;

        Ok(())
    }

    /// Remove a notification with the specified `tag`.
    pub fn remove(&self, tag: &str) -> Result<()> {
        let history = ToastNotificationManager::History()?;

        history.Remove(&hs(tag))?;

        Ok(())
    }

    /// Clear all toast notifications from this application.
    pub fn clear(&self) -> Result<()> {
        let history = ToastNotificationManager::History()?;

        history.ClearWithId(&self.app_id)?;

        Ok(())
    }

    /// Register a callback for when a toast notification is activated.
    pub fn on_activated<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(Option<String>) + Send + 'static,
    {
        self.on_activated = Some(TypedEventHandler::new(
            move |_, args: &Option<IInspectable>| {
                f(Self::get_activated_action(args));
                Ok(())
            },
        ));
        self
    }

    fn get_activated_action(inspect: &Option<IInspectable>) -> Option<String> {
        let args = inspect
            .as_ref()
            .and_then(|arg| arg.cast::<ToastActivatedEventArgs>().ok());

        args.and_then(|args| {
            args.Arguments().map(|s| s.to_string()).ok().and_then(|s| {
                if !s.is_empty() {
                    Some(s)
                } else {
                    None
                }
            })
        })
    }

    /// Register a callback for when a toast notification is dismissed.
    pub fn on_dismissed<F>(mut self, f: F) -> Self
    where
        F: Fn(Result<DismissalReason>) + Send + 'static,
    {
        self.on_dismissed = Some(TypedEventHandler::new(
            move |_, args: &Option<ToastDismissedEventArgs>| {
                f(Self::get_dismissal_reason(args));
                Ok(())
            },
        ));
        self
    }

    fn get_dismissal_reason(args: &Option<ToastDismissedEventArgs>) -> Result<DismissalReason> {
        let args = args.as_ref().and_then(|arg| arg.Reason().ok());
        match args {
            Some(reason) => DismissalReason::from_winrt(reason),
            None => Err(WinToastError::InvalidDismissalReason),
        }
    }

    /// Register a callback for when a toast notification fails to display.
    pub fn on_failed<F>(mut self, f: F) -> Self
    where
        F: Fn(WinToastError) + Send + 'static,
    {
        self.on_failed = Some(TypedEventHandler::new(
            move |_, args: &Option<ToastFailedEventArgs>| {
                f(Self::get_failed_error(args));
                Ok(())
            },
        ));
        self
    }

    fn get_failed_error(args: &Option<ToastFailedEventArgs>) -> WinToastError {
        let err = args.as_ref().and_then(|e| e.ErrorCode().ok());
        if let Some(e) = err {
            if e.is_err() {
                return WinToastError::Os(e.into());
            }
        }
        WinToastError::Unknown
    }

    /// Send a toast to Windows for display.
    pub fn show_with_callbacks(
        &self,
        toast: &Toast,
        // on_failed: Option<Box<dyn FnMut(WinToastError) + Send + 'static>>,
    ) -> Result<()> {
        let notifier = ToastNotificationManager::CreateToastNotifierWithId(&self.app_id)?;

        let toast_doc = XmlDocument::new()?;

        let toast_el = toast_doc.CreateElement(&hs("toast"))?;
        toast_doc.AppendChild(&toast_el)?;

        if let Some(scenario) = &toast.scenario {
            toast_el.SetAttribute(&hs("scenario"), &hs(scenario.as_str()))?;
        }

        if let Some(launch) = &toast.launch {
            toast_el.SetAttribute(&hs("launch"), &hs(launch))?;
        }

        if let Some(duration) = &toast.duration {
            toast_el.SetAttribute(&hs("duration"), &hs(duration.as_str()))?;
        }

        // <header>
        if let Some(header) = &toast.header {
            let el = toast_doc.CreateElement(&hs("header"))?;
            toast_el.AppendChild(&el)?;
            header.write_to_element(&el)?;
        }
        // </header>
        // <visual>
        {
            let visual_el = toast_doc.CreateElement(&hs("visual"))?;
            toast_el.AppendChild(&visual_el)?;
            // <binding>
            {
                let binding_el = toast_doc.CreateElement(&hs("binding"))?;
                visual_el.AppendChild(&binding_el)?;
                binding_el.SetAttribute(&hs("template"), &hs("ToastGeneric"))?;
                {
                    if let Some(text) = &toast.text.0 {
                        let el = toast_doc.CreateElement(&hs("text"))?;
                        binding_el.AppendChild(&el)?;
                        text.write_to_element(1, &el)?;
                    }
                    if let Some(text) = &toast.text.1 {
                        let el = toast_doc.CreateElement(&hs("text"))?;
                        binding_el.AppendChild(&el)?;
                        text.write_to_element(2, &el)?;
                    }
                    if let Some(text) = &toast.text.2 {
                        let el = toast_doc.CreateElement(&hs("text"))?;
                        binding_el.AppendChild(&el)?;
                        text.write_to_element(3, &el)?;
                    }

                    for (id, image) in &toast.images {
                        let el = toast_doc.CreateElement(&hs("image"))?;
                        binding_el.AppendChild(&el)?;
                        image.write_to_element(*id, &el)?;
                    }
                }
            }
            // </binding>
        }
        // </visual>
        // <actions>
        if !toast.actions.is_empty() {
            let actions_el = toast_doc.CreateElement(&hs("actions"))?;
            toast_el.AppendChild(&actions_el)?;
            for action in &toast.actions {
                let el = toast_doc.CreateElement(&hs("action"))?;
                actions_el.AppendChild(&el)?;
                action.write_to_element(&el)?;
            }
        }
        // </actions>

        let toast_notifier = ToastNotification::CreateToastNotification(&toast_doc)?;

        if let Some(group) = &toast.group {
            toast_notifier.SetGroup(&hs(group))?;
        }
        if let Some(tag) = &toast.tag {
            toast_notifier.SetTag(&hs(tag))?;
        }
        if let Some(remote_id) = &toast.remote_id {
            toast_notifier.SetRemoteId(&hs(remote_id))?;
        }
        if let Some(exp) = toast.expires_in {
            let now = Calendar::new()?;
            now.AddSeconds(exp.as_secs() as i32)?;
            let dt = now.GetDateTime()?;
            toast_notifier.SetExpirationTime(
                &PropertyValue::CreateDateTime(dt)?.cast::<IReference<DateTime>>()?,
            )?;
        }

        if let Some(handler) = &self.on_activated {
            toast_notifier.Activated(handler)?;
        }

        if let Some(handler) = &self.on_dismissed {
            toast_notifier.Dismissed(handler)?;
        }

        if let Some(handler) = &self.on_failed {
            toast_notifier.Failed(handler)?;
            /*
            toast_notifier.Failed(&TypedEventHandler::new(
                move |_, args: &Option<ToastFailedEventArgs>| {
                    if let Some(args) = args {
                        let e = args.ErrorCode().and_then(|e| e.ok());
                        if let Err(e) = e {
                            failed(e.into())
                        }
                    }
                    Ok(())
                },
            ))?;
            */
        }

        notifier.Show(&toast_notifier)?;

        Ok(())
    }

    /// Send a toast to Windows for display without any callbacks.
    pub fn show(&self, in_toast: &Toast) -> Result<()> {
        self.show_with_callbacks(in_toast)
    }
}
