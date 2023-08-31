use crate::StaticCowStr;

/// This provides the context menu builder
#[derive(Debug, PartialEq, Eq, Default)]
pub struct ContextMenu {
    /// The ID of the context menu target.
    /// This id will be used to listen for events in javascript
    pub id: &'static str,
    /// The HTML content for the context menu
    pub content: StaticCowStr,
    /// The Javascript to handle the context menu
    pub script: StaticCowStr,
    /// The appearance of the context menu
    pub styles: StaticCowStr,
}

impl ContextMenu {
    /// Initialize [ContextMenu] with  defaults
    pub fn new() -> Self {
        ContextMenu::default()
    }

    /// Add the HTML content for the context menu
    pub fn add_id(mut self, id: &'static str) -> Self {
        self.id = id;

        self
    }

    /// Add the HTML content for the context menu
    pub fn add_content(mut self, content: StaticCowStr) -> Self {
        self.content = content;

        self
    }

    /// Add the Javascript to handle the context menu
    pub fn add_script(mut self, script: StaticCowStr) -> Self {
        self.script = script;

        self
    }

    /// Add the appearance of the context menu
    pub fn add_style(mut self, styles: StaticCowStr) -> Self {
        self.styles = styles;

        self
    }

    /// Builds the context script by using the `self.id` as event listener target
    pub fn build_script(&self) -> StaticCowStr {
        StaticCowStr::Borrowed(
            "  
        <script>
            function hideCustomContextMenu() {
                document.getElementById('",
        ) + self.id
            + "').style.display = \"none\";
            }
            // toggling the menu on right click to the page
            function showCustomContextMenu(event) {
                event.preventDefault();
                var myContextMenu = document.getElementById('"
            + self.id
            + "');
                if (myContextMenu.style.display == \"block\") {
                myContextMenu.style.display = \"none\";
                }
                else {
                myContextMenu.style.display = \"block\";
                myContextMenu.style.left = event.pageX + \"px\";
                myContextMenu.style.top = event.pageY + \"px\";
                }
            }
            document.onclick = hideCustomContextMenu;
            document.oncontextmenu = showCustomContextMenu;
        </script>"
    }
}
