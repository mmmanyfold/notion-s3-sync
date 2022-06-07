mod cmd;
mod config;
mod repo;

use crate::config::NotionAPIConfig;
use crate::repo::notion::client;

#[tokio::main]
async fn main() {
    let nac: NotionAPIConfig = config::init();
    let args = cmd::cli::run();
    let db_id = args
        .value_of("database")
        .expect("required")
        .parse()
        .unwrap();

    if let Ok(body) = client::get_pages(&nac.key, &db_id).await {
        println!("page body response: {:#?}", body);
    }
}
