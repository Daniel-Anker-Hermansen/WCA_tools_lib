use async_trait::async_trait;

use super::*;

pub struct StagingBuilder<T>(pub(super) T);

impl<T> OAuthBuilder for StagingBuilder<T> where T: OAuthBuilder {
    type ImplicitOAuth = T::ImplicitOAuth;

    fn scopes(&self) -> Vec<&str> {
        self.0.scopes()
    }

    fn authenticate_implicit(self, access_token: String) -> Self::ImplicitOAuth {
        let mut oauth = self.0.authenticate_implicit(access_token);
        oauth.set_prefix("https://staging.worldcubeassociation.org/api/v0/".to_owned());
        oauth
    }
}

#[async_trait]
impl<T> OAuthBuilderWithSecret for StagingBuilder<T> where T: OAuthBuilderWithSecret + Sync + Send {
    type ExplicitOAuth = T::ExplicitOAuth;

    fn set_url(&mut self, url: String) {
        self.0.set_url(url);
    }

    async fn authenticate_explicit(mut self, access_code: String) -> Result<Self::ExplicitOAuth, Error> {
        self.0.set_url("https://staging.worldcubeassociation.org/oauth/token".to_owned());
        let mut oauth = self.0.authenticate_explicit(access_code).await?;
        oauth.set_prefix("https://staging.worldcubeassociation.org/api/v0/".to_owned());
        Ok(oauth)
    }
}
