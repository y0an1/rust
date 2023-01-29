/// 更新字符串
#[allow(unused)]
mod demo1 {
    pub fn main() {
        let mut s = String::from("hello");
        s.push_str(",world"); // push_str 是追加一个字符串
        s.push('!'); // push 是追加一个字符
        println!("{}", s);
    }
}

/// 字符串拼接
#[allow(unused)]
mod demo2 {
    pub fn main() {
        let s1 = String::from("Hello, ");
        let s2 = String::from("World!");
        // 字符串拼接运算符 + 左边只能是一个 String 类型，不能是 & 或 字符串切片类型
        let s3 = s1 + &s2;
        println!("{}", s3);
        // 因为 + 运算符实际上调用了类似 fn add(self, s: &str) -> String 的方法，因为第一个参数是 self，所以其所有权被转移了
        // println!("{}", s1);
        println!("{}", s2);

        // format! 格式化宏不会获取其参数的所有权，所以后续可以正常使用参数
        let s1 = String::from("Hello");
        let s2 = String::from("my");
        let s3 = String::from("rust!");
        let s = format!("{} {} {}", s1, s2, s3);
        println!("{}", s);
        println!("{}", s1);
        println!("{}", s2);
        println!("{}", s3);
    }
}

/// 对 String 按索引的形式进行访问
#[allow(unused)]
mod demo3 {
    pub fn main() {
        // 在 rust 中，字符串是无法按索引进行访问，因为 rust 默认是 utf-8 编码
        let s = String::from("Hello world");
        // let c = s[0];
        // error[E0277]: the type `String` cannot be indexed by `{integer}`
        //  --> src/demo/string/mod.rs:50:13
        //    |
        // 50 |     let c = s[0];
        //    |             ^^^^ `String` cannot be indexed by `{integer}`
        //    |
        //    = help: the trait `Index<{integer}>` is not implemented for `String`
    }
}

/// String 的长度
#[allow(unused)]
mod demo4 {
    pub fn main() {
        // 获取到的 length 是根据 Unicode 标量值来的，但如果单个单词无法使用一个字节表示时，其length 是会进行变化的
        let len = String::from("Hola").len();
        println!("{}", len); // 4

        let len = String::from("Здравствуйте").len();
        println!("{}", len); // 24
    }
}

/// 遍历字符串
#[allow(unused)]
mod demo5 {
    pub fn main() {
        let s = "こんにちは";
        // 字节
        for x in s.bytes() {
            println!("{}", x);
        }
        // Unicode 标量值
        for x in s.chars() {
            println!("{}", x);
        }

        // 字形簇在标准库中没有相关api，可以通过第三方库
    }
}

/// 切割字符串
#[allow(unused)]
mod demo6 {
    pub fn main() {
        let s = "Здравствуйте";
        // 因为该字符串切片的标量值对应的是一个标量值对应两个字节，所以此处没有报错
        let mut str = &s[..4];
        println!("{}", str); //Зд

        // str = &s[..3];
        // println!("{}", str);
        // thread 'main' panicked at 'byte index 3 is not a char boundary; it is inside 'д' (bytes 2..4) of `Здравствуйте`'
    }
}

pub fn main() {
    // demo1::main();
    // demo2::main();
    // demo3::main();
    // demo4::main();
    // demo5::main();
    // demo6::main();
}
