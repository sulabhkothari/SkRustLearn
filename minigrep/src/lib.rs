use std::env;
use std::fs;
use std::process;
use std::error::Error;
use std::str::FromStr;

// CASE_INSENSITIVE=1 cargo run to poem.txt

// For now, just know that Box<dyn Error> means the function will return a type that implements the
// Error trait, but we don’t have to specify what particular type the return value will be. This gives
// us flexibility to return error values that may be of different types in different error cases.
// The dyn keyword is short for “dynamic.”
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        // We’re using the is_err method on the Result to check whether it’s an error and therefore
        // unset, which means it should do a case-sensitive search. If the CASE_INSENSITIVE environment
        // variable is set to anything, is_err will return false and the program will perform a
        // case-insensitive search. We don’t care about the value of the environment variable, just
        // whether it’s set or unset, so we’re checking is_err rather than using unwrap, expect, or
        // any of the other methods we’ve seen on Result.
        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        if args.len() == 4 {
            case_sensitive = bool::from_str(&args[3][..]).unwrap();
        }

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
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

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
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