mod cmd;
mod config;
mod repo;

fn main() {
    let c = config::init();
    println!("notion api key: {:#?}", c);
}
