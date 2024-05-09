mod error;

use minijinja::Environment;
use serde::Serialize;

pub use self::error::Error;
use super::{TemplateContext, Theme};

static HTML: &str = include_str!("template.html");
static TEXT: &str = include_str!("template.text");

#[derive(Serialize)]
struct DefaultThemeContext<'a> {
    #[serde(flatten)]
    context: &'a TemplateContext<'a>,

    logo_max_height: u32,
}

#[derive(Debug, Clone)]
pub struct DefaultTheme {
    environment: Environment<'static>,

    pub logo_max_height: u32,
}

impl DefaultTheme {
    pub fn new() -> Result<Self, Error> {
        let mut environment = Environment::new();
        environment.add_template("html", HTML)?;
        environment.add_template("text", TEXT)?;

        Ok(Self {
            environment,

            logo_max_height: 50,
        })
    }

    fn render(&self, template: &str, context: &TemplateContext) -> Result<String, Error> {
        let context = DefaultThemeContext {
            context,

            logo_max_height: self.logo_max_height,
        };

        let rendered = self.environment.get_template(template)?.render(context)?;
        Ok(rendered)
    }
}

impl Theme for DefaultTheme {
    type Error = Error;

    fn html(&self, context: &TemplateContext) -> Result<String, Self::Error> {
        let html = self.render("html", context)?;
        let html = css_inline::inline(&html)?;

        Ok(html)
    }

    fn text(&self, context: &TemplateContext) -> Result<String, Self::Error> {
        let text = self.render("text", context)?;
        let text = html2text::from_read(text.as_bytes(), 80);

        Ok(text)
    }
}
