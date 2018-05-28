#[macro_use]
extern crate clap;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use clap::{App, Arg, ArgMatches};

// TODO: Make parallel for large inputs. Get rid of the unwrap.
fn read_lines<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let input = File::open(path)?;
    let reader = BufReader::new(input);
    Ok(reader.lines().map(|l| l.unwrap()).collect())
}

fn run(matches: &ArgMatches) {
    let file1 = matches.value_of("FILE1").unwrap();
    let file2 = matches.value_of("FILE2").unwrap();
    let mut lines1 = read_lines(file1).unwrap();
    let mut lines2 = read_lines(file2).unwrap();
    lines1.sort();
    lines2.sort();

    for line in lines1 {
        if !lines2.contains(&line) {
            println!("{}", line);
        }
    }
}

fn main() {
    let matches = App::new("cot")
        .about("A blend of comm(1) and cut(1)")
        // TODO: Add the long about eventually.
        .version(crate_version!())
        .arg(
            Arg::with_name("FILE1")
                .value_name("FILE1")
                .help("the first file to compare")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("FILE2")
                .value_name("FILE2")
                .help("the second file to compare")
                .required(true)
                .index(2),
        )
        .get_matches();

    run(&matches);
}
