use std::{error::Error, fs,env, process};

#[derive(Debug)]
pub struct Config {
    query: String,
    file_name: String,
}

impl Config {
    // 优化前
    // pub fn new(args: &[String]) -> Result<Config, &'static str> {
    //     if args.len() < 3 {
    //         return Err("长度小于 3");
    //     }

    //     let query = &args[1];
    //     let file_name = &args[2];

    //     Ok(Config {
    //         query: query.clone(),
    //         file_name: file_name.clone(),
    //     })
    // }

    // 使用迭代器
    // 这里加上 mut 是因为迭代器是可变的
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); // 这里调用 next 是因为不需要它的路径

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("没有输入搜索字符串"),
        };
        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("没有输入文件名"),
        };

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
    // 优化前
    // let mut vec = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         vec.push(line);
    //     }
    // }
    // vec

    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
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

pub fn main() {
    // 优化前的处理
    // let args: Vec<String> = env::args().collect();
    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("解析参数错误:{}", err);
    //     process::exit(1);
    // });

    // 利用迭代器来获取参数
    let config = Config::new( env::args()).unwrap_or_else(|err| {
        eprintln!("解析参数错误:{}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error:{}", e);
        process::exit(1);
    }
}
