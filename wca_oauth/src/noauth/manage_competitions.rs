use async_trait::async_trait;
use super::*;

pub struct WithManageCompetition<T>(pub(super) T);

impl<T> OAuthBuilder for WithManageCompetition<T> where T: OAuthBuilder {
    type ImplicitOAuth = ManageCompetitionsScope<T::ImplicitOAuth>;

    fn scopes(&self) -> Vec< &str>  {
        let mut result = self.0.scopes();
        result.push("manage_competitions");
        result
    }

    fn authenticate_implicit(self, access_token: String) -> Self::ImplicitOAuth {
        ManageCompetitionsScope(self.0.authenticate_implicit(access_token))
    }
}

#[async_trait]
impl<T> OAuthBuilderWithSecret for WithManageCompetition<T> where T: OAuthBuilderWithSecret + Send {
    type ExplicitOAuth = ManageCompetitionsScope<T::ExplicitOAuth>;

    async fn authenticate_explicit(self, access_code: String) -> Result<Self::ExplicitOAuth, String> {
        self.0.authenticate_explicit(access_code)
            .await
            .map(|inner| ManageCompetitionsScope(inner))
    }
}

pub struct ManageCompetitionsScope<T>(T);

pub struct ManageCompetitionTypes;

impl ManageCompetitions for ManageCompetitionTypes { }

#[async_trait]
impl<T> OAuth for ManageCompetitionsScope<T> where T: OAuth + Sync {
    type Email = T::Email;

    type ManageCompetitions = ManageCompetitionTypes;

    type DateOfBirth = T::DateOfBirth;

    fn prefix(&self) -> &str {
        self.0.prefix()
    }

    async fn custom_route(&self, suffix: &str) -> String {
        let result = self.0.custom_route(suffix);
        result.await
    }
}

#[async_trait]
impl<T> Refreshable for ManageCompetitionsScope<T> where T: Refreshable + Send {
    async fn refresh(&mut self) -> Result<(), String> {
        self.0.refresh().await
    }
}
