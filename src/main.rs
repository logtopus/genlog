// #[macro_use]
// extern crate log;

use clap::{App, Arg, ArgMatches};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let cli_matches = parse_cli();

    gen_log(&cli_matches);
}

fn gen_log(cli_matches: &ArgMatches) {
    let lines = cli_matches
        .value_of("l")
        .map(|s| s.parse().unwrap())
        .unwrap_or(100);
    let rotations = cli_matches
        .value_of("r")
        .map(|s| s.parse().unwrap())
        .unwrap_or(0);
    let path = Path::new(cli_matches.value_of("o").unwrap());

    let msgtpl: Vec<&str> = vec![
        "{dt} TRACE This is a trace message. Its line number {count}\n",
        "{dt} DEBUG This is a debug message. Its line number {count}\n",
        "{dt} INFO This is an info message. Its line number {count}\n",
        "{dt} WARN This is a warning message. Its line number {count}\n",
        "{dt} ERROR This is an error message. Its line number {count}\n",
    ];

    let now = chrono::Utc::now();
    let mut ts = now - chrono::Duration::days(10);

    let mut count = rotations;
    while count >= 0 {
        let path = if count == 0 {
            path.join("testlog.log")
        } else {
            path.join(format!("testlog.log.{}", count))
        };

        println!("Creating file {:?}", path.to_str().unwrap());

        let mut file = File::create(path).unwrap();
        for i in 0..lines {
            let msg_idx = rand::random::<usize>() % msgtpl.len();

            let line = msgtpl.get(msg_idx).unwrap();
            let line = line.replace("{dt}", &ts.format("%Y-%m-%d %H:%M:%S").to_string());
            let line = line.replace("{count}", &(count * lines + i).to_string());
            file.write_all(line.as_bytes()).unwrap();

            let time_add = rand::random::<u64>() % 10000;
            ts = ts + chrono::Duration::milliseconds(time_add as i64);
        }
        count -= 1;
    }
}

fn parse_cli() -> clap::ArgMatches<'static> {
    App::new("Logtopus test log generator")
        .version(VERSION)
        .author(AUTHORS)
        .about("Provide logfiles for testing purposes")
        .arg(
            Arg::with_name("o")
                .required(true)
                .short("o")
                .long("outpath")
                .value_name("OUTPUT PATH")
                .help("Sets output path for generated log files")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("l")
                .short("l")
                .long("lines")
                .value_name("LINES")
                .help("Set number of log file lines to generate, defaults to 100")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("r")
                .short("r")
                .long("rotations")
                .value_name("ROTATIONS")
                .help("Set number of log file rotations, defaults to 0")
                .takes_value(true),
        )
        .get_matches()
}
