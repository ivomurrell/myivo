use askama::Template;

use crate::scrapers::apple_music::AppleMusicClient;

#[derive(Template, Debug, Clone)]
#[template(path = "am-auth-flow.html")]
pub struct AuthFlowTemplate {
    token: String,
}

impl AuthFlowTemplate {
    pub fn new(apple_music_client: &AppleMusicClient) -> anyhow::Result<Self> {
        let token = apple_music_client.build_developer_token()?;
        Ok(Self { token })
    }
}
