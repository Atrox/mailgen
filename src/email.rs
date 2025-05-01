use std::collections::HashMap;
use std::fmt::Display;

use serde::Serialize;

/// Email for rendering
#[derive(Debug, Clone, Serialize)]
pub struct Email<'a> {
    /// E-Mail summary, gets rendered in preview box on most email clients
    pub summary: Option<&'a str>,
    /// E-Mail greeting
    pub greeting: Option<Greeting<'a>>,
    /// Intro sentences, first displayed in the email
    pub intros: Option<Vec<&'a str>>,
    /// A list of key+value (useful for displaying parameters/settings/personal info)
    pub dictionary: Option<Vec<(&'a str, &'a str)>>,
    /// Table data to display in the email
    pub tables: Option<Vec<Table<'a>>>,
    /// Actions are a list of actions that the user will be able to execute via a button click
    pub actions: Option<Vec<Action<'a>>>,
    /// Outro sentences, last displayed in the email
    pub outros: Option<Vec<&'a str>>,
    /// Signature for the contacted person (default to 'Yours truly')
    pub signature: Option<&'a str>,
}

/// Column configuration for table
#[derive(Debug, Clone, Serialize)]
pub struct TableColumns<'a> {
    /// Custom width for specific columns
    pub custom_width: Option<HashMap<&'a str, &'a str>>,
    /// Custom alignment for specific columns
    pub custom_alignment: Option<HashMap<&'a str, &'a str>>,
}

/// Table data to display in the email
#[derive(Debug, Clone, Serialize)]
pub struct Table<'a> {
    /// Table title
    pub title: &'a str,
    /// Table data rows
    pub data: Vec<HashMap<&'a str, &'a str>>,
    /// Column configuration
    pub columns: Option<TableColumns<'a>>,
}

#[derive(Debug, Clone)]
pub enum Greeting<'a> {
    /// Displays a greeting by name. Renders as `Hey {name},`
    Name(&'a str),
    /// Custom greeting
    Custom(&'a str),
}

impl Display for Greeting<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Greeting::Name(name) => write!(f, "Hey {name},"),
            Greeting::Custom(custom) => f.write_str(custom),
        }
    }
}

impl Serialize for Greeting<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

/// Action is an action the user can do on the email (click on a button)
#[derive(Debug, Default, Clone, Serialize)]
pub struct Action<'a> {
    /// Button text
    pub text: &'a str,
    /// Button link
    pub link: &'a str,

    /// Text displayed before the button
    pub instructions: Option<&'a str>,
    /// Custom colors for the button in the format: (color, background-color)
    pub color: Option<(&'a str, &'a str)>,
}
