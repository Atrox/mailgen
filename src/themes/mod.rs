#[cfg(feature = "default-theme")]
mod default;

#[cfg(feature = "default-theme")]
pub use default::DefaultTheme;
use serde::Serialize;

use crate::{Branding, Email};

pub trait Theme {
    type Error: std::error::Error;

    fn html(&self, context: &TemplateContext) -> Result<String, Self::Error>;
    fn text(&self, context: &TemplateContext) -> Result<String, Self::Error>;
}

#[derive(Serialize)]
pub struct TemplateContext<'a> {
    pub branding: &'a Branding,
    pub email: &'a Email<'a>,
}
