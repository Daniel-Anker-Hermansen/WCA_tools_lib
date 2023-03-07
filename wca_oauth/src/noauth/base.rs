use async_trait::async_trait;
use reqwest::Client;
use super::*;

pub struct BaseOAuthBuilder;

impl OAuthBuilder for BaseOAuthBuilder {
    type ImplicitOAuth = ImplicitOAuth;

    fn scopes(&self) -> Vec<&str> {
        vec![]
    }

    fn authenticate_implicit(self, access_token: String) -> Self::ImplicitOAuth {
        ImplicitOAuth {
            access_token,
            prefix: "https://www.worldcubeassociation.org/api/v0/".to_owned(),
            client: Client::new(),
        }
    }
}

pub struct ImplicitOAuth {
    access_token: String,
    prefix: String,
    client: Client,
}

#[async_trait]
impl OAuth for ImplicitOAuth {
    type Email = ();

    type ManageCompetitions = ();

    type DateOfBirth = ();

    fn prefix(&self) -> &str {
        &self.prefix
    }

    async fn custom_route(&self, suffix: &str) -> String {
        let url = format!("{}{}", self.prefix, suffix);
        
        self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await.unwrap()
            .text()
            .await.unwrap()
    }
}
