
#[allow(unused)]
mod demo1 {
    /// 更新字符串
    pub fn main() {
        let mut s = String::from("hello");
        s.push_str(",world");
        s.push('!');
        println!("{}", s);
    }
}

#[allow(unused)]
mod demo2 {
    /// 字符串拼接
    pub fn main() {
        let s1 = String::from("Hello, ");
        let s2 = String::from("World!");
        let s3 = s1 + &s2; // 字符串拼接运算符 + 左边只能是一个 String 类型，不能是 & 或 字符串切片类型
        println!("{}", s3);
        // println!("{}", s1);  // 因为 + 运算符实际上调用了类似 fn add(self, s: &str) -> String 的方法，因为第一个参数是 self，所以其所有权被转移了
        println!("{}", s2);
        // error[E0382]: borrow of moved value: `s1`
        //   --> src/demo/string/mod.rs:18:20
        //    |
        // 14 |     let s1 = String::from("Hello, ");
        //    |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
        // 15 |     let s2 = String::from("World!");
        // 16 |     let s3 = s1 + &s2;
        //    |              -- value moved here
        // 17 |     println!("{}", s3);
        // 18 |     println!("{}", s1);
        //    |                    ^^ value borrowed here after move
        //    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

        let s1 = String::from("Hello");
        let s2 = String::from("my");
        let s3 = String::from("rust!");

        let s = format!("{} {} {}", s1, s2, s3);
        println!("{}", s);

        // format! 格式化宏不会获取其参数的所有权，所以后续可以正常使用参数
        println!("{}", s1);
        println!("{}", s2);
        println!("{}", s3);
    }
}

#[allow(unused)]
mod demo3 {
    /// 对 String 按索引的形式进行访问
    pub fn main() {
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

#[allow(unused)]
mod demo4 {
    /// String 的长度
    pub fn main() {
        // 获取到的 length 是根据 Unicode 标量值来的，但如果单个单词无法使用一个字节表示时，其length 是会进行变化的
        let len = String::from("Hola").len();
        println!("{}", len);    // 4

        let len = String::from("Здравствуйте").len();
        println!("{}", len);    // 24
    }
}

#[allow(unused)]
mod demo5 {
    /// 遍历字符串
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

        // 字形簇在标准库中没有相关api
    }
}

#[allow(unused)]
mod demo6{
    /// 切割字符串
    pub fn main() {
        let s = "Здравствуйте";
        let mut str = &s[..4];
        println!("{}", str);     // 因为该字符串切片的标量值对应的是一个标量值对应两个字节，所以此处没有报错
        //Зд

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