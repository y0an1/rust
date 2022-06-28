use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

/// 获取错误信息
#[allow(unused)]
fn demo1() {
    let f = File::open("hello.txt");
    let f = match f {
        // 打开成功，返回文件句柄
        Ok(file) => file,
        // 失败则打印错误信息，退出程序
        Err(error) => {
            panic!("Error opening file {:?}", error)
        }
    };
}

/// 遍历错误信息
#[allow(unused)]
fn demo2() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        // 失败了，再匹配下具体的错误
        Err(error) => match error.kind() {
            // 没有找到文件，则创建文件
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file_create) => file_create,
                Err(e) => panic!("Error creating file {:?}", e),
            },
            // 其他错误，则打印错误信息，并退出程序
            other_error => panic!("Error opening file {:?}", other_error),
        }
    };
}

/// 使用闭包的方式来重写 demo2
#[allow(unused)]
fn demo3() {
    let f = File::open("hello.ttxt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Error creating file {:?}", error);
            })
        } else {
            panic!("Error opening file {:?}", error);
        }
    });
}

/// 使用 unwarp 来重写 demo1
#[allow(unused)]
fn demo4() {
    let f = File::open("hello.txt").unwrap();
}

/// 使用 unwarp 来重写 demo1
#[allow(unused)]
fn demo5() {
    let f = File::open("hello.txt").expect("无法打开 hello.txt 文件");
}

/// 传播错误
#[allow(unused)]
fn demo6() {
    let result = read_string_from_file();
}

fn read_string_from_file() -> Result<String, io::Error> {
    /// 正常传播错误
    // let f = File::open("hello.txt");
    // let mut f = match f{
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    //
    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    /// 使用 ？ 运算符重写上面的代码
    // let mut s = String::new();
    // let mut f = File::open("hello.txt")?;   /// 与 74-77 行的作用是一样的
    // f.read_to_string(&mut s)?;  /// 与 80-83 行的作用是一样的
    // Ok(s)

    /// 使用链式调用再次简化上面代码
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;  /// 与 87-88 行的作用是一样的
    Ok(s)
}


pub fn main() {
    // demo1();
    // demo2();
    // demo3();
    // demo4();
    // demo5();
    demo6();
}
