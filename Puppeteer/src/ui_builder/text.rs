use hex_color::HexColor;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TextStyle<'p> {
    font: &'p str,
    font_family: &'p str,
    line_height: u16,
    letter_spacing: i16,
    align: TextAlign,
    text_decoration: TextDecoration,
    decoration_color: HexColor,
    ident: (bool, usize), //TODO Is the ident specified as percentage or pixels
    justify: TextJustify,
    overflow: TextOverflow<'p>,
    // shadow: //TODO
    transform_text: TextTransform,
    word_wrap: bool, // false for `normal` and true for `break-word` in CSS
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TextTransform {
    Capitalize,
    Lowercase,
    None,
    Uppercase,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TextAlign {
    Left,
    Right,
    Center,
    Justify,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TextOverflow<'p> {
    Clip,
    Ellipsis,
    Custom(&'p str),
    Justify,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TextJustify {
    Auto,
    None,
    InterWord,
    Distribute,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TextDecoration {
    None,
    Underline,
    Overline,
    LineThrough,
    Blink,
}
