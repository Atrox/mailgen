use crate::email::{Action, Email, Greeting, Table};

/// Email Builder
///
/// ```
/// use mailgen::{Action, EmailBuilder, Greeting};
///
/// let email = EmailBuilder::new()
///     .greeting(Greeting::Name("person name"))
///     .intro("test intro")
///     .intro("another intro")
///     .dictionary("test key", "test value")
///     .dictionary("test key 2", "test value 2")
///     .action(Action {
///         text: "Test Action",
///         link: "https://test.com/action",
///         color: Some(("black", "white")),
///         ..Default::default()
///     })
///     .action(Action {
///         text: "Test Action 2",
///         link: "https://test.com/action2",
///         instructions: Some("test instruction"),
///         ..Default::default()
///     })
///     .outro("test outr 1")
///     .outro("test outro 2")
///     .signature("test signature...")
///     .build();
/// ```
#[derive(Clone, Default)]
pub struct EmailBuilder<'a> {
    summary: Option<&'a str>,
    greeting: Option<Greeting<'a>>,
    intros: Option<Vec<&'a str>>,
    dictionary: Option<Vec<(&'a str, &'a str)>>,
    tables: Option<Vec<Table<'a>>>,
    actions: Option<Vec<Action<'a>>>,
    outros: Option<Vec<&'a str>>,
    signature: Option<&'a str>,
}

impl<'a> EmailBuilder<'a> {
    /// New email builder with sane defaults
    #[must_use]
    pub fn new() -> Self {
        Self {
            greeting: Some(Greeting::Custom("Hey")),
            signature: Some("Yours truly,"),

            ..Default::default()
        }
    }

    /// E-Mail summary, gets rendered in preview box on most email clients
    #[must_use]
    pub fn summary(mut self, v: &'a str) -> Self {
        self.summary = Some(v);
        self
    }

    /// E-Mail greeting
    #[must_use]
    pub fn greeting(mut self, v: Greeting<'a>) -> Self {
        self.greeting = Some(v);
        self
    }

    /// Intro sentences, first displayed in the email
    #[must_use]
    pub fn intro(mut self, v: &'a str) -> Self {
        match &mut self.intros {
            Some(intros) => intros.push(v),
            None => self.intros = Some(vec![v]),
        }
        self
    }

    /// Intro sentences, first displayed in the email
    #[must_use]
    pub fn set_intros(mut self, intros: Vec<&'a str>) -> Self {
        self.intros = Some(intros);
        self
    }

    /// A list of key+value (useful for displaying parameters/settings/personal info)
    #[must_use]
    pub fn dictionary(mut self, key: &'a str, value: &'a str) -> Self {
        match &mut self.dictionary {
            Some(dictionary) => dictionary.push((key, value)),
            None => self.dictionary = Some(vec![(key, value)]),
        }
        self
    }

    /// A list of key+value (useful for displaying parameters/settings/personal info)
    #[must_use]
    pub fn set_dictionary(mut self, dictionary: Vec<(&'a str, &'a str)>) -> Self {
        self.dictionary = Some(dictionary);
        self
    }

    /// Actions are a list of actions that the user will be able to execute via a button click
    #[must_use]
    pub fn action(mut self, action: Action<'a>) -> Self {
        match &mut self.actions {
            Some(actions) => actions.push(action),
            None => self.actions = Some(vec![action]),
        }
        self
    }

    /// Actions are a list of actions that the user will be able to execute via a button click
    #[must_use]
    pub fn set_actions(mut self, actions: Vec<Action<'a>>) -> Self {
        self.actions = Some(actions);
        self
    }

    /// Outro sentences, last displayed in the email
    #[must_use]
    pub fn outro(mut self, outro: &'a str) -> Self {
        match &mut self.outros {
            Some(outros) => outros.push(outro),
            None => self.outros = Some(vec![outro]),
        }
        self
    }

    /// Outro sentences, last displayed in the email
    #[must_use]
    pub fn set_outros(mut self, outros: Vec<&'a str>) -> Self {
        self.outros = Some(outros);
        self
    }

    /// Signature for the contacted person (default to 'Yours truly')
    #[must_use]
    pub fn signature(mut self, signature: &'a str) -> Self {
        self.signature = Some(signature);
        self
    }

    /// Add a table to the email
    #[must_use]
    pub fn table(mut self, table: Table<'a>) -> Self {
        match &mut self.tables {
            Some(tables) => tables.push(table),
            None => self.tables = Some(vec![table]),
        }
        self
    }

    /// Set all tables for the email
    #[must_use]
    pub fn set_tables(mut self, tables: Vec<Table<'a>>) -> Self {
        self.tables = Some(tables);
        self
    }

    /// Build the email
    #[must_use]
    pub fn build(self) -> Email<'a> {
        Email {
            summary: self.summary,
            greeting: self.greeting,
            intros: self.intros,
            dictionary: self.dictionary,
            tables: self.tables,
            actions: self.actions,
            outros: self.outros,
            signature: self.signature,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::email::Greeting;
    use crate::EmailBuilder;

    #[test]
    fn usage() {
        let _email = EmailBuilder::new()
            .greeting(Greeting::Custom("custom greeting"))
            .intro("test")
            .build();

        let mut email = EmailBuilder::new()
            .intro("test intro")
            .dictionary("test key", "test value")
            .dictionary("test key 2", "test value 2");

        {
            let greeting = Greeting::Name("Test greeting");
            email = email.greeting(greeting);
        }

        let _email = email.build();
    }
}
