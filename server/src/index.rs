use crate::{scrobble::ScrobblesTemplate, scrobble_monitor::ScrobbleMonitor};

use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(path = "index.html")]
pub struct RootTemplate {
    scrobble: Option<ScrobblesTemplate>,
}

pub async fn get_index(mut monitor: ScrobbleMonitor) -> RootTemplate {
    let scrobbles_template = monitor.try_get_scrobble();
    if scrobbles_template.is_none() {
        // start fetching scrobble so we can send a fresh response to the client ASAP
        tokio::spawn(async move { monitor.get_scrobble().await });
    }

    RootTemplate {
        scrobble: scrobbles_template,
    }
}
