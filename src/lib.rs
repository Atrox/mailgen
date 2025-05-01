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
//! let theme = DefaultTheme::new()?;
//! let branding = Branding::new("test product", "https://testproduct.com");
//! let mailgen = Mailgen::new(theme, branding);
//!
//! let email = EmailBuilder::new()
//!     .summary("this is a test email that contains stuff to test...")
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
//! std::fs::write("./email-doctest.txt", rendered)?;
//!
//! let rendered = mailgen.render_html(&email)?;
//! std::fs::write("./email-doctest.html", rendered)?;
//!
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

mod builder;
mod email;
pub mod themes;

pub use builder::EmailBuilder;
pub use email::{Action, Email, GoToAction, Greeting, Table, TableColumns};
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
        let copyright = format!("Copyright © {name}. All rights reserved.");
        let trouble_text = "If you're having trouble with the button '{ACTION}', copy and paste the URL below into your web browser."
            .to_string();

        Self {
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
    use crate::{Action, Branding, Greeting, Mailgen, Table, TableColumns};

    #[test]
    #[cfg(feature = "default-theme")]
    fn test_default_theme() -> Result<(), Box<dyn std::error::Error>> {
        use crate::themes::DefaultTheme;

        let theme = DefaultTheme::new()?;
        let product = Branding::new("test product", "https://testproduct.com");
        let mailgen = Mailgen::new(theme, product);

        let email = EmailBuilder::new()
            .summary("this is a test email that contains stuff to test...")
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
        std::fs::write("./email.txt", rendered)?;

        let rendered = mailgen.render_html(&email)?;
        std::fs::write("./email.html", rendered)?;

        Ok(())
    }

    #[test]
    #[cfg(feature = "default-theme")]
    fn test_tables() -> Result<(), Box<dyn std::error::Error>> {
        use crate::themes::DefaultTheme;
        use std::collections::HashMap;

        let theme = DefaultTheme::new()?;
        let branding = Branding::new("test product", "https://testproduct.com");
        let mailgen = Mailgen::new(theme, branding);

        // Create table data
        let mut row1 = HashMap::new();
        row1.insert("Item", "Product 1");
        row1.insert("Price", "$10.99");
        row1.insert("Quantity", "1");

        let mut row2 = HashMap::new();
        row2.insert("Item", "Product 2");
        row2.insert("Price", "$24.99");
        row2.insert("Quantity", "2");

        let mut custom_alignment = HashMap::new();
        custom_alignment.insert("Price", "right");
        custom_alignment.insert("Quantity", "center");

        let columns = TableColumns {
            custom_width: None,
            custom_alignment: Some(custom_alignment),
        };

        let table = Table {
            title: "Order Summary",
            data: vec![row1, row2],
            columns: Some(columns),
        };

        let email = EmailBuilder::new()
            .summary("Order Confirmation")
            .greeting(Greeting::Name("Customer"))
            .intro("Thank you for your order!")
            .table(table)
            .outro("Your order will be processed soon.")
            .signature("The Sales Team")
            .build();

        let rendered = mailgen.render_html(&email)?;
        std::fs::write("./email_with_table.html", rendered)?;

        let rendered = mailgen.render_text(&email)?;
        std::fs::write("./email_with_table.txt", rendered)?;

        Ok(())
    }

    #[test]
    #[cfg(feature = "default-theme")]
    fn create_eml_file() -> Result<(), Box<dyn std::error::Error>> {
        use crate::themes::DefaultTheme;
        use std::collections::HashMap;
        use std::fs::File;
        use std::io::Write;

        let theme = DefaultTheme::new()?;
        let product = Branding::new("test product", "https://testproduct.com");
        let mailgen = Mailgen::new(theme, product);

        // Create table data
        let mut row1 = HashMap::new();
        row1.insert("Item", "Product 1");
        row1.insert("Price", "$10.99");
        row1.insert("Quantity", "1");

        let mut row2 = HashMap::new();
        row2.insert("Item", "Product 2");
        row2.insert("Price", "$24.99");
        row2.insert("Quantity", "2");

        let mut custom_alignment = HashMap::new();
        custom_alignment.insert("Price", "right");
        custom_alignment.insert("Quantity", "center");

        let columns = TableColumns {
            custom_width: None,
            custom_alignment: Some(custom_alignment),
        };

        let table = Table {
            title: "Order Summary",
            data: vec![row1, row2],
            columns: Some(columns),
        };

        let email = EmailBuilder::new()
            .summary("Email Test Subject")
            .greeting(Greeting::Name("Test User"))
            .intro("Welcome to our service!")
            .intro("We're excited to have you on board.")
            .dictionary("Account", "test@example.com")
            .dictionary("Plan", "Premium")
            .action(Action {
                text: "Confirm Account",
                link: "https://example.com/confirm",
                color: Some(("#48cfad", "#ffffff")),
                ..Default::default()
            })
            .table(table)
            .outro("Need help, or have questions?")
            .outro("Just reply to this email, we'd love to help.")
            .signature("The Example Team")
            .build();

        let text_content = mailgen.render_text(&email)?;
        let html_content = mailgen.render_html(&email)?;

        // Create a boundary for the multipart email
        let boundary = "------------MAILGEN_BOUNDARY";

        // Create an .eml file with proper headers and MIME structure
        let mut eml_file = File::create("./email_test.eml")?;

        // Write email headers
        writeln!(
            eml_file,
            "From: \"Test Product\" <no-reply@testproduct.com>"
        )?;
        writeln!(eml_file, "To: \"Test User\" <test@example.com>")?;
        writeln!(eml_file, "Subject: Email Test Subject")?;
        writeln!(eml_file, "MIME-Version: 1.0")?;
        writeln!(
            eml_file,
            "Content-Type: multipart/alternative; boundary=\"{boundary}\""
        )?;
        writeln!(eml_file)?;

        // Write the text part
        writeln!(eml_file, "--{boundary}")?;
        writeln!(eml_file, "Content-Type: text/plain; charset=UTF-8")?;
        writeln!(eml_file, "Content-Transfer-Encoding: 8bit")?;
        writeln!(eml_file)?;
        writeln!(eml_file, "{}", text_content)?;
        writeln!(eml_file)?;

        // Write the HTML part
        writeln!(eml_file, "--{boundary}")?;
        writeln!(eml_file, "Content-Type: text/html; charset=UTF-8")?;
        writeln!(eml_file, "Content-Transfer-Encoding: 8bit")?;
        writeln!(eml_file)?;
        writeln!(eml_file, "{}", html_content)?;
        writeln!(eml_file)?;

        // End the MIME message
        writeln!(eml_file, "--{boundary}--")?;

        Ok(())
    }

    #[test]
    #[cfg(feature = "default-theme")]
    fn test_go_to_action() -> Result<(), Box<dyn std::error::Error>> {
        use crate::themes::DefaultTheme;

        let theme = DefaultTheme::new()?;
        let branding = Branding::new("Test Company", "https://example.com");
        let mailgen = Mailgen::new(theme, branding);

        let email = EmailBuilder::new()
            .summary("Account Activation")
            .greeting(Greeting::Name("John Doe"))
            .intro("Welcome to our service! Please activate your account.")
            .action(Action {
                text: "Activate Account",
                link: "https://example.com/activate",
                instructions: Some("Click the button below to activate your account:"),
                ..Default::default()
            })
            .go_to_action(
                "Activate Now",
                "https://example.com/activate",
                "Activate your account with one click",
            )
            .outro("Need help? Just reply to this email.")
            .build();

        let rendered_html = mailgen.render_html(&email)?;

        // Verify JSON-LD script tag is present
        assert!(rendered_html.contains("application/ld+json"));
        assert!(rendered_html.contains("Activate your account with one click"));

        std::fs::write("./email_with_goto_action.html", rendered_html)?;

        let rendered_text = mailgen.render_text(&email)?;
        std::fs::write("./email_with_goto_action.txt", rendered_text)?;

        Ok(())
    }
}
