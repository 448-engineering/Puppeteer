use crate::{StyleDeclaration, Styling, UiPaint, DEFAULT_SVG_AS_BYTES};
use base64ct::{Base64, Encoding};
use core::fmt;
use std::borrow::Cow;

#[derive(Debug, Default)]
pub struct Container<'p> {
    id: Option<&'p str>,
    class: Vec<&'p str>,
    children: Vec<Box<dyn UiPaint>>,
    style: Option<Styling<'p>>,
}

impl<'p> Container<'p> {
    pub fn new() -> Self {
        Container::default()
    }

    pub fn add_class(mut self, class: &'p str) -> Self {
        self.class.push(class);

        self
    }

    pub fn add_class_borrowed(&mut self, class: &'p str) -> &mut Self {
        self.class.push(class);

        self
    }

    pub fn add_classes(mut self, classes: &[&'p str]) -> Self {
        classes.iter().for_each(|class| {
            self.class.push(class);
        });

        self
    }

    pub fn add_classes_borrowed(&mut self, classes: &[&'p str]) -> &mut Self {
        classes.iter().for_each(|class| {
            self.class.push(class);
        });

        self
    }

    pub fn add_id(mut self, id: &'p str) -> Self {
        self.id = Some(id);

        self
    }

    pub fn add_id_borrowed(&mut self, id: &'p str) -> &mut Self {
        self.id = Some(id);

        self
    }

    pub fn add_style(mut self, style: Styling<'p>) -> Self {
        self.style = Some(style);

        self
    }

    pub fn add_style_borrowed(&mut self, style: Styling<'p>) -> &mut Self {
        self.style = Some(style);

        self
    }

    pub fn add_child_borrowed(&mut self, child: impl UiPaint + 'static) -> &mut Self {
        self.children.push(Box::new(child));

        self
    }

    pub fn add_child(mut self, child: impl UiPaint + 'static) -> Self {
        self.children.push(Box::new(child));

        self
    }

    pub fn add_children(&mut self, children: Vec<impl UiPaint + 'static>) -> &mut Self {
        children.into_iter().for_each(|child| {
            self.children.push(Box::new(child));
        });

        self
    }

    pub fn replace_children(&mut self, children: Vec<Box<dyn UiPaint>>) -> &mut Self {
        self.children = children;

        self
    }

    pub fn id(&self) -> Option<&str> {
        self.id
    }

    pub fn class(&self) -> &[&str] {
        self.class.as_ref()
    }
}

impl<'p> UiPaint for Container<'p> {
    fn to_html(&self) -> Cow<str> {
        let mut opening_tag = Cow::Borrowed("<div");

        if let Some(id) = self.id {
            opening_tag += " id=\"";
            opening_tag += id;
            opening_tag += "\"";
        }

        if !self.class.is_empty() {
            opening_tag += " class=\"";

            self.class.iter().for_each(|class| {
                opening_tag += " ";
                opening_tag += *class;
            });

            opening_tag += " \"";
        }

        if let Some(style) = self.style.as_ref() {
            opening_tag += " style=\" ";
            opening_tag += style.to_css();
            opening_tag += "\"";
        }

        opening_tag += ">";

        if !self.children.is_empty() {
            self.children.iter().for_each(|child| {
                opening_tag += child.to_html();
            });
        }

        opening_tag + "</div>"
    }
}

pub struct Spacer {}

pub struct BottomSheet {}

pub struct SnackBar {}

pub struct Tab {}

pub struct Badges {}

pub struct AppBar {
    _location: String,
}

pub struct Dialog {}

pub struct Search {}

pub struct SideSheet {}

pub struct Switch {}

pub struct Pickers {} //DATE AND TIME

pub struct ToolTips {}

pub struct Accordion {}

pub struct Audio {}

pub struct Video {}

pub struct BorderBottomBar {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Image<'p> {
    pub content: &'p [u8],
    pub image_type: ImageType,
} //Image types - SVG, AVIF

impl<'p> StyleDeclaration for Image<'p> {
    fn to_css(&self) -> Cow<str> {
        let encoded = Base64::encode_string(self.content);

        self.image_type.to_css() + Cow::Owned(encoded)
    }
}

impl<'p> Default for Image<'p> {
    fn default() -> Self {
        Image {
            content: DEFAULT_SVG_AS_BYTES,
            image_type: ImageType::Svg,
        }
    }
}

impl<'p> fmt::Debug for Image<'p> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{:?} IMAGE => {}]",
            self.image_type,
            blake3::hash(self.content)
        )
    }
}

impl<'p> fmt::Display for Image<'p> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[IMAGE TYPE => {} - Blake3 Hash => {}]",
            self.image_type.as_str(),
            blake3::hash(self.content)
        )
    }
}

/// Types of images supported by the library
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum ImageType {
    /// Scalable Vector Graphics (SVG) is an XML-based vector image format for defining two-dimensional graphics, having support for interactivity and animation.
    /// [SVG Wikipedia](https://en.wikipedia.org/wiki/SVG)
    #[default]
    Svg,
    /// AV1 Image File Format (AVIF) is an open, royalty-free image file format specification for
    /// storing images or image sequences compressed with AV1 in the HEIF container format. [AVIF Wikipedia](https://en.wikipedia.org/wiki/AVIF)
    Avif,
    /// AV1 Image File Format (AVIF) sequence of images more similar to GIF images or MP4 files
    Avifs,
    /// Joint Photographic Experts Group is a commonly used method of lossy compression for digital images,
    /// particularly for those images produced by digital photography. [JPG Wikipedia](https://en.wikipedia.org/wiki/JPEG)
    Jpg,
    /// Portable Network Graphics is a raster-graphics file format that supports lossless data compression. [PNG Wikipedia](https://en.wikipedia.org/wiki/PNG)
    Png,
    /// WebP is an image file format developed by Google intended as a replacement for JPEG, PNG, and GIF file formats. It supports both lossy and lossless compression,
    /// as well as animation and alpha transparency. [WebP Wikipedia](https://en.wikipedia.org/wiki/WebP)
    Webp,
    /// The Graphics Interchange Format is a bitmap image format [GIF Wikipedia](https://en.wikipedia.org/wiki/GIF)
    Gif,
}

impl ImageType {
    /// The String representation of `ImageType`
    pub fn as_str(&self) -> &str {
        match self {
            Self::Svg => "svg",
            Self::Avif => "avif",
            Self::Avifs => "avifs",
            Self::Jpg => "jpg",
            Self::Png => "png",
            Self::Webp => "webp",
            Self::Gif => "gif",
        }
    }
}

impl StyleDeclaration for ImageType {
    fn to_css(&self) -> Cow<str> {
        let value = match self {
            Self::Svg => "svg+xml",
            Self::Avif => "avif",
            Self::Avifs => "avifs",
            Self::Jpg => "jpg",
            Self::Webp => "webp",
            Self::Gif => "gif",
            _ => todo!(), //TODO // FIXME
        };

        Cow::Borrowed("data:image/") + value + ";base64, "
    }
}

pub struct ClipBoard {}

pub struct Knob {}

pub struct Swipe {}

pub struct Pulse {}

pub struct PillVertical {}

pub struct Blur {}

pub struct Parallax {}

pub struct List {
    /*
    list_style
    list_style_image
    list_style_position
    list_style_type
    */
}
