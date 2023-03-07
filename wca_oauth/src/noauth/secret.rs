use super::*;
use async_trait::async_trait;
use reqwest::Client;

pub struct ExplicitOauth<T> {
    client_id: String,
    secret: String,
    redirect_uri: String,
    inner: T,
}

#[async_trait]
impl<T> OAuth for ExplicitOauth<T> where T: OAuth + Send + Sync {
    type Email = T::Email;

    type ManageCompetitions = T::ManageCompetitions;

    type DateOfBirth = T::DateOfBirth;

    fn prefix(&self) ->  &str {
        self.inner.prefix()
    }

    async fn custom_route(&self, suffix: &str) -> String {
        self.inner.custom_route(suffix).await
    }
}

#[async_trait]
impl<T> Refreshable for ExplicitOauth<T> where T: OAuth + Send + Sync {
    async fn refresh(&mut self) -> Result<(), String> {
        todo!()
    }
}


pub struct WithSecret<T> {
    pub(super) client_id: String,
    pub(super) secret: String,
    pub(super) redirect_uri: String,
    pub(super) inner: T,
}

impl<T> OAuthBuilder for WithSecret<T> where T: OAuthBuilder {
    type ImplicitOAuth = T::ImplicitOAuth;

    fn scopes(&self) -> Vec<&str> {
        self.inner.scopes()
    }

    fn authenticate_implicit(self, access_token: String) -> Self::ImplicitOAuth {
        self.inner.authenticate_implicit(access_token)
    }
}

#[async_trait]
impl<T> OAuthBuilderWithSecret for WithSecret<T> where T: OAuthBuilder + Send {
    type ExplicitOAuth = ExplicitOauth<T::ImplicitOAuth>;

    async fn authenticate_explicit(self, access_code: String) -> Result<Self::ExplicitOAuth, String> {
        let params = [
            ("grant_type", "authorization_code"),
            ("client_id", &self.client_id),
            ("client_secret", &self.secret),
            ("redirect_uri", &self.redirect_uri),
            ("code", access_code.trim()),
        ];

        let url = "https://www.worldcubeassociation.org/oauth/token";

        let response = Client::new()
            .post(url)
            .form(&params)
            .send()
            .await.map_err(|e| e.to_string())?
            .text()
            .await.map_err(|e| e.to_string())?;

        let response_json: serde_json::value::Value = serde_json::from_str(&response).unwrap();

        dbg!(&response_json);

        let access_token = response_json["access_token"].as_str().unwrap().to_owned();

        let inner = self.inner.authenticate_implicit(access_token);
        Ok(ExplicitOauth {
            client_id: self.client_id,
            secret: self.secret,
            redirect_uri: self.redirect_uri,
            inner,
        })
    }
}
