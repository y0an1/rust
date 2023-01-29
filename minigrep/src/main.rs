use minigrep::{self, Config};
use std::{env, process};

fn main() {
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

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error:{}", e);
        process::exit(1);
    }
}
