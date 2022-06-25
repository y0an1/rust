
// 有一个函数，它接受字符串作为参数，返回它在这个字符串里找到的第一个单词，如果函数没找到任何空格，那么整个字符串就被返回


fn _demo_1(){
    let mut s = String::from("hello world");
    let idx = _first_word(&s);

    s.clear();  // 该例子中，idx 的值是与 s 进行绑定，如果 s 的值在后面被释放了，而仍然使用 idx 去获取 s 的空格，则会有 bug

    println!("space idex: {}", idx);
}

//  目前实现的功能是： 找到字符串中的空格后，返回该空格的下标；没有则返回字符串长度
fn _first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // 普通的遍历
    // let mut idx = 0;
    // for x in bytes {
    //     if *x == b' ' { // x 是一个 u8 类型的引用，所以此处需要进行解引用
    //         return idx;
    //     }
    //     idx += 1;
    // }

    // 使用 iter 遍历， 返回的是一个元组，可以用模式匹配，i 是索引，item 是引用项
    for (i, &item)  in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return bytes.len();
}


// 字符串切片
fn _demo_2() {
    let s = String::from("Hello World");

    // 切片是从下标 0 开始，到下标 4 结束，不包含下标 5
    let hello = &s[0 .. 5];
    let world = &s[6 .. 11];

    println!("{}, {}", hello, world);
}


// 使用字符串切片的功能完善 demo1
fn _demo_3() {
    let mut _s = String::from("hello world");
    let slice = _first_word_str_slice(&_s);
    // s.clear();
    println!("{}", slice);
    /*
    error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
      --> src/demo/slice/_struct:54:5
       |
    53 |     let slice = _first_word_str_slice(&s);
       |                                       -- immutable borrow occurs here
    54 |     s.clear();
       |     ^^^^^^^^^ mutable borrow occurs here
    55 |     println!("{}", slice);
       |                    ----- immutable borrow later used here
    */
}

fn _first_word_str_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

// 使用字符串切片作为函数参数，从而提高灵活性
fn _demo_4() {
    let my_string = String::from("hello world");
    let word_index = _first_word_str_slice_param(&my_string[..]);   // 创建一个完整的字符串切片
    println!("{}", word_index);

    let my_string_literal = "hello world";
    let word_index = _first_word_str_slice_param(my_string_literal);
    println!("{}", word_index);
}

fn _first_word_str_slice_param(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

pub fn main() {
    // _demo_1();
    // _demo_2();
    // _demo_3();
    _demo_4();
}