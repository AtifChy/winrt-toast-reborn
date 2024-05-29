use crate::hs;
use std::fmt::Debug;
use windows::Data::Xml::Dom::XmlElement;

/// An enum representing the sounds available.
#[derive(Debug, Clone)]
pub enum Sound {
    /// The default system sound.
    Default,
    /// A sound typically used for instant messages.
    IM,
    /// A sound typically used for incoming mail.
    Mail,
    /// A sound typically used for reminders.
    Reminder,
    /// A sound typically used for incoming SMS messages.
    SMS,
    /// Enable looping sound. See [`LoopingSound`] for the available sounds.
    Looping(LoopingSound),
    /// No sound.
    None,
}

impl Sound {
    fn as_str(&self) -> &'static str {
        match self {
            Sound::Default => "Default",
            Sound::IM => "IM",
            Sound::Mail => "Mail",
            Sound::Reminder => "Reminder",
            Sound::SMS => "SMS",
            Sound::Looping(s) => s.as_str(),
            Sound::None => "",
        }
    }
}

/// An enum representing the looping sounds available.
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum LoopingSound {
    Alarm,
    Alarm2,
    Alarm3,
    Alarm4,
    Alarm5,
    Alarm6,
    Alarm7,
    Alarm8,
    Alarm9,
    Alarm10,
    Call,
    Call2,
    Call3,
    Call4,
    Call5,
    Call6,
    Call7,
    Call8,
    Call9,
    Call10,
}

impl LoopingSound {
    fn as_str(&self) -> &'static str {
        match self {
            LoopingSound::Alarm => "Alarm",
            LoopingSound::Alarm2 => "Alarm2",
            LoopingSound::Alarm3 => "Alarm3",
            LoopingSound::Alarm4 => "Alarm4",
            LoopingSound::Alarm5 => "Alarm5",
            LoopingSound::Alarm6 => "Alarm6",
            LoopingSound::Alarm7 => "Alarm7",
            LoopingSound::Alarm8 => "Alarm8",
            LoopingSound::Alarm9 => "Alarm9",
            LoopingSound::Alarm10 => "Alarm10",
            LoopingSound::Call => "Call",
            LoopingSound::Call2 => "Call2",
            LoopingSound::Call3 => "Call3",
            LoopingSound::Call4 => "Call4",
            LoopingSound::Call5 => "Call5",
            LoopingSound::Call6 => "Call6",
            LoopingSound::Call7 => "Call7",
            LoopingSound::Call8 => "Call8",
            LoopingSound::Call9 => "Call9",
            LoopingSound::Call10 => "Call10",
        }
    }
}

/// Represents an audio element in a toast.
#[derive(Debug, Clone)]
pub struct Audio {
    src: Sound,
    loop_: bool,
    silent: bool,
}

impl Audio {
    /// Create a new audio element.
    pub fn new(src: Sound) -> Self {
        Self {
            src,
            loop_: false,
            silent: false,
        }
    }

    /// Set the audio to loop.
    pub fn with_looping(mut self) -> Self {
        self.loop_ = true;
        self
    }

    /// Set the audio to be silent.
    pub fn with_silent(mut self) -> Self {
        self.silent = true;
        self
    }

    pub(crate) fn write_to_element(&self, el: &XmlElement) -> crate::Result<()> {
        let mut silent = self.silent;
        match &self.src {
            Sound::None => silent = true,
            Sound::Looping(s) => {
                el.SetAttribute(
                    &hs("src"),
                    &hs(format!(
                        "ms-winsoundevent:Notification.Looping.{}",
                        s.as_str(),
                    )),
                )?;
            }
            _ => {
                el.SetAttribute(
                    &hs("src"),
                    &hs(format!(
                        "ms-winsoundevent:Notification.{}",
                        self.src.as_str(),
                    )),
                )?;
            }
        }
        el.SetAttribute(&hs("loop"), &hs(if self.loop_ { "true" } else { "false" }))?;
        el.SetAttribute(&hs("silent"), &hs(if silent { "true" } else { "false" }))?;

        Ok(())
    }
}
