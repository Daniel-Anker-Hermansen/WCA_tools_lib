use crate::WcifContainer;

/// The special WCA loopback uri, useful in early development
pub const LOOPBACK_URI: &str = "urn:ietf:wg:oauth:2.0:oob";

/// Main trait for an oauth instance
/// An implementation of this can be obtained through an OAuthBuilder
pub trait OAuth {

}

/// Inidicates an oauth which can be refreshed, i.e. is acquired through regular flow.
pub trait Refreshable: OAuth {
    fn refresh(&mut self);
}

/// Indicates an oauth which has the manage competition scope
pub trait ManageCompetition: OAuth {
    fn get_wcif(&self, competition_name: String) -> WcifContainer;
}

pub trait OAuthBuilder: Sized {
    type ImplicitOAuth;

    fn with_secret(self, secret: String) -> WithSecret<Self> {
        WithSecret { secret, inner: self }
    }

    fn with_manage_competition_scope(self) -> WithManageCompetition<Self> {
        WithManageCompetition(self)
    }

    fn scopes(&self) -> Vec<&str>;

    fn authenticate_implicit(self, auth_code: String) -> Self::ImplicitOAuth;

    fn authenticate_explicit(self, auth_token: String) -> <Self as OAuthBuilderWithSecret>::ExplicitOAuth where Self: OAuthBuilderWithSecret;
}

pub trait OAuthBuilderWithSecret {
    type ExplicitOAuth;

    fn get_secret(&self) -> &str;
}

pub struct WithSecret<T> {
    secret: String,
    inner: T,
}

pub struct WithManageCompetition<T>(T);
