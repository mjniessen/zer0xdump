use clap::ArgMatches;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

pub fn read_stdin() -> Vec<u8> {
    let mut buffer = Vec::new();
    let _ = io::stdin().read_to_end(&mut buffer);
    buffer
}

pub fn read_file(matches: &ArgMatches) -> Vec<u8> {
    let path = Path::new(matches.get_one::<String>("FILE").unwrap());
    // let display = path.display();
    let mut file = match File::open(path) {
        Err(why) => panic!("File {} couldn't be opened: {}", path.display(), why),
        Ok(file) => file,
    };
    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer);
    buffer
}
