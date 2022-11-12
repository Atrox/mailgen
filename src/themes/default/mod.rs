use super::Theme;

#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultTheme {}

impl DefaultTheme {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Theme<'static> for DefaultTheme {
    fn html(&self) -> &'static str {
        include_str!("template.html")
    }

    fn text(&self) -> &'static str {
        include_str!("template.text")
    }
}
