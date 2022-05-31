mod cmd;
mod config;
mod repo;

fn main() {
    let c = config::init();
    println!("notion api config: {:#?}", c);
    let r = cmd::cli::run();
    let db_id = r.value_of("database").expect("required");
    println!("notion database id: {:#?}", db_id);
}
