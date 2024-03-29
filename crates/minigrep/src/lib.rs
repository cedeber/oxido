use console::Term;
use regex::Regex;
use std::error::Error;
use std::fs;

pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let term = Term::stdout();
	let contents = fs::read_to_string(config.filename)?;

	let results = if config.case_sensitive {
		search(&config.query, &contents)
	} else {
		search_case_insensitive(&config.query, &contents)
	};

	for line in results {
		term.write_line(line)?;
	}

	Ok(())
}

/// Search
///
/// # Examples
///
/// ```
/// use minigrep::search;
/// search("en", "Then");
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let re = Regex::new(query).unwrap();

	contents
		.lines()
		// .filter(|line| line.contains(query))
		.filter(|line| re.find(line).is_some())
		.collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.to_lowercase().contains(&query) {
			results.push(line);
		}
	}

	results
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}

	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

		assert_eq!(
			vec!["Rust:", "Trust me."],
			search_case_insensitive(query, contents)
		);
	}
}
