use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Whether to prompt the user again when they have already authorized the
/// application.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum Prompt {
    /// Always ask the user for consent.
    Consent,
    /// Don't ask the user for consent.
    None,
}

impl Prompt {
    /// Return the name of the prompt.
    ///
    /// This is equivalent to what you would get when serializing it.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_oauth2::Prompt;
    ///
    /// assert_eq!("consent", Prompt::Consent.name());
    /// ```
    pub fn name(&self) -> &str {
        match self {
            Self::Consent => "consent",
            Self::None => "none",
        }
    }
}

impl Display for Prompt {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::Prompt;
    use serde::{Deserialize, Serialize};
    use static_assertions::assert_impl_all;
    use std::fmt::{Debug, Display};

    assert_impl_all!(
        Prompt: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Display,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn test_prompts() {
        assert_eq!("consent", Prompt::Consent.name());
        assert_eq!("consent", Prompt::Consent.to_string());
        assert_eq!("none", Prompt::None.name());
        assert_eq!("none", Prompt::None.to_string());
    }
}
