use serde::{Deserialize, Serialize};

/// Type of approved grant.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    /// Authorization code.
    AuthorizationCode,
    /// Client credentials.
    ClientCredentials,
    /// Refresh token.
    RefreshToken,
}

impl GrantType {
    /// Return the name of the grant type.
    ///
    /// This is equivalent to what you would get when serializing it.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_oauth2::GrantType;
    ///
    /// assert_eq!("authorization_code", GrantType::AuthorizationCode.name());
    /// ```
    pub fn name(&self) -> &str {
        match self {
            Self::AuthorizationCode => "authorization_code",
            Self::ClientCredentials => "client_credentials",
            Self::RefreshToken => "refresh_token",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GrantType;
    use serde::{Deserialize, Serialize};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(
        GrantType: Clone,
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
    fn test_grant_types() {
        assert_eq!("authorization_code", GrantType::AuthorizationCode.name());
        assert_eq!("client_credentials", GrantType::ClientCredentials.name());
        assert_eq!("refresh_token", GrantType::RefreshToken.name());
    }
}
