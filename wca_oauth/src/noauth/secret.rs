use std::collections::HashSet;

use super::*;
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

pub struct ExplicitOauth<T> {
    client_id: String,
    secret: String,
    redirect_uri: String,
    refresh_token: String,
    scope: String,
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

    async fn custom_route(&self, suffix: &str) -> Result<String, reqwest::Error> {
        self.inner.custom_route(suffix).await
    }
}

#[async_trait]
impl<T> Refreshable for ExplicitOauth<T> where T: OAuth + Send + Sync {
    fn scopes(&self) -> Vec<&str> {
        self.scope.split_whitespace().collect()
    }

    async fn refresh(&mut self) -> Result<(), Error> {
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

    async fn authenticate_explicit(self, access_code: String) -> Result<Self::ExplicitOAuth, Error> {
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
            .await?
            .text()
            .await?;

        let auth_response = serde_json::from_str::<AuthResponse>(&response)
            .map_err(|_| serde_json::from_str::<ApiError>(&response)
                .map(|e| Error::from(e))
                .unwrap_or_else(|_| Error::from(format!("Error in requring authentification. The error did not conform to the expected format. Received following: {response}"))))?;

        let access_token = auth_response.access_token;

        let required_scopes: HashSet<_> = self.scopes().into_iter().collect();
        let obtained_scopes: HashSet<_> = auth_response.scope.split_whitespace().collect();
        let mut difference_scopes = required_scopes.difference(&obtained_scopes);

        if let Some(scope) = difference_scopes.next() {
            return Err(Error::MissingScope(scope.to_string()));
        }

        let inner = self.inner.authenticate_implicit(access_token);
        Ok(ExplicitOauth {
            client_id: self.client_id,
            secret: self.secret,
            redirect_uri: self.redirect_uri,
            inner,
            refresh_token: auth_response.refresh_token,
            scope: auth_response.scope,
        })
    }
}

#[derive(Deserialize, Debug)]
struct AuthResponse {
    access_token: String,
    //created_at: i64,
    //expires_in: i64,
    refresh_token: String,
    scope: String,
    //token_type: String,
}
