use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use reqwest::Client;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
struct Scrobble {
    data: String,
    fetch_time: Instant,
}

#[derive(Debug, Clone)]
pub struct ScrobbleMonitor {
    client: Client,
    api_key: String,
    last_scrobble: Arc<RwLock<Option<Scrobble>>>,
}

impl ScrobbleMonitor {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            last_scrobble: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn get_scrobble(&mut self) -> anyhow::Result<String> {
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
                let latest = self.fetch_scrobble().await?;
                *last_scrobble = Some(Scrobble {
                    data: latest.clone(),
                    fetch_time: Instant::now(),
                });
                Ok(latest)
            }
        }
    }

    async fn fetch_scrobble(&self) -> anyhow::Result<String> {
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
        Ok(response.text().await?)
    }
}
