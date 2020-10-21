use serde::{Deserialize, Serialize};

/// Type of token.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "PascalCase")]
pub enum TokenType {
    /// Bearer token.
    Bearer,
}

impl TokenType {
    /// Return the name of the prompt.
    ///
    /// This is equivalent to what you would get when serializing it.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_oauth2::TokenType;
    ///
    /// assert_eq!("Bearer", TokenType::Bearer.name());
    /// ```
    pub fn name(&self) -> &str {
        match self {
            Self::Bearer => "Bearer",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TokenType;
    use serde::{Deserialize, Serialize};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(
        TokenType: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn test_token_types() {
        assert_eq!("Bearer", TokenType::Bearer.name());
    }
}
