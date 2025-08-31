use askama::Template;

use crate::scrapers::apple_music;

#[derive(Template, Debug, Clone)]
#[template(path = "am-auth-flow.html")]
pub struct AuthFlowTemplate {
    token: String,
}

impl AuthFlowTemplate {
    pub fn new() -> anyhow::Result<Self> {
        let token = apple_music::build_developer_token()?;
        Ok(Self { token })
    }
}
