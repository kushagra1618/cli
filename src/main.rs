use cli::Reader;
use std::env;

fn main() {
    let args = env::args();

    let reader = Reader::new(args).unwrap_or_else(|err| {
        eprintln!("Problem in parsing arguments: {}", err);
        std::process::exit(1);
    });

    println!("Searching for {}", reader.query);
    println!("In file {}", reader.filename);

    if let Err(e) = Reader::run(reader) {
        eprintln!("Application error:{}", e);
        std::process::exit(1);
    }
}
