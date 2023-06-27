use crate::UiPaint;
use std::borrow::Cow;

pub struct Page {
    pub name: &'static str,
    pub content: &'static str,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct HtmlContent<'p> {
    pub content: &'p str,
}

impl<'p> UiPaint for HtmlContent<'p> {
    fn to_html(&self) -> Cow<str> {
        Cow::Borrowed(self.content)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct HtmlStaticContent {
    pub content: &'static str,
}

impl UiPaint for HtmlStaticContent {
    fn to_html(&self) -> Cow<str> {
        Cow::Borrowed(self.content)
    }
}
