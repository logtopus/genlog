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
        .value_of("lines")
        .map(|s| s.parse().unwrap())
        .unwrap_or(100);
    let rotations = cli_matches
        .value_of("rotations")
        .map(|s| s.parse().unwrap())
        .unwrap_or(0);
    let path = Path::new(cli_matches.value_of("output").unwrap());

    let now = chrono::Utc::now();

    let end = cli_matches
        .value_of("timerange-end")
        .map(|s| {
            chrono::DateTime::from_utc(
                chrono::NaiveDateTime::from_timestamp(
                    s.parse::<i64>().expect("End requires seconds as argument"),
                    0,
                ),
                chrono::Utc,
            )
        })
        .unwrap_or(now);

    let begin = cli_matches
        .value_of("timerange-begin")
        .map(|s| {
            chrono::DateTime::from_utc(
                chrono::NaiveDateTime::from_timestamp(
                    s.parse::<i64>()
                        .expect("Begin requires seconds as argument"),
                    0,
                ),
                chrono::Utc,
            )
        })
        .unwrap_or(end - chrono::Duration::days(10));

    if begin > end {
        panic!("Begin cannot be after end.")
    }

    let maxgap = cli_matches
        .value_of("maximum-gap")
        .map(|s| {
            s.parse::<u64>()
                .expect("Maximum gap must be a positive value in milliseconds")
        })
        .unwrap_or(1000 * 60 * 5);

    let msgtpl: Vec<&str> = vec![
        "{dt} TRACE This is a trace message. Its line number {count}\n",
        "{dt} DEBUG This is a debug message. Its line number {count}\n",
        "{dt} INFO This is an info message. Its line number {count}\n",
        "{dt} WARN This is a warning message. Its line number {count}\n",
        "{dt} ERROR This is an error message. Its line number {count}\n",
    ];

    let mut ts = begin;
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

            let time_add = rand::random::<u64>() % maxgap;
            ts = ts + chrono::Duration::milliseconds(time_add as i64);
            if ts > end {
                break;
            }
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
            Arg::with_name("output")
                .required(true)
                .short("o")
                .long("outpath")
                .value_name("OUTPUT PATH")
                .help("Sets output path for generated log files")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .value_name("LINES")
                .help("Set maximum number of log file lines to generate per rotation, defaults to 100. Truncated if end time stamp is reached")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("rotations")
                .short("r")
                .long("rotations")
                .value_name("ROTATIONS")
                .help("Set number of log file rotations, defaults to 0")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("timerange-begin")
                .short("b")
                .long("begin")
                .value_name("start time stamp in UTC seconds")
                .help("Set optional start timestamp, defaults to 10 days before 'end'")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("timerange-end")
                .short("e")
                .long("end")
                .value_name("stop time stamp in UTC seconds")
                .help("Set optional end timestamp, defaults to now")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("maximum-gap")
                .short("g")
                .long("maxgap")
                .value_name("maximum gap in milliseconds")
                .help("Set optional maximum gap between log entries, defaults to 5 minutes")
                .takes_value(true),
        )
        .get_matches()
}
