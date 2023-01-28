/// match 依次匹配
#[allow(unused)]
mod demo1 {
    enum Coin {
        Penny,      // 1美分
        Nickel,     // 5美分
        Dime,       // 10美分
        Quarter,    // 25美分
    }

    fn value_in_cents(coin: Coin) -> u8 {
        // match 运行模式：
        // 将 coin 变量的值提取出来，依次与花括号内的值进行比对，如果相等则执行 “=>” 内的代码
        return match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        };
    }

    pub fn main() {
        let coin = Coin::Dime;
        println!("{}", value_in_cents(coin));
    }
}

/// 绑定值的模式
#[allow(unused)]
mod demo2 {
    #[derive(Debug)]
    pub enum UsState {
        Alabama,
        Alaska,
    }

    #[derive(Debug)]
    pub enum Coin {
        Penny,      // 1美分
        Nickel,     // 5美分
        Dime,       // 10美分
        Quarter(UsState),    // 25美分
    }

    fn value_in_cents(coin: Coin) -> u8 {
        return match coin {
            // 当内部是多行表达式时，要使用花括号
            Coin::Penny => {
                println!("Penny");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            // 将对应变体的值存放到 state 变量中
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            }
        };
    }

    pub fn main() {
        let coin = Coin::Quarter(UsState::Alaska);
        println!("{}", value_in_cents(coin));
    }
}

/// match 必须穷举所有可能，如果要忽略，使用 “_”
#[allow(unused)]
mod demo3 {
    use super::demo2::Coin;

    fn no_match_some_do(coin: Coin) -> u8 {
        return match coin {
            Coin::Penny => {
                println!("Penny");
                1
            }
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            }
            _ => 0,
        };
    }

    pub fn main() {
        let coin = Coin::Nickel;
        println!("{}", no_match_some_do(coin));
    }
}

/// match 匹配 Option<T>
#[allow(unused)]
mod demo4 {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        // let val = match x {
        //     None => 0,
        //     Some(val) => val + 1
        // };
        // Some(val)

        // 书本写法
        match x {
            None => None,
            Some(val) => Some(val + 1),
        }
    }

    pub fn main() {
        let mut x = Some(5);
        x = plus_one(x);
        println!("x: {}", x.expect("none"));
    }
}

/// if let
#[allow(unused)]
mod demo5 {
    pub fn main() {
        // match 是所有可能的匹配模式，而 if let 是单一的匹配模式
        let x = Some(0u8);
        match x {
            Some(3) => println!("three"),
            _ => println!("other")
        }

        // 两者是等价的
        // “=” 左边不能写变量，否则将变成赋值表达式
        if let Some(3) = x {
            println!("Nickel");
        } else {
            println!("other")
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