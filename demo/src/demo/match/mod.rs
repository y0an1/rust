
#[allow(unused)]
mod demo1 {
    enum Coin {
        Penny,      // 1美分
        Nickel,     // 5美分
        Dime,       // 10美分
        Quarter,    // 25美分
    }

    pub fn main() {
        let coin = Coin::Dime;
        println!("{}", value_in_cents(coin));
    }

    fn value_in_cents (coin: Coin) -> u8 {
        // match 运行模式：
        // 将 coin 变量的值提取出来，依次与花括号内的值进行比对，如果相等则执行 “=>” 内的代码
        return match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        };
    }

}


#[allow(unused)]
mod demo2 {
    /// 绑定值的模式
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

    pub fn main() {
        let coin = Coin::Quarter(UsState::Alaska);
        println!("{}", value_in_cents(coin));
    }

    fn value_in_cents(coin: Coin) -> u8 {
        // match 运行模式：
        // 将 coin 变量的值提取出来，依次与花括号内的值进行比对，如果相等则执行 “=>” 内的代码
        return match coin {
            Coin::Penny => {
                println!("Penny");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {  // 将对应变体的值存放到 state 变量中
                println!("{:?}", state);
                25
            }
        };
    }
}

#[allow(unused)]
mod demo3 {
    use super::demo2::Coin;

    pub fn main() {
        let coin = Coin::Nickel;
        println!("{}", no_match_some_do(coin));
    }

    fn no_match_some_do (coin: Coin) -> u8 {
        // match 运行模式：
        // 将 coin 变量的值提取出来，依次与花括号内的值进行比对，如果相等则执行 “=>” 内的代码
        return match coin {
            Coin::Penny => {
                println!("Penny");
                1
            }
            Coin::Quarter(state) => {  // 将对应变体的值存放到 state 变量中
                println!("{:?}", state);
                25
            }
            _ => 0, // 除了以上的值外，其他都不处理，则使用 “_” 替代
        };
    }
}


#[allow(unused)]
mod demo4 {
    use super::demo2::Coin;

    pub fn main() {
        let mut coin = Coin::Nickel;
        if let Coin::Nickel = coin { // “=” 左边不能写变量，否则将变成赋值表达式
            println!("Nickel");
        } else {
            println!("others"); // 不执行
        }
    }
}



pub fn main() {
    // demo1::main();
    // demo2::main();
    // demo3::main();
    // demo4::main();
}