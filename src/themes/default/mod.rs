mod error;

use tera::Tera;

pub use self::error::Error;
use super::{TemplateContext, Theme};

static HTML: &str = include_str!("template.html");
static TEXT: &str = include_str!("template.text");

#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultTheme {}

impl DefaultTheme {
    pub fn new() -> Self {
        Self::default()
    }

    fn render(&self, template: &str, context: &TemplateContext) -> Result<String, Error> {
        let tera_context = tera::Context::from_serialize(context)?;
        let rendered = Tera::one_off(template, &tera_context, true)?;

        Ok(rendered)
    }
}

impl Theme for DefaultTheme {
    type Error = Error;

    fn html(&self, context: &TemplateContext) -> Result<String, Self::Error> {
        let html = self.render(HTML, context)?;
        let html = css_inline::inline(&html)?;

        Ok(html)
    }

    fn text(&self, context: &TemplateContext) -> Result<String, Self::Error> {
        let text = self.render(TEXT, context)?;
        let text = html2text::from_read(text.as_bytes(), 80);

        Ok(text)
    }
}
