
#[allow(unused)]
mod demo1 {
    /// 创建 vector
    pub fn main() {
        // 使用标准的 new 函数进行创建
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);

        // 使用宏+初始值进行创建
        let mut v = vec![1,2,3,];
    }
}

#[allow(unused)]
mod demo2 {
    /// 读取 vector
    pub fn main() {
        let mut v = vec![1,2,3,4,5,6];
        let third = &v[2];
        println!("The third element is {}" ,third);

        match v.get(2) {
            Some(third) => println!("The third element is {}" ,third),
            None => println!("There is no third element"),
        }
    }
}


#[allow(unused)]
mod demo3 {
    /// 所有权和借用规则
    pub fn main() {
        let mut v = vec![1,2,3,4,5,6];
        let first = &v[0];                          // 此处是一个不可变的引用
        // v.push(7);                                  // 此处是一个可变引用
        println!("The first element is {}", first);       // 此处是一个不可变的引用
        // 根据所有权的借用规则，同一个作用域内不能同时存在可变和不可变的引用，编译器抛出错误
        // 出现此问题的情况：当执行第 33 行时，有可能内存中没有那么大的空间，需要重新申请，从而导致 32 行变成一个悬空引用
    }
}

#[allow(unused)]
mod demo4 {
    /// 遍历 vector
    pub fn main() {
        let v = vec![1, 2, 3, 4];
        for i in &v {
            println!("{}", i);
        }

        for (i, item) in v.iter().enumerate() {
            println!("enumerate: i: {}, item: {}", i, item);
        }
    }
}

#[allow(unused)]
mod demo5 {
    /// 更改 vector 的值
    pub fn main() {
        let mut v = vec![1, 2, 3, 4];
        for i in &mut v {
            *i += 10;
        }

        for x in &v {
            println!("x: {}", x);
        }
    }
}


#[allow(unused)]
mod demo6 {
    /// 存放多多种数据类型
    #[derive(Debug)]
    enum MultTypeEnum {
        Int(i32),
        Float(f64),
        Text(String),
    }

    pub fn main() {
        let v = vec![
            MultTypeEnum::Float(3.14),
            MultTypeEnum::Int(8),
            MultTypeEnum::Text(String::from("hello")),
        ];

        println!("{:?}", v);    // [Float(3.14), Int(8), Text("hello")]
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