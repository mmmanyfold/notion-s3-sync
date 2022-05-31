use clap::{Arg, ArgMatches, Command};

pub fn run() -> ArgMatches {
    Command::new("ns3")
        .about("Notion S3 Sync")
        .arg(
            Arg::new("database")
                .short('d').long("database")
                .help("The notion database id to download")
                .takes_value(true)
                .required(true),
        )
        .get_matches()
}
