use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("args: {:?}", args);    //args: ["target/debug/command_line_example", "123", "123.txt"]
    let search_content = match args.get(1) {
        None => String::from(""),
        Some(s) => s.to_string(),
    };

    let file_name = match args.get(2) {
        None => String::from(""),
        Some(s) => s.to_string(),
    };

    println!("search_content: {}", search_content);
    println!("file_name: {}", file_name);


    let file_context = fs::read_to_string(file_name).expect("读取文件错误");
    println!("file context: {}", file_context);
}
