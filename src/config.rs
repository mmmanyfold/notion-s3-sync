use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NotionAPIConfig {
    #[serde(default = "default_unset")]
    pub key: String,
    #[serde(default = "default_unset")]
    pub workspace: String,
}

fn default_unset() -> String {
    String::from("UNSET")
}

pub fn init() -> NotionAPIConfig {
    envy::prefixed("NOTION_API_")
        .from_env::<NotionAPIConfig>()
        .expect("Please provide NOTION_API_KEY and NOTION_API_WORKSPACE")
}
