use super::super::{
    scope::{self, Scope},
    Client, GrantType, TokenType,
};
use serde::{Deserialize, Serialize};
use twilight_model::id::ApplicationId;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ClientCredentialsGrantRequestBody<'a> {
    /// ID of the application that was authorized.
    pub client_id: ApplicationId,
    /// Secret of the application that was authorized.
    pub client_secret: &'a str,
    /// Type of grant approval.
    pub grant_type: GrantType,
    /// List of scopes that the user granted.
    ///
    /// This is space-delimited.
    pub scope: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ClientCredentialsGrantRequest<'a> {
    /// Body to send.
    pub body: ClientCredentialsGrantRequestBody<'a>,
    /// Headers to send.
    pub headers: &'static [(&'static str, &'static str)],
    /// Base of the URL.
    ///
    /// Use the [`url`] method for the full URL with query parameters.
    ///
    /// [`url`]: #method.url
    pub url_base: &'static str,
}

impl ClientCredentialsGrantRequest<'_> {
    /// Retrieve a URL with the body urlencoded as query parameters.
    ///
    /// This URL can be used to make a POST request with the specified
    /// [`headers`].
    ///
    /// [`headers`]: #structfield.url_base
    pub fn url(&self) -> String {
        let mut buf = self.url_base.to_owned();
        buf.push_str("?grant_type=");
        buf.push_str(self.body.grant_type.name());

        if !self.body.scope.is_empty() {
            buf.push_str("&scope=");
            buf.push_str(&urlencoding::encode(&self.body.scope));
        }

        buf
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ClientCredentialsGrantResponse {
    /// Access token to be used when making requests to the API on the user's
    /// behalf.
    pub access_token: String,
    /// Number of seconds from issuing that the access token is valid.
    ///
    /// After this duration, the refresh token must be exchanged for another
    /// access token and refresh token pair.
    pub expires_in: u64,
    /// Type of token provided.
    ///
    /// This will always be [`TokenType::Bearer`].
    ///
    /// [`TokenType::Bearer`]: ../enum.TokenType.html#variant.Bearer
    pub token_type: TokenType,
    /// Space-delimited list of scopes that the token has had approved.
    pub scope: String,
}

/// Create a client credentials grant request.
///
/// This can be used to quickly create a Bearer access token for the bot's
/// owner. Discord's documentation warns:
///
/// # Examples
///
/// Create a URL that can be POSTed to that will create an access token
/// for the bot's owner:
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use twilight_model::id::ApplicationId;
/// use twilight_oauth2::Client;
///
/// let application_id = ApplicationId(123);
/// let client_secret = "abcdef01234567890";
///
/// let client = Client::new(application_id, client_secret, &["https://example.com"])?;
/// let mut url_builder = client.client_credentials_grant();
/// let request = url_builder.build();
///
/// println!("grant url: {}", request.url());
/// # Ok(()) }
/// ```
#[derive(Clone, Debug)]
pub struct ClientCredentialsGrantBuilder<'a> {
    client: &'a Client,
    scopes: &'a [Scope],
}

impl<'a> ClientCredentialsGrantBuilder<'a> {
    const BASE_URL: &'static str = "https://discord.com/api/v6/oauth2/token";

    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            scopes: &[Scope::Identify],
        }
    }

    /// Build a client credentials grant URL.
    pub fn build(&'a self) -> ClientCredentialsGrantRequest<'a> {
        ClientCredentialsGrantRequest {
            body: ClientCredentialsGrantRequestBody {
                client_id: self.client.client_id(),
                client_secret: self.client.client_secret(),
                grant_type: GrantType::ClientCredentials,
                scope: scope::join(self.scopes),
            },
            headers: &[("Content-Type", "application/x-www-form-urlencoded")],
            url_base: Self::BASE_URL,
        }
    }

    /// Set the scopes for the client credentials grant request.
    ///
    /// By default the [`Identify`] scope is selected.
    ///
    /// Read about Discord's [scope documentation].
    ///
    /// [RFC 6749 § 3.3] on access token scopes.
    ///
    /// [`Bot`]: enum.Scope.html#variant.Bot
    /// [`Identify`]: enum.Scope.html#variant.Identify
    /// [RFC 6749 § 3.3]: https://tools.ietf.org/html/rfc6749#section-3.3
    /// [scope documentation]: https://discord.com/developers/docs/topics/oauth2#shared-resources-oauth2-scopes
    pub fn scopes(&mut self, scopes: &'a [Scope]) -> &mut Self {
        self.scopes = scopes;

        self
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Client, ClientCredentialsGrantBuilder, ClientCredentialsGrantRequest,
        ClientCredentialsGrantRequestBody, ClientCredentialsGrantResponse, GrantType, Scope,
    };
    use serde::{Deserialize, Serialize};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;
    use twilight_model::id::ApplicationId;

    assert_fields!(ClientCredentialsGrantRequestBody<'_>: client_id, client_secret, grant_type, scope);
    assert_fields!(ClientCredentialsGrantRequest<'_>: body, headers, url_base);
    assert_fields!(
        ClientCredentialsGrantResponse: access_token,
        expires_in,
        token_type,
        scope
    );
    assert_impl_all!(ClientCredentialsGrantBuilder<'_>: Clone, Debug, Send, Sync);
    assert_impl_all!(ClientCredentialsGrantRequestBody<'_>: Clone, Debug, Eq, PartialEq, Send, Serialize, Sync);
    assert_impl_all!(ClientCredentialsGrantRequest<'_>: Clone, Debug, Eq, PartialEq, Send, Serialize, Sync);
    assert_impl_all!(
        ClientCredentialsGrantResponse: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn test_client_credentials_grant_request() {
        let client = Client::new(ApplicationId(1), "a", &["https://example.com"]).unwrap();
        let mut builder = client.client_credentials_grant();
        let req = builder.build();
        assert_eq!(
            req.headers,
            &[("Content-Type", "application/x-www-form-urlencoded")]
        );
        assert_eq!(req.url_base, "https://discord.com/api/v6/oauth2/token");
        assert_eq!(
            req.body,
            ClientCredentialsGrantRequestBody {
                client_id: ApplicationId(1),
                client_secret: "a",
                grant_type: GrantType::ClientCredentials,
                scope: Scope::Identify.name().to_owned(),
            }
        );
        assert_eq!(
            "https://discord.com/api/v6/oauth2/token?grant_type=client_credentials&scope=identify",
            req.url(),
        );

        builder.scopes(&[Scope::Guilds, Scope::Identify]);
        let req = builder.build();
        assert_eq!(
            req.body,
            ClientCredentialsGrantRequestBody {
                client_id: ApplicationId(1),
                client_secret: "a",
                grant_type: GrantType::ClientCredentials,
                scope: "guilds identify".to_owned(),
            }
        );
        assert_eq!(
            "https://discord.com/api/v6/oauth2/token?grant_type=client_credentials&scope=guilds%20identify",
            req.url(),
        );
    }
}
