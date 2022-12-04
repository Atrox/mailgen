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
//! let mailgen = Mailgen::new(theme, branding);
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
//! std::fs::write("./email-doctest.txt", &rendered)?;
//!
//! let rendered = mailgen.render_html(&email)?;
//! std::fs::write("./email-doctest.html", &rendered)?;
//!
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

mod builder;
mod email;
pub mod themes;

pub use builder::EmailBuilder;
pub use email::{Action, Email, Greeting};
use serde::{Deserialize, Serialize};
use themes::{TemplateContext, Theme};

pub struct Mailgen<T: Theme> {
    theme: T,
    branding: Branding,
}

impl<T: Theme> Mailgen<T> {
    pub fn new(theme: T, branding: Branding) -> Self {
        Self { theme, branding }
    }

    pub fn render_html(&self, email: &Email) -> Result<String, T::Error> {
        let context = TemplateContext {
            email,
            branding: &self.branding,
        };

        self.theme.html(&context)
    }

    pub fn render_text(&self, email: &Email) -> Result<String, T::Error> {
        let context = TemplateContext {
            email,
            branding: &self.branding,
        };

        self.theme.text(&context)
    }
}

/// Product represents your company product (brand)
/// Appears in header & footer of e-mails
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branding {
    pub name: String,
    pub link: String,
    pub logo: Option<String>,
    pub copyright: Option<String>,
    pub trouble_text: String,
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

#[cfg(test)]
mod tests {
    use crate::builder::EmailBuilder;
    use crate::email::{Action, Greeting};
    use crate::{Branding, Mailgen};

    #[test]
    #[cfg(feature = "default-theme")]
    fn test_default_theme() -> Result<(), Box<dyn std::error::Error>> {
        use crate::themes::DefaultTheme;

        let theme = DefaultTheme::new();
        let product = Branding::new("test product", "https://testproduct.com");
        let mailgen = Mailgen::new(theme, product);

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
