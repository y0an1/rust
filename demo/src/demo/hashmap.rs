/// 创建 HashMap
#[allow(unused)]
mod demo1 {
    // 在标准库中，需要先导入才可以使用
    use std::collections::HashMap;

    pub fn main() {
        // 单独创建空的 HashMap 时，编译器汇报错，因为无法推导出 key 和 value 的类型，可以手动指明或使用 insert() 方法进行插入数据
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
    }
}

/// 使用 collect 方法来组合成 HashMap
#[allow(unused)]
mod demo2 {
    use std::collections::HashMap;

    pub fn main() {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let intial_scores = vec![10, 50];
        // 调用 iter 创建一个元素类型为 tuple 的数组
        // 调用 zip 将两个数组进行链接
        // 调用 collect 来将其转化成 HashMap
        // “_” 表示使用数组的类型
        let scores: HashMap<_, _> = teams.iter().zip(intial_scores.iter()).collect();
    }
}

/// HashMap 和所有权
#[allow(unused)]
mod demo3 {
    use std::collections::HashMap;

    pub fn main() {
        let mut map = HashMap::new();
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");
        // 当将值赋值给 hashmap 时，会发生移动现象
        map.insert(field_name, field_value);
        // println!("{},{}", field_name, field_value);
        // error[E0382]: borrow of moved value: `field_name`
        //   --> src/demo/hashmap/mod.rs:29:23
        //    |
        // 23 |     let field_name = String::from("Favorite color");
        //    |         ---------- move occurs because `field_name` has type `String`, which does not implement the `Copy` trait
        // ...
        // 27 |     map.insert(field_name, field_value);
        //    |                ---------- value moved here
        // 28 |
        // 29 |     println!("{},{}", field_name, field_value);
        //    |                       ^^^^^^^^^^ value borrowed here after move
        //    |
        //    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
    }
}

/// 访问 HashMap 的值
#[allow(unused)]
mod demo4 {
    use std::collections::HashMap;

    pub fn main() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name);

        match score {
            Some(s) => println!("{}", s),
            None => println!("team not exists"),
        };
    }
}

/// 遍历 HashMap
#[allow(unused)]
mod demo5 {
    use std::collections::HashMap;

    pub fn main() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        // 注意：如果后续要使用 scores 这个变量的话，遍历时要使用引用
        for (k, v) in &scores {
            println!("{}:{}", k, v);
        }
    }
}

/// 更新 HashMap 的值
#[allow(unused)]
mod demo6 {
    use std::collections::HashMap;

    pub fn main() {
        let mut scores = HashMap::new();

        // 1. 已有 key，替换现有的 value
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 50);
        println!("{:?}", scores); //{"Blue": 50}

        // 2. 没有 key，插入 value
        let mut key = "Yellow";
        let entry = scores.entry(key.to_string());
        println!("{:?}", entry); // Entry(VacantEntry("Yellow")) VacantEntry 表示空的，找不到这个 key
        // 当 entry == VacantEntry(...) 时，才会执行 or_insert() 函数
        entry.or_insert(25);
        // 找到了这个 key，所以后续的插入就不执行了
        key = "Blue";
        scores.entry(key.to_string()).or_insert(30);
        println!("{:?}", scores); //{"Blue": 50, "Yellow": 25}

        // 3. 基于现有的 key 进行 value 更新
        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        // split_whitespace 对字符串按空格进行切割
        for word in text.split_whitespace() {
            // 如果这个单词在 map 中不存在，则插入默认值为 0，并返回这个单词对应的 value 的引用
            let count = map.entry(word).or_insert(0);
            // 将 value 进行解引用并加一
            *count += 1;
        }

        println!("{:#?}", map);
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
