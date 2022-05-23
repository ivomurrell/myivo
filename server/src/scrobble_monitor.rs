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
        if let Some(scrobble) = &*self.last_scrobble.read().await {
            if scrobble.fetch_time.elapsed() < Duration::from_secs(30) {
                tracing::debug!("returning recently fetched scrobble data");
                return Ok(scrobble.data.clone());
            }
        }

        tracing::debug!("fetching new scrobble data");
        // try and prevent multiple handlers calling the API at the same time
        let mut write_guard = self.last_scrobble.write().await;
        let latest = self.fetch_scrobble().await?;
        *write_guard = Some(Scrobble {
            data: latest.clone(),
            fetch_time: Instant::now(),
        });
        Ok(latest)
    }

    async fn fetch_scrobble(&self) -> anyhow::Result<String> {
        let response = self
            .client
            .get("http://ws.audioscrobbler.com/2.0")
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
