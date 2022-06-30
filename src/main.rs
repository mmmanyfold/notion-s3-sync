mod cmd;
mod config;
mod repo;

use crate::config::NotionAPIConfig;
use crate::repo::notion::client::NotionAPIClient;

#[tokio::main]
async fn main() {
    let nac: NotionAPIConfig = config::init();
    let args = cmd::cli::run();
    let db_id = args
        .value_of("database")
        .expect("required")
        .parse()
        .unwrap();

    let client: NotionAPIClient = NotionAPIClient::new(&nac.key).unwrap();
    if let Ok(pages) = client.get_pages(db_id).await {
        println!("page body response: {:#?}", pages);
    }
}
