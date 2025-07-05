use std::env;
use std::process;
use minigrep::Config;

fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprint!("Error: {}", err);
            process::exit(1)
    });

    minigrep::run(config).unwrap_or_else(|err| {
        eprint!("Error: {}", err);
            process::exit(1)
    })
}
