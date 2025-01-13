use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use reqwest::Client;
use tokio::sync::RwLock;

use crate::scrobble::{scrobble_partial, Scrobble, ScrobblesTemplate};

#[derive(Debug, Clone)]
struct CachedScrobble {
    data: ScrobblesTemplate,
    fetch_time: Instant,
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

    pub async fn get_scrobble(&mut self) -> anyhow::Result<ScrobblesTemplate> {
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
                let scrobble_partial = scrobble_partial(&scrobble);
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
}
