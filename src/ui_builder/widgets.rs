use crate::{TagBuilder, UiPaint};
use std::borrow::Cow;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct Heading<'p> {
    pub size: HeadingSize,
    pub id: Option<&'p str>,
    pub class: Option<&'p str>,
    pub content: &'p str,
}

impl<'p> Heading<'p> {
    pub fn new(content: &'p str) -> Self {
        Heading {
            size: HeadingSize::default(),
            id: Option::default(),
            class: Option::default(),
            content,
        }
    }

    pub fn set_size(&mut self, size: HeadingSize) -> &mut Self {
        self.size = size;

        self
    }

    pub fn set_id(&mut self, id: &'p str) -> &mut Self {
        self.id = Some(id);

        self
    }

    pub fn set_class(&mut self, class: &'p str) -> &mut Self {
        self.id = Some(class);

        self
    }

    pub fn class(self) -> Option<&'p str> {
        self.class
    }

    pub fn id(self) -> Option<&'p str> {
        self.id
    }

    pub fn content(self) -> &'p str {
        self.content
    }
}

impl<'p> UiPaint for Heading<'p> {
    fn to_html(&self) -> Cow<str> {
        let tags = TagBuilder::new()
            .id(self.id)
            .class(self.class)
            .tag(self.size.html_tag())
            .build();

        tags.0 + self.content + tags.1
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum HeadingSize {
    #[default]
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl HeadingSize {
    fn html_tag(&self) -> &str {
        match self {
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::H5 => "h5",
            Self::H6 => "h6",
        }
    }
}

pub struct Input {}

pub struct TextArea {}

pub struct Radio {}

pub struct Checkbox {}

pub struct ComboBox {}

pub struct Paragraph {}

pub struct Slider {}

pub struct ProgressBar {}

pub struct Hyperlink {}

pub struct Table {}

pub struct DragArea {}

pub struct Plot {}

pub struct Canvas {}

pub struct UiWindow {}
