
#[allow(unused)]
enum Coin {
    Penny,      // 1美分
    Nickel,     // 5美分
    Dime,       // 10美分
    Quarter,    // 25美分
}

#[allow(unused)]
fn demo1() {
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

#[derive(Debug)]
#[allow(unused)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(unused)]
enum Coin_ {
    Penny,      // 1美分
    Nickel,     // 5美分
    Dime,       // 10美分
    Quarter(UsState),    // 25美分
}

/// 绑定值的模式
#[allow(unused)]
fn demo2() {
    let coin = Coin_::Quarter(UsState::Alaska);
    println!("{}", value_in_cents_(coin));
}

fn value_in_cents_ (coin: Coin_) -> u8 {
    // match 运行模式：
    // 将 coin 变量的值提取出来，依次与花括号内的值进行比对，如果相等则执行 “=>” 内的代码
    return match coin {
        Coin_::Penny => {
            println!("Penny");
            1
        }
        Coin_::Nickel => 5,
        Coin_::Dime => 10,
        Coin_::Quarter(state) => {  // 将对应变体的值存放到 state 变量中
            println!("{:?}", state);
            25
        }
    };
}


#[allow(unused)]
fn demo3() {
    let coin = Coin_::Nickel;
    println!("{}", no_match_some_do(coin));
}

fn no_match_some_do (coin: Coin_) -> u8 {
    // match 运行模式：
    // 将 coin 变量的值提取出来，依次与花括号内的值进行比对，如果相等则执行 “=>” 内的代码
    return match coin {
        Coin_::Penny => {
            println!("Penny");
            1
        }
        Coin_::Quarter(state) => {  // 将对应变体的值存放到 state 变量中
            println!("{:?}", state);
            25
        }
        _ => 0, // 除了以上的值外，其他都不处理，则使用 “_” 替代
    };
}

#[allow(unused)]
fn demo4() {
    let mut coin = Coin_::Nickel;
    if let Coin_::Nickel = coin { // “=” 左边不能写变量，否则将变成赋值表达式
        println!("Nickel");
    } else {
        println!("others"); // 不执行
    }
}

pub fn main() {
    // demo1();
    // demo2();
    // demo3();
    // demo4();
}