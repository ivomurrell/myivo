use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::anyhow;
use reqwest::Client;
use tokio::sync::RwLock;

use crate::scrobble::{Response, Scrobble, ScrobblesTemplate};

#[derive(Debug, Clone)]
struct CachedScrobble {
    data: ScrobblesTemplate,
    fetch_time: Instant,
}

impl CachedScrobble {
    fn is_fresh(&self) -> bool {
        self.fetch_time.elapsed() < Duration::from_secs(30)
    }
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

    pub fn try_get_scrobble(&self) -> Option<ScrobblesTemplate> {
        let scrobble = &*self.last_scrobble.try_read().ok()?;
        scrobble
            .as_ref()
            .filter(|scrobble| scrobble.is_fresh())
            .map(|scrobble| scrobble.data.clone())
    }

    pub async fn get_scrobble(&self) -> anyhow::Result<ScrobblesTemplate> {
        if let Some(scrobble) = &*self.last_scrobble.read().await {
            if scrobble.is_fresh() {
                tracing::debug!("returning recently fetched scrobble data");
                return Ok(scrobble.data.clone());
            }
        }

        let mut last_scrobble = self.last_scrobble.write().await;
        match &*last_scrobble {
            // make sure another task hasn't fetched the new data first after we
            // both waited for write access
            Some(scrobble) if scrobble.is_fresh() => {
                tracing::debug!("returning (very) recently fetched scrobble data");
                Ok(scrobble.data.clone())
            }
            _ => {
                tracing::debug!("fetching new scrobble data");
                let scrobble = self.fetch_scrobble().await?;
                let scrobble_partial = ScrobblesTemplate::new(&scrobble);
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
        let response: Response = response.json().await?;
        match response {
            Response::Scrobble(scrobble) => Ok(scrobble),
            Response::Error(err) => {
                Err(anyhow!("last.fm responded with an error: {}", err.message))
            }
        }
    }
}
