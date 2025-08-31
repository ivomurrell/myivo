use std::{env, fs, time::Duration};

use anyhow::Context;
use cached::proc_macro::once;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct AppleMusic {
    pub name: String,
    pub art: String,
}

#[derive(Serialize, Debug, Clone)]
struct Claims {
    iss: String,
    iat: u64,
    exp: u64,
}

impl Claims {
    fn new(issuer_id: String) -> Self {
        let iat = jsonwebtoken::get_current_timestamp();
        Claims {
            iss: issuer_id,
            iat,
            exp: iat + 3600,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct AppleMusicResponse {
    data: [AppleMusicTrack; 1],
}

#[derive(Deserialize, Debug, Clone)]
struct AppleMusicTrack {
    attributes: AppleMusicTrackAttributes,
}

#[derive(Deserialize, Debug, Clone)]
struct AppleMusicTrackAttributes {
    name: String,
    artwork: AppleMusicTrackArtwork,
}

#[derive(Deserialize, Debug, Clone)]
struct AppleMusicTrackArtwork {
    url: String,
}

pub struct AppleMusicClient {
    http_client: Client,
    key_id: String,
    team_id: String,
    key: EncodingKey,
    user_token: String,
}

impl AppleMusicClient {
    pub fn new() -> anyhow::Result<Self> {
        let key_id =
            env::var("APPLE_DEVELOPER_TOKEN_KEY_ID").context("missing apple developer key ID")?;
        let team_id =
            env::var("APPLE_DEVELOPER_TOKEN_TEAM_ID").context("missing apple developer team ID")?;
        let auth_key = fs::read("keys/AuthKey.p8").context("missing apple developer auth key")?;
        let key = EncodingKey::from_ec_pem(&auth_key)
            .context("failed to parse apple developer auth key")?;
        let user_token = env::var("APPLE_USER_TOKEN").context("missing apple user token")?;

        Ok(Self {
            http_client: Client::new(),
            key_id,
            team_id,
            key,
            user_token,
        })
    }

    pub async fn fetch(&self) -> anyhow::Result<AppleMusic> {
        let jwt = self.build_developer_token()?;

        let response: AppleMusicResponse = self
            .http_client
            .get("https://api.music.apple.com/v1/me/recent/played/tracks")
            .bearer_auth(jwt)
            .header("Music-User-Token", self.user_token.clone())
            .query(&[("types", "songs"), ("limit", "1")])
            .send()
            .await
            .context("failed to call Apple Music API")?
            .json()
            .await
            .context("failed to parse Apple Music response")?;
        let track = &response.data[0];

        let artwork_url = track.attributes.artwork.url.clone();
        let dimensions = "240";
        let artwork_url = artwork_url
            .replace("{w}", dimensions)
            .replace("{h}", dimensions);

        Ok(AppleMusic {
            name: track.attributes.name.clone(),
            art: artwork_url,
        })
    }

    pub fn build_developer_token(&self) -> anyhow::Result<String> {
        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(self.key_id.clone());
        let claims = Claims::new(self.team_id.clone());

        jsonwebtoken::encode(&header, &claims, &self.key)
            .context("failed to encode apple developer JWT")
    }
}

#[once(time = 30, option = false)]
pub async fn cached_fetch(this: &AppleMusicClient) -> Option<AppleMusic> {
    this.fetch()
        .await
        .map_err(|error| tracing::warn!(?error, "failed to call Apple Music"))
        .ok()
}
