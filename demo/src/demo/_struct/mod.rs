struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn _demo_1() {
    // 当实例化结构体时， 有多少个字段就必须定义多少个
    let user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 当要对实例化结构体中的字段进行修改，必须先申明为可变，且结构体为可变后，其内部所有字段都是可变的
    // user.email = String::from("anotheremail@example.com");

    let mut user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user.email = String::from("anotheremail@example.com");
}


// 字段简写，当字段名和变量名是一样时，可以只写一个
fn _demo_2() {
    let user = build_user("someone@example.com".to_string(), "someusername123".to_string());
}

fn build_user(email: String, username: String) -> User {
    return User {
        email,
        username,
        active: true,
        sign_in_count: 0,
    };
}

// struct 更新语法
fn _demo_3() {
    let user1 = build_user("someone@example.com".to_string(), "someusername123".to_string());

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
}

// Tuple struct
fn _demo_4() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// 计算长方形的面积
struct Rectangle {
    length: u32,
    width: u32,
}

fn _demo_5() {
    // 不使用结构体，width 和 length 的关联性弱
    let width = 10;
    let length = 20;

    let mut area = rectangle_area_1(width, length);

    // 使用结构体后，width 和 length 的关联性明确
    let rect = Rectangle{
        width: 20,
        length: 30,
    };
    area = rectangle_area_2(&rect);

    println!("{}", area);
}

fn rectangle_area_1(width: u32, length: u32) -> u32 {
    return width * length;
}

fn rectangle_area_2(rect: &Rectangle) -> u32 {
    return rect.width * rect.length;
}


pub fn main() {
    // _demo_1();
    // _demo_2();
    // _demo_3();
    // _demo_4();
    _demo_5();
}