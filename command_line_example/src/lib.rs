pub mod business {
    use std::error::Error;
    use std::fs;
    use std::env;

    pub struct Config {
        pub query_str: String,
        pub file_name: String,
        pub case_sensitive: bool,
    }

    impl Config {
        /// 使用 panic 是不可取的，因为这个错误是可预期的错误，而非程序性错误
        /// 可以使用 Result() 来进行处理

        // pub fn new(args: &[String]) -> Config {
        //     if args.len() <3 {
        //         panic!("not enough arguments");
        //     }
        //
        //     let query_str = args[1].clone();
        //     let file_name = args[2].clone();
        //
        //     Config {
        //         query_str,
        //         file_name,
        //     }
        // }
        pub fn new(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments");
            }

            let query_str = args[1].clone();
            let file_name = args[2].clone();
            let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); // 获取环境变量（CASE_INSENSITIVE，如果存在则返回true，不存在则返回错误，is_err() 函数用于判单 var() 是否有错误

            // println!("query_str: {}", query_str);
            // println!("file_name: {}", file_name);

            Ok(Config { query_str, file_name, case_sensitive })
        }
    }

    /// 业务逻辑提取到 lib 中，后续方便维护和测试
    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_name)?; // 此处的 ？ 表示如果发生了错误，该函数不进行处理，由 caller 处理
        let results =
            if config.case_sensitive {
                search(config.query_str.as_str(), &contents)
            } else {
                search_case_insensitive(config.query_str.as_str(), &contents)
            };
        for line in results {
            println!("line: {}", line);
        }

        Ok(())
    }

    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut ret = Vec::new();
        for line in contents.lines() {
            if line.contains(query) {
                ret.push(line);
            }
        }
        ret
    }

    pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut ret = Vec::new();
        let query = query.to_lowercase();

        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                ret.push(line);
            }
        }
        ret
    }
}


/// 创建一个测试模块，用于 TDD
#[cfg(test)]
mod tests {
    use super::business::*;

    #[test]
    fn case_sensitive() {
        let contents = "\
Rust:
safe, fast, productive,
Pick three.
        ";
        let query = "duct";

        assert_eq!(vec!["safe, fast, productive,"], search(query, &contents));
    }

    #[test]
    /// 测试：区分大小写
    fn case_insensitive() {
        let contents = "\
Rust:
safe, fast, productive,
Pick three.
Duct tape.
        ";
        let query = "duct";

        assert_eq!(vec!["safe, fast, productive,", "Duct tape."], search_case_insensitive(query, &contents));
    }
}