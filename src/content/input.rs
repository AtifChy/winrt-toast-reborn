use crate::hs;
use windows::Data::Xml::Dom::XmlElement;

/// Specifies an input field used in the toast template.
#[derive(Debug, Clone)]
pub struct Input {
    id: String,
    type_: InputType,
    place_holder: Option<String>,
    title: Option<String>,
    default_input: Option<String>,
}

impl Input {
    /// Create a new input element.
    pub fn new(id: impl Into<String>, type_: InputType) -> Self {
        Self {
            id: id.into(),
            type_,
            place_holder: None,
            title: None,
            default_input: None,
        }
    }

    /// The placeholder content of the input.
    pub fn with_placeholder(mut self, content: impl Into<String>) -> Self {
        self.place_holder = Some(content.into());
        self
    }

    /// The title of the input.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// The default input of the input.
    pub fn with_default_input(mut self, default_input: impl Into<String>) -> Self {
        self.default_input = Some(default_input.into());
        self
    }

    pub(crate) fn write_to_element(&self, el: &XmlElement) -> crate::Result<()> {
        el.SetAttribute(&hs("id"), &hs(&self.id))?;
        el.SetAttribute(&hs("type"), &hs(self.type_.as_str()))?;
        if let Some(place_holder_content) = &self.place_holder {
            el.SetAttribute(&hs("placeHolderContent"), &hs(place_holder_content))?;
        }
        if let Some(title) = &self.title {
            el.SetAttribute(&hs("title"), &hs(title))?;
        }
        if let Some(default_input) = &self.default_input {
            el.SetAttribute(&hs("defaultInput"), &hs(default_input))?;
        }

        Ok(())
    }
}

/// The type of input field.
#[derive(Debug, Clone)]
pub enum InputType {
    /// A text input field.
    Text,
    /// A selection input field.
    Selection,
}

impl InputType {
    fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Selection => "selection",
        }
    }
}

/// Represents a selection in a selection input field.
#[derive(Debug, Clone)]
pub struct Selection {
    id: String,
    content: String,
}

impl Selection {
    /// Create a new selection.
    pub fn new(id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
        }
    }

    pub(crate) fn write_to_element(&self, el: &XmlElement) -> crate::Result<()> {
        el.SetAttribute(&hs("id"), &hs(&self.id))?;
        el.SetAttribute(&hs("content"), &hs(&self.content))?;

        Ok(())
    }
}
