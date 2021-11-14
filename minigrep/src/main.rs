use clap::{App, Arg};
use console::style;
use console::Term;
use minigrep::Config;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let term = Term::stdout();

    let matches = App::new("Minigrep")
        .version("0.1.0-beta.1")
        .author("Cédric Eberhardt <hello+code@cedeber.fr>")
        .about("The toolbox for searching in files")
        .arg(
            Arg::new("QUERY")
                .required(true)
                .about("Sets the query string to search"),
        )
        .arg(
            Arg::new("INPUT")
                .required(true)
                .about("Sets the input file to use"),
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
