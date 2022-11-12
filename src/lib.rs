//! # Overview
//! This crate allows you to generate pretty emails without all the hassle.
//! Take a look at the [README](https://github.com/atrox/mailgen) for screenshots.
//!
//! # Examples
//!
//! ```
//! use mailgen::themes::DefaultTheme;
//! use mailgen::{Action, Branding, EmailBuilder, Greeting, Mailgen};
//!
//! let theme = DefaultTheme::new();
//! let branding = Branding::new("test product", "https://testproduct.com");
//! let mailgen = Mailgen::new(&theme, branding);
//!
//! let email = EmailBuilder::default()
//!     .greeting(Greeting::Name("person name"))
//!     .intro("test intro")
//!     .intro("another intro")
//!     .dictionary("test key", "test value")
//!     .dictionary("test key 2", "test value 2")
//!     .action(Action {
//!         text: "Test Action",
//!         link: "https://test.com/action",
//!         color: Some(("black", "white")),
//!         ..Default::default()
//!     })
//!     .action(Action {
//!         text: "Test Action 2",
//!         link: "https://test.com/action2",
//!         instructions: Some("test instruction"),
//!         ..Default::default()
//!     })
//!     .outro("test outr 1")
//!     .outro("test outro 2")
//!     .signature("test signature...")
//!     .build();
//!
//! let rendered = mailgen.render_text(&email)?;
//! std::fs::write("./email.txt", &rendered)?;
//!
//! let rendered = mailgen.render_html(&email)?;
//! std::fs::write("./email.html", &rendered)?;
//!
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

mod builder;
mod email;
mod error;
pub mod themes;

pub use builder::EmailBuilder;
pub use email::{Action, Email, Greeting};
pub use error::Error;
use serde::{Deserialize, Serialize};
use tera::Tera;
use themes::Theme;

pub struct Mailgen {
    branding: Branding,

    // theme
    html_template: String,
    text_template: String,
}

/// Product represents your company product (brand)
/// Appears in header & footer of e-mails
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branding {
    name: String,
    link: String,
    logo: Option<String>,
    copyright: Option<String>,
    trouble_text: String,
}

impl Branding {
    pub fn new<S: Into<String>>(name: S, link: S) -> Self {
        let name = name.into();
        let link = link.into();
        let copyright = format!("Copyright © {}. All rights reserved.", name);
        let trouble_text = "If you’re having trouble with the button '{ACTION}', copy and paste \
                            the URL below into your web browser."
            .to_string();

        Branding {
            name,
            link,
            trouble_text,
            copyright: Some(copyright),
            logo: None,
        }
    }
}

#[derive(Serialize)]
struct TemplateContext<'a> {
    branding: &'a Branding,
    email: &'a Email<'a>,
}

impl Mailgen {
    pub fn new<'t>(theme: &impl Theme<'t>, branding: Branding) -> Self {
        let html_template = theme.html().to_string();
        let text_template = theme.text().to_string();

        Self {
            branding,

            html_template,
            text_template,
        }
    }

    pub fn render_html(&self, email: &Email) -> Result<String, Error> {
        let html = self.render(email, &self.html_template)?;
        let html = css_inline::inline(&html)?;
        Ok(html)
    }

    pub fn render_text(&self, email: &Email) -> Result<String, Error> {
        let html = self.render(email, &self.text_template)?;
        let html = html2text::from_read(html.as_bytes(), 80);
        Ok(html)
    }

    fn render(&self, email: &Email, template: &str) -> Result<String, Error> {
        let context = TemplateContext {
            email,
            branding: &self.branding,
        };

        let tera_context = tera::Context::from_serialize(&context)?;
        let html = Tera::one_off(template, &tera_context, true)?;

        Ok(html)
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::EmailBuilder;
    use crate::email::{Action, Greeting};
    use crate::themes::DefaultTheme;
    use crate::{Branding, Mailgen};

    #[test]
    fn test_general_usage() -> Result<(), Box<dyn std::error::Error>> {
        let theme = DefaultTheme::new();
        let product = Branding::new("test product", "https://testproduct.com");
        let mailgen = Mailgen::new(&theme, product);

        let email = EmailBuilder::default()
            .greeting(Greeting::Name("person name"))
            .intro("test intro")
            .intro("another intro")
            .dictionary("test key", "test value")
            .dictionary("test key 2", "test value 2")
            .action(Action {
                text: "Test Action",
                link: "https://test.com/action",
                color: Some(("black", "white")),
                ..Default::default()
            })
            .action(Action {
                text: "Test Action 2",
                link: "https://test.com/action2",
                instructions: Some("test instruction"),
                ..Default::default()
            })
            .outro("test outr 1")
            .outro("test outro 2")
            .signature("test signature...")
            .build();

        let rendered = mailgen.render_text(&email)?;
        std::fs::write("./email.txt", &rendered)?;

        let rendered = mailgen.render_html(&email)?;
        std::fs::write("./email.html", &rendered)?;

        Ok(())
    }
}
