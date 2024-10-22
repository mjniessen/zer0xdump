use clap::{command, value_parser, Arg, ArgMatches};
use std::io;
use std::io::Write;

use crate::hexdump::hexdump;
use crate::input::{read_file, read_stdin};

mod hexdump;
mod input;

fn main() {
    let mut stdout = io::stdout();

    let matches: ArgMatches = command!()
        .author("Maurice-Jörg Nießen <info@mjniessen.com>")
        .arg(
            Arg::new("SKIP")
                .long("skip")
                .short('s')
                .value_parser(value_parser!(u64).range(0..))
                .default_value("0")
                .help("Bytes to skip"),
        )
        .arg(
            Arg::new("COUNT")
                .long("count")
                .short('c')
                .value_parser(value_parser!(u64).range(0..))
                .default_value("0")
                .help("Bytes to dump"),
        )
        .arg(
            Arg::new("WIDTH")
                .long("width")
                .short('w')
                .value_parser(value_parser!(u16).range(1..128))
                .default_value("16")
                .help("Column width per line"),
        )
        .arg(
            Arg::new("NOCOLOR")
                .long("nocolor")
                .short('n')
                .action(clap::ArgAction::SetTrue)
                .help("No colors"),
        )
        .arg(Arg::new("FILE").help("File to hexdump."))
        .get_matches();

    if matches.contains_id("FILE") {
        let buffer = read_file(&matches);
        let len = buffer.len();
        hexdump(buffer, matches);
        writeln!(stdout, "Length: {} bytes", len);
    } else {
        let buffer = read_stdin();
        hexdump(buffer, matches);
    }
}
