use std::env;
use std::process;
use command_line_example::business::{Config, run};


fn main() {
    let args: Vec<String> = env::args().collect();

    // 此处则需要对返回值进一步的判断
    // unwrap_or_else() 内部是一个闭包函数，用于处理错误发生
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("application error: {}", e);
        process::exit(1);
    }
}

