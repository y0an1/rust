use std::{error::Error, fs};

#[derive(Debug)]
pub struct Config {
    query: String,
    file_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("长度小于 3");
        }

        let query = &args[1];
        let file_name = &args[2];

        Ok(Config {
            query: query.clone(),
            file_name: file_name.clone(),
        })
    }

    pub fn file_name(&self) -> &String {
        &self.file_name
    }

    pub fn query(&self) -> &String {
        &self.query
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name())?;

    let result = search(&config.query, &contents);
    for ele in result {
        println!("{}", ele);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut vec = Vec::new();
    for ele in contents.lines().into_iter() {
        if ele.contains(query) {
            vec.push(ele);
        }
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "333";
        let contents = "\
111 aaa
222 bbb
333 ccc
444 ddd";

        assert_eq!(vec!["333 ccc"], search(query, contents));
    }
}
