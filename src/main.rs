use std::{
    fs,
    io::{self, Write},
};

// (Buf) Uncomment these lines to have the output buffered, this can provide
// better performance but is not always intuitive behaviour.
// use std::io::BufWriter;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "{{project-name}}",
    about = "",
    version = concat!(env!("CARGO_PKG_VERSION"), concat!("_", env!("GIT_SHORT_HASH")))
)]
struct Options {
    // The pattern we want to look for.
    pattern: String,
    // The path of the file we want to look at.
    path: String,
}

fn main() {
    let args = Options::from_args();
    let contents = fs::read_to_string(&args.path).expect("Could not read file.");
    let mut stdout = io::stdout();
    // (Buf) Wraps stdout in a buffer.
    // let mut stdout = BufWriter::new(stdout);

    for (line_no, line) in contents.lines().enumerate() {
        if line.contains(&args.pattern) {
            writeln!(stdout, "{}: {}", line_no + 1, line);
        }
    }
}
