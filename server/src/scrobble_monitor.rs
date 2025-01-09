use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use maud::{html, Markup};
use reqwest::Client;
use serde::Deserialize;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
struct CachedScrobble {
    data: Markup,
    fetch_time: Instant,
}

#[derive(Debug, Clone, Deserialize)]
struct ScrobbleArtist {
    #[serde(rename = "#text")]
    text: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ScrobbleImage {
    #[serde(rename = "#text")]
    text: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ScrobbleAttributes {
    #[serde(rename = "nowplaying")]
    now_playing: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct ScrobbleTrack {
    artist: ScrobbleArtist,
    image: Vec<ScrobbleImage>,
    name: String,
    #[serde(rename = "@attr")]
    attributes: Option<ScrobbleAttributes>,
}

#[derive(Debug, Clone, Deserialize)]
struct ScrobbleRecentTracks {
    track: Vec<ScrobbleTrack>,
}

#[derive(Debug, Clone, Deserialize)]
struct Scrobble {
    #[serde(rename = "recenttracks")]
    recent_tracks: ScrobbleRecentTracks,
}

#[derive(Debug, Clone)]
pub struct ScrobbleMonitor {
    client: Client,
    api_key: String,
    last_scrobble: Arc<RwLock<Option<CachedScrobble>>>,
}

impl ScrobbleMonitor {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            last_scrobble: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn get_scrobble(&mut self) -> anyhow::Result<Markup> {
        let is_fresh = |fetch_time: &Instant| fetch_time.elapsed() < Duration::from_secs(30);

        if let Some(scrobble) = &*self.last_scrobble.read().await {
            if is_fresh(&scrobble.fetch_time) {
                tracing::debug!("returning recently fetched scrobble data");
                return Ok(scrobble.data.clone());
            }
        }

        let mut last_scrobble = self.last_scrobble.write().await;
        match &*last_scrobble {
            // make sure another task hasn't fetched the new data first after we
            // both waited for write access
            Some(scrobble) if is_fresh(&scrobble.fetch_time) => {
                tracing::debug!("returning (very) recently fetched scrobble data");
                Ok(scrobble.data.clone())
            }
            _ => {
                tracing::debug!("fetching new scrobble data");
                let scrobble = self.fetch_scrobble().await?;
                let scrobble_partial = self.scrobble_partial(&scrobble);
                *last_scrobble = Some(CachedScrobble {
                    data: scrobble_partial.clone(),
                    fetch_time: Instant::now(),
                });
                Ok(scrobble_partial)
            }
        }
    }

    async fn fetch_scrobble(&self) -> anyhow::Result<Scrobble> {
        let response = self
            .client
            .get("https://ws.audioscrobbler.com/2.0")
            .query(&[
                ("method", "user.getRecentTracks"),
                ("api_key", &self.api_key),
                ("user", "Doomboy95"),
                ("limit", "1"),
                ("format", "json"),
            ])
            .send()
            .await?;
        Ok(response.json().await?)
    }

    fn scrobble_partial(&self, scrobble: &Scrobble) -> Markup {
        let latest_track = &scrobble.recent_tracks.track[0];
        let srcset = format!(
            "{}, {} 2x, {} 3x",
            latest_track.image[0].text, latest_track.image[1].text, latest_track.image[2].text
        );
        let text_intro = if latest_track
            .attributes
            .as_ref()
            .map_or(false, |attr| attr.now_playing)
        {
            "Now playing: "
        } else {
            "Last played: "
        };
        let now_playing = format!("{} - {}", latest_track.name, latest_track.artist.text);

        html! {
            .bar-container {
                img .bar-cover
                    src=(latest_track.image[0].text)
                    alt="Cover art"
                    srcset=(srcset);

                p .bar-text-intro {
                    (text_intro)
                }

                p .bar-text-music {
                    (now_playing)
                }
            }
        }
    }
}
