use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("rendering error: {0}")]
    Tera(#[from] tera::Error),
    #[error("css inlining error: {0}")]
    InlineCSS(#[from] css_inline::InlineError),
}
