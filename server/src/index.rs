use crate::{scrobble::ScrobblesTemplate, scrobble_monitor::ScrobbleMonitor};

use std::time::Duration;

use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(path = "index.html")]
pub struct RootTemplate {
    scrobble: Option<ScrobblesTemplate>,
}

pub async fn get_index(mut monitor: ScrobbleMonitor) -> RootTemplate {
    let scrobbles_template = tokio::time::timeout(
        Duration::from_millis(200),
        tokio::spawn(async move { monitor.get_scrobble().await }),
    )
    .await;

    RootTemplate {
        scrobble: scrobbles_template
            .map_err(|_| tracing::debug!("last.fm request took too long"))
            .and_then(|scrobble| {
                scrobble
                    .map_err(|err| tracing::error!(?err, "Panicked when trying to get scrobble"))
            })
            .and_then(|scrobble| {
                scrobble.map_err(|err| tracing::warn!(?err, "Failed to get scrobble"))
            })
            .ok(),
    }
}
