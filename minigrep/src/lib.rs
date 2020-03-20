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
    //pub fn new(args: &[String]) -> Result<Config, &'static str> {
    //  Because we’re taking ownership of args and we’ll be mutating args by iterating over it, we
    // can add the mut keyword into the specification of the args parameter to make it mutable.
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        //if args.len() < 3 {
        //    return Err("not enough arguments");
        //}

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name")
        };

        //let query = args[1].clone();
        //let filename = args[2].clone();

        // We’re using the is_err method on the Result to check whether it’s an error and therefore
        // unset, which means it should do a case-sensitive search. If the CASE_INSENSITIVE environment
        // variable is set to anything, is_err will return false and the program will perform a
        // case-insensitive search. We don’t care about the value of the environment variable, just
        // whether it’s set or unset, so we’re checking is_err rather than using unwrap, expect, or
        // any of the other methods we’ve seen on Result.
        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        case_sensitive = match args.next() {
            Some(arg) => {
                //println!("***********************{}", arg);
                bool::from_str(&arg[..]).unwrap()
            },
            None => case_sensitive

        };

        //if args.len() == 4 {
        //    case_sensitive = bool::from_str(&args[3][..]).unwrap();
        //}

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //let mut results = Vec::new();

    //for line in contents.lines() {
    //    if line.contains(query) {
    //        results.push(line);
    //    }
    //}

    //results

    // We can write this code in a more concise way using iterator adaptor methods. Doing so also
    // lets us avoid having a mutable intermediate results vector. The functional programming style
    // prefers to minimize the amount of mutable state to make code clearer. Removing the mutable
    // state might enable a future enhancement to make searching happen in parallel, because we
    // wouldn’t have to manage concurrent access to the results vector.
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //let query = query.to_lowercase();
    //let mut results = Vec::new();

    //for line in contents.lines() {
    //    if line.to_lowercase().contains(&query) {
    //        results.push(line);
    //    }
    //}

    //results
    contents.lines()
        .filter(|s| s.to_lowercase().contains(&query.to_lowercase()))
        .collect()
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