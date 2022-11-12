mod default;

pub use default::DefaultTheme;

pub trait Theme<'a> {
    fn html(&self) -> &'a str;
    fn text(&self) -> &'a str;
}
