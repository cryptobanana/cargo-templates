use log;
use log::LevelFilter::{Debug, Error, Info, Trace};
use simple_logger::SimpleLogger;
use structopt::StructOpt;

mod webapp;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "{{project-name}}",
    about = "A short description of this project.",
    version = concat!(env!("CARGO_PKG_VERSION"), concat!("_", env!("GIT_SHORT_HASH")))
)]
struct Options {
    /// Suppress non-error messages
    #[structopt(short)]
    quiet: bool,

    /// Increase logging verbosity
    #[structopt(short, parse(from_occurrences))]
    verbosity: usize,

    /// Example optional boolean flag
    #[structopt(short, long)]
    some_flag: Option<bool>,

    /// Webserver Port Number
    #[structopt(long, default_value = "8080", env = "BIND_PORT")]
    webserver_port: u16,

    /// Webserver IP Address
    #[structopt(long, default_value = "::1", env = "BIND_ADDRESS")]
    webserver_bind_address: std::net::IpAddr,
}

fn main() {
    let args = Options::from_args();
    let level = match args.quiet {
        true => Error,
        false => vec![Info, Debug, Trace][(args.verbosity).min(2)],
    };

    SimpleLogger::new()
        .with_level(level)
        .init()
        .expect("SimpleLogger instantiation failed.");

    log::info!("Logging with level {}", level);

    webapp::main(args.webserver_bind_address, args.webserver_port).unwrap();
}
