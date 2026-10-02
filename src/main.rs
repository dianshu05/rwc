use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: rwc <file>");
        std::process::exit(1);
    }

    let filename = &args[1];

    if let Err(error) = count_file(filename) {
        eprintln!("rwc: {}: {}", filename, error);
        std::process::exit(1);
    }
}

fn count_file(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut lines = 0;
    let mut words = 0;
    let mut bytes = 0;

    for line in reader.lines() {
        let line = line?;

        lines += 1;
        words += line.split_whitespace().count();

        // line.len() counts the UTF-8 bytes in the line.
        bytes += line.len();

        // `lines()` removes the line ending.
        // Add one byte for the newline that was removed.
        bytes += 1;
    }

    println!(
        "lines:{} words:{} bytes:{} filename:{}",
         lines,words, bytes, filename
    );

    Ok(())
}
