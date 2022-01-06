use clap::Parser;
use console::{style, Term};
use minigrep::Config;
use std::{env, error::Error};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Sets the query string to search
    #[clap()]
    query: String,

    /// Sets the input file to use
    #[clap()]
    input: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let term = Term::stdout();
    let args = Args::parse();

    let query = args.query;
    let filename = args.input;

    term.write_line(&format!(
        "Parsing input file: {}",
        style(filename.clone()).blue().bold()
    ))?;

    minigrep::run(Config {
        query,
        filename,
        case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
    })?;

    Ok(())
}
