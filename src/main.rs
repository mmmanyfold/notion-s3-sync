mod cmd;
mod config;
mod repo;

use crate::config::NotionAPIConfig;
use repo::notion;

fn main() {
    let nac: NotionAPIConfig = config::init();
    let args = cmd::cli::run();
    let db_id = args
        .value_of("database")
        .expect("required")
        .parse()
        .unwrap();

    if let Ok(body) = notion::get_pages(&nac.key, &db_id) {
        println!("page body response: {:#?}", body);
    }
}
