use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NotionAPI {
    #[serde(default = "default_unset")]
    key: String,
    #[serde(default = "default_unset")]
    workspace: String,
}

fn default_unset() -> String {
    String::from("UNSET")
}

pub fn init() -> NotionAPI {
    envy::prefixed("NOTION_API_")
        .from_env::<NotionAPI>()
        .expect("Please provide NOTION_API_KEY and NOTION_API_WORKSPACE")
}
