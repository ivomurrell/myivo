use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(path = "index.html")]
pub struct RootTemplate {}

impl RootTemplate {
    pub fn new() -> RootTemplate {
        RootTemplate {}
    }
}
