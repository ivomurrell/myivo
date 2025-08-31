use std::{env, time::Duration};

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

impl AppleMusic {
    pub async fn fetch() -> anyhow::Result<Self> {
        let jwt = build_developer_token()?;
        let user_token = env::var("APPLE_USER_TOKEN").context("missing apple user token")?;

        let client = Client::new();
        let response: AppleMusicResponse = client
            .get("https://api.music.apple.com/v1/me/recent/played/tracks")
            .bearer_auth(jwt)
            .header("Music-User-Token", user_token)
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

        Ok(Self {
            name: track.attributes.name.clone(),
            art: artwork_url,
        })
    }
}

#[once(time = 30, option = false)]
pub async fn cached_fetch() -> Option<AppleMusic> {
    AppleMusic::fetch()
        .await
        .map_err(|error| tracing::warn!(?error, "failed to call Apple Music"))
        .ok()
}

pub fn build_developer_token() -> anyhow::Result<String> {
    let mut header = Header::new(Algorithm::ES256);
    header.kid =
        Some(env::var("APPLE_DEVELOPER_TOKEN_KEY_ID").context("missing apple developer key ID")?);
    let team_id =
        env::var("APPLE_DEVELOPER_TOKEN_TEAM_ID").context("missing apple developer team ID")?;
    let claims = Claims::new(team_id);
    let auth_key =
        env::var("APPLE_DEVELOPER_TOKEN_AUTH_KEY").context("missing apple developer auth key")?;
    let key = EncodingKey::from_ec_pem(auth_key.as_bytes())
        .context("failed to parse appple developer auth key")?;

    jsonwebtoken::encode(&header, &claims, &key).context("failed to encode apple developer JWT")
}
