#[derive(Debug, Default)]
pub struct TagBuilder<'p> {
    pub tag: &'p str,
    pub id: &'p str,
    pub class: &'p str,
    pub styles: Option<&'p str>,
}

impl<'p> TagBuilder<'p> {
    pub fn new() -> Self {
        TagBuilder::default()
    }

    pub fn tag(mut self, tag: &'p str) -> Self {
        self.tag = tag;

        self
    }

    pub fn id(mut self, id: &'p str) -> Self {
        self.id = id;

        self
    }

    pub fn class(mut self, class: &'p str) -> Self {
        self.class = class;

        self
    }

    pub fn styles(mut self, styles: Option<&'p str>) -> Self {
        self.styles = styles;

        self
    }

    /// Output tuple (Opening Tag, Closing Tag)
    pub fn build(self) -> (String, String) {
        let mut opening_tag = String::from("<");

        opening_tag = opening_tag + self.tag;
        if !self.id.is_empty() {
            opening_tag = opening_tag + " " + "id=\"" + self.id + "\"";
        }
        if !self.class.is_empty() {
            opening_tag = opening_tag + " " + "class=\"" + self.class + "\"";
        }

        if let Some(styles) = self.styles {
            opening_tag = opening_tag + styles;
        }

        opening_tag = opening_tag + ">";

        let closing_tag = "</".to_string() + self.tag + ">";

        (opening_tag, closing_tag)
    }
}

pub fn style_builder(values: &[(&str, String)]) -> Option<String> {
    if values.is_empty() {
        None
    } else {
        let styles = String::from("style=\"")
            + values
                .into_iter()
                .map(|value| value.0.to_string() + ": " + value.1.as_str() + " ")
                .collect::<String>()
                .as_str()
            + "\"";

        Some(styles)
    }
}

pub fn style_builder_prebuilt(values: &[&str]) -> Option<String> {
    if values.is_empty() {
        None
    } else {
        let styles = String::from("style=\"")
            + values
                .into_iter()
                .map(|value| value.to_string() + " ")
                .collect::<String>()
                .as_str()
            + "\"";

        Some(styles)
    }
}
