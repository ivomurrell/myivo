use crate::{scrobble::ScrobblesTemplate, scrobble_monitor::ScrobbleMonitor};

use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(path = "index.html")]
pub struct RootTemplate {
    scrobble: Option<ScrobblesTemplate>
}

pub async fn get_index(monitor: &mut ScrobbleMonitor) -> RootTemplate {
    let scrobbles_template = monitor.get_scrobble().await;
    if let Err(err) = scrobbles_template.as_ref() {
        tracing::warn!(?err, "Failed to get scrobble");
    }

    RootTemplate {
        scrobble: scrobbles_template.ok()
    }
}
