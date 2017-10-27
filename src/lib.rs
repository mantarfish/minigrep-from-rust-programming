use std::error::Error;
use std::fs::File;
use std::io::Read;


pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let query = config.query;
    let contents = contents;

    let result: Vec<&str>;
    if config.case_sensitive {
        result = search(&query, &contents);
    } else {
        result = search_case_insensitive(&query, &contents);
    }

    for i in result {
        println!("\n{:?}\n", i);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    return results;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    return results;
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
        let filename = args[2].clone();
        let query = args[1].clone();
        let mut case_sensitive = false;
        if args.len() == 4 {
            if args[3] == "c" {
                case_sensitive = true;
            }
        }

        Ok(Config {filename, query, case_sensitive})
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
            rust:\n\
            safe, fast, productive.\n\
            pick three.\n\
        ";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn muti_lines() {
        let query = "e";
        let contents = "\
safe, fast, productive.
ee
eeew
        ";
        assert_eq!(
            vec!["safe, fast, productive.", "ee", "eeew"],
            search(query, contents)
        )
    }

    #[test]
    fn case_insensitive_search() {
        let query = "E";
        let contents = "e";
        assert_eq!(vec!(contents), search_case_insensitive(query, contents))
    }
}
