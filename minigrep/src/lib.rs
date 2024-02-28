use std::{env, fs, io};

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut vec: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // ignore first value;
        vec.next();

        let query = match vec.next() {
            Some(q) => q,
            None => return Err("not enough arguments!"),
        };

        // let query = vec[1].clone();
        let filepath = match vec.next() {
            Some(fp) => fp,
            None => return Err("not enough arguments!"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            filepath: filepath,
            ignore_case: ignore_case,
        })
    }
}

pub fn run(config: &Config) -> Result<(), io::Error> {
    let content = fs::read_to_string(&config.filepath)?;
    let found = if !config.ignore_case {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for line in found {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|l| l.contains(query)).collect()

    // let mut found = Vec::new();

    // for line in content.lines() {
    //     if line.contains(query) {
    //         found.push(line);
    //     }
    // }

    // found
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|l| l.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn search_through_content() {
        let query = "duct";
        let content = "\
        Rust,
        Production server should have rust,
        Duck goes in a Duct,
        I only believe and don't know the future.";

        assert_eq!(
            vec!["        Production server should have rust,"],
            search(&query, &content)
        );
    }
    #[test]
    fn search_case_insensitive_test() {
        let query = "RuSt";
        let content = "\
        Rust,
        Production server should have rust,
        I only believe and don't know the future.";

        assert_eq!(
            vec!["Rust,", "        Production server should have rust,"],
            search_case_insensitive(&query, &content)
        );
    }
}
