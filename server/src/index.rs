use crate::scrobble_monitor::ScrobbleMonitor;

use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(path = "index.html")]
pub struct RootTemplate {
    intro: String,
    now_playing: String,
    image: String,
    srcset: String,
}

pub async fn get_index(monitor: &mut ScrobbleMonitor) -> anyhow::Result<RootTemplate> {
    let scrobbles_template = monitor.get_scrobble().await?;

    Ok(RootTemplate {
        intro: scrobbles_template.intro,
        now_playing: scrobbles_template.now_playing,
        image: scrobbles_template.image,
        srcset: scrobbles_template.srcset,
    })
}
