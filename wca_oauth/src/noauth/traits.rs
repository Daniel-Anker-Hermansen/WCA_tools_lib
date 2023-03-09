use async_trait::async_trait;
use super::*;

/// The special WCA loopback uri, useful in early development
pub const LOOPBACK_URI: &str = "urn:ietf:wg:oauth:2.0:oob";

/// Main trait for an oauth instance
/// An implementation of this can be obtained through an OAuthBuilder
#[async_trait]
pub trait OAuth {
    type Email: Email;

    type ManageCompetitions: ManageCompetitions;
    
    type DateOfBirth: DateOfBirth;

    async fn me(&self) -> String {
        self.custom_route("me").await.unwrap()
    }

    async fn wcif(&self, competition_name: &str) -> String where Self: OAuth<ManageCompetitions = ManageCompetitionTypes> {
        let suffix = format!("competitions/{}/wcif", competition_name);
        self.custom_route(&suffix).await.unwrap()
    }

    fn prefix(&self) -> &str;

    async fn custom_route(&self, suffix: &str) -> Result<String, reqwest::Error>;
}

#[async_trait]
pub trait Refreshable { 
    fn scopes(&self) -> Vec<&str>;

    async fn refresh(&mut self) -> Result<(), Error>;
}

pub trait Email { }

impl Email for () { }

pub trait ManageCompetitions { }

impl ManageCompetitions for () { }

pub trait DateOfBirth { }

impl DateOfBirth for () { }

/// Builder trait for building an oauth instance
pub trait OAuthBuilder: Sized {
    type ImplicitOAuth: OAuth + Sync + Send;

    fn with_secret(self, client_id: String, secret: String, redirect_uri: String) -> WithSecret<Self> {
        WithSecret { client_id, secret, redirect_uri, inner: self }
    }

    fn with_manage_competition_scope(self) -> WithManageCompetition<Self> {
        WithManageCompetition(self)
    }

    fn scopes(&self) -> Vec<&str>;

    fn authenticate_implicit(self, access_token: String) -> Self::ImplicitOAuth;
}

#[async_trait]
pub trait OAuthBuilderWithSecret: Sized + OAuthBuilder {
    type ExplicitOAuth: OAuth + Refreshable + Sync + Send;

    async fn authenticate_explicit(self, access_code: String) -> Result<Self::ExplicitOAuth, Error>;
}


