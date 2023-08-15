mod error;

use tera::Tera;

pub use self::error::Error;
use super::{TemplateContext, Theme};

static HTML: &str = include_str!("template.html");
static TEXT: &str = include_str!("template.text");

#[derive(Debug, Clone, Copy)]
pub struct DefaultTheme {
    pub logo_max_height: u32,
}

impl Default for DefaultTheme {
    fn default() -> Self {
        Self {
            logo_max_height: 50,
        }
    }
}

impl DefaultTheme {
    pub fn new() -> Self {
        Self::default()
    }

    fn render(&self, template: &str, context: &TemplateContext) -> Result<String, Error> {
        let mut tera_context = tera::Context::from_serialize(context)?;
        tera_context.insert("logo_max_height", &self.logo_max_height);

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
