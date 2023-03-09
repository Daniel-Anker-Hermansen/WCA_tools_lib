use std::collections::HashMap;

use crate::Competition;

use super::*;

pub struct Competitions<T> {
    query: HashMap<String, String>,
    inner: T,
}

impl<T> Competitions<T> where T: OAuth {
    pub async fn send(self) -> Result<Vec<Competition>, Error> {
        let url = format!("competitions?{}",
            self.query.into_iter()
                .map(|(key, value)| format!("{key}={value}"))
                .collect::<Vec<_>>() // replace with intersperce when stable
                .join("&"));
        let json = self.inner.custom_route(&url).await?;
        parse_json(&json)
    }
}

fn parse_json<'de, T>(json: &'de str) -> Result<T, Error> where T: Deserialize<'de> {
    serde_json::from_str(&json)
        .or_else(|_| {
            Err(serde_json::from_str::<ApiError>(&json)
                .map(|api_error| api_error.into())
                .unwrap_or_else(|_| {
                    Error::Other(json.to_owned())
                }))
        })
}
