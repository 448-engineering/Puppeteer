use crate::{TagBuilder, UiPaint};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Heading<'p> {
    pub size: HeadingSize,
    pub id: &'p str,
    pub class: &'p str,
    pub content: &'p str,
}

impl<'p> UiPaint for Heading<'p> {
    fn to_html(&self) -> String {
        let tags = TagBuilder::new()
            .id(self.id)
            .class(self.class)
            .tag(self.size.html_tag())
            .build();

        tags.0 + self.content + tags.1.as_str()
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
