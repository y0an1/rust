
#[allow(unused)]
mod demo1 {
    /// 定义结构体
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    pub fn main() {
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
}


/// 字段简写，当字段名和变量名是一样时，可以只写一个
#[allow(unused)]
mod demo2 {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    pub fn main() {
        let user = build_user("someone@example.com".to_string(), "someusername123".to_string());
        println!("{:#?}", user);
    }

    fn build_user(email: String, username: String) -> User {
        return User {
            email,
            username,
            active: true,
            sign_in_count: 0,
        };
    }
}

#[allow(unused)]
mod demo3 {
    /// struct 更新语法
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    pub fn main() {
        let user1 = build_user("someone@example.com".to_string(), "someusername123".to_string());

        let user2 = User {
            email: String::from("another@example.com"),
            username: String::from("anotherusername567"),
            ..user1
        };
    }


    fn build_user(email: String, username: String) -> User {
        return User {
            email,
            username,
            active: true,
            sign_in_count: 0,
        };
    }
}


#[allow(unused)]
mod demo4
{
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    /// Tuple struct
    pub fn main() {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }
}

#[allow(unused)]
mod demo5 {
    /// 计算长方形的面积
    struct Rectangle {
        length: u32,
        width: u32,
    }

    /// 使用结构体来进行定义和传参
    pub fn main() {
        // 不使用结构体，width 和 length 的关联性弱
        let width = 10;
        let length = 20;
        println!("{}", area_1(width, length));

        // 使用结构体后，width 和 length 的关联性明确
        let rect = Rectangle {
            width: 20,
            length: 30,
        };
        println!("{}", area_2(&rect));
    }

    fn area_1(width: u32, length: u32) -> u32 {
        return width * length;
    }

    fn area_2(rect: &Rectangle) -> u32 {
        return rect.width * rect.length;
    }
}

#[allow(unused)]
mod demo6 {
    /// struct 方法
    struct Rectangle {
        length: u32,
        width: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            return self.length * self.width;
        }
    }

    pub fn main() {
        let rect = Rectangle {
            width: 20,
            length: 30,
        };
        println!("{}", rect.area());
    }
}

#[allow(unused)]
mod demo7 {
    /// 关联函数
    struct Rectangle {
        length: u32,
        width: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            return self.length * self.width;
        }

        // 创建一个正方形
        fn square(length: u32) -> Rectangle {
            return Rectangle {
                length,
                width: length,
            };
        }
    }

    pub fn main() {
        let square = Rectangle::square(30);
        println!("square area is: {}", square.area());
    }
}

pub fn main() {
    // demo1::main();
    // demo2::main();
    // demo3::main();
    // demo4::main();
    // demo5::main();
    // demo6::main();
    // demo7::main();
}