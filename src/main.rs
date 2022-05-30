use serde::Deserialize;

fn main() {
    let config = envy::prefixed("NOTION_API_")
        .from_env::<NotionAPI>()
        .expect("Please provide NOTION_API_KEY and NOTION_API_WORKSPACE");

    println!("notion api key: {:#?}", config);
}

#[derive(Deserialize, Debug)]
struct NotionAPI {
    #[serde(default="default_unset")]
    key: String,
    #[serde(default="default_unset")]
    workspace: String
}

fn default_unset() -> String {
    String::from("UNSET")
}
