use std::error::Error;
use std::{env, fs};

#[derive(Debug)]
pub struct InputConfig {
    file_path: String,
    search_pattern: String,
}

impl InputConfig {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<InputConfig, &'static str> {
        args.next();

        let file_path = match args.next() {
            Some(val) => val,
            None => return Err("Didnt get a file path"),
        };

        let search_pattern = match args.next() {
            Some(val) => val,
            None => return Err("Didnt get a search pattern"),
        };

        Ok(InputConfig {
            file_path,
            search_pattern,
        })
    }

    pub fn file_path(&self) -> &String {
        &self.file_path
    }

    pub fn search_pattern(&self) -> &String {
        &self.search_pattern
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let ignore_case = match env::var("IGNORE_CASE") {
        Err(_) => false,
        Ok(val) => val == "1",
    };

    let parsed_query = if ignore_case {
        query.to_lowercase()
    } else {
        query.to_string()
    };

    contents
        .lines()
        .filter(|line| {
            if ignore_case {
                line.to_lowercase()
            } else {
                line.to_string()
            }
            .contains(&parsed_query)
        })
        .collect()
}

pub fn run(conf: InputConfig) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(conf.file_path())?;

    for res_line in search(conf.search_pattern(), &file_contents) {
        println!("{res_line}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast productive.
Pick three.";

        assert_eq!(vec!["safe, fast productive."], search(query, contents));
    }
}
