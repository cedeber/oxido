use clap::clap_app;
use console::style;
use console::Term;
use minigrep::Config;
use std::{env, error::Error};

// Use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() -> Result<(), Box<dyn Error>> {
    let term = Term::stdout();

    let matches = clap_app!(gpxtools =>
        (name: "Minigrep")
        (version: "0.1.0-beta.1")
        (author: "Cédric Eberhardt <hello+code@cedeber.fr>")
        (about: "The toolbox for searching in files")
        (@arg QUERY: +required "Sets the query string to search")
        (@arg INPUT: +required "Sets the input file to use")
        // (@arg verbose: -v --verbose "Print information verbosely")
    )
    .get_matches();

    let query = matches.value_of("QUERY").unwrap();
    let filename = matches.value_of("INPUT").unwrap();

    term.write_line(&format!(
        "Parsing input file: {}",
        style(filename).blue().bold()
    ))?;

    minigrep::run(Config {
        query: query.to_string(),
        filename: filename.to_string(),
        case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
    })?;

    Ok(())
}
