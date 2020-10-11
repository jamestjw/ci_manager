use std::env;
use std::process;

use ci_manager::Session;

#[tokio::main]
async fn main() {
    let session = Session::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = ci_manager::run(session) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
