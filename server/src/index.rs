use crate::scrobble_monitor::ScrobbleMonitor;

pub async fn render_index(monitor: &mut ScrobbleMonitor) -> anyhow::Result<String> {
    let index_template = include_str!("../files/index.html");
    let scrobble_partial = monitor.get_scrobble().await?;

    Ok(index_template.replace("{{scrobbles}}", &scrobble_partial.into_string()))
}
