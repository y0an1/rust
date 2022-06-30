
#[allow(unused)]
mod demo1 {
    /// 生命周期 - 避免悬垂引用（dangling reference）
    pub fn main() {
        {
            // let r;
            // {
            //     let x = 5;  // x 的作用域是 6-9行
            //     r = &x;         // 当 r 是在 10 行使用的，而此时已经出了 x 的作用域，所以 r 是一个悬垂引用
            // }
            // println!("r: {}", r);
        }
    }
}

#[allow(unused)]
mod demo2 {
    /// 函数中的泛型生命周期
    pub fn main() {
        let string1 = String::from("abcd");
        let string2 = "zxcv";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    /// 'a 是生命周期参数，用于描述返回值的生命周期的关系
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

#[allow(unused)]
mod demo3 {
    /// 指定生命周期参数的方式依赖于函数所做的事情
    pub fn main() {
        let string1 = String::from("abcd");
        let string2 = "zxcv";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    fn longest<'a>(x: &'a str, y: &str) -> &'a str {
        x   // 该函数只是返回 x 的引用，没有涉及到 y，所以该函数的 y 参数可以不指定生命周期参数
    }
}

#[allow(unused)]
mod demo4 {
    /// 从函数返回引用时，返回类型的生命周期参数需要与其中一个参数的生命周期匹配，如果返回的引用没有指向任何参数，那么它就是悬垂引用
    pub fn main() {
        let string1 = String::from("abcd");
        let string2 = "zxcv";
        // let result = longest(string1.as_str(), string2);
        // println!("The longest string is {}", result);
    }

    // fn longest<'a>(x: &str, y: &str) -> &'a str {
    //     // let result = String::from("abc");
    //     // result.as_str()     // 悬垂引用
    // }
}

#[allow(unused)]
mod demo5 {
    use std::fmt::Display;

    /// Struct 的字段类型是引用
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    pub fn main() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not found a '.'");
        let i = ImportantExcerpt {
            part: first_sentence
        };
    }

    /// 方法定义中的生命周期标注
    impl<'a> ImportantExcerpt<'a> {
        #[allow(unused)]
        fn level(&self) -> i32 {    // 根据省略规则 1，可以不用标注生命周期参数
            3
        }

        #[allow(unused)]
        fn announce_and_return_part(&self, announcement: &str) -> &str {   // 根据省略规则 3，可以不同标注生命周期参数
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    /// 泛型参数类型、Trait Bound、生命周期
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        where T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

pub fn main() {
    // demo1::main();
    // demo2::main();
    // demo3::main();
    // demo4::main();
    // demo5::main();
}