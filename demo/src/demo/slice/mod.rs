
// 有一个函数，它接受字符串作为参数，返回它在这个字符串里找到的第一个单词，如果函数没找到任何空格，那么整个字符串就被返回


fn _demo_1(){
    let mut s = String::from("hello world");
    let idx = _first_word(&s);

    s.clear();  // 该例子中，idx 的值是与 s 进行绑定，如果 s 的值在后面被释放了，而仍然使用 idx 去获取 s 的空格，则会有 bug

    println!("space idex: {}", idx);
}

//  目前实现的功能是： 找到字符串中的空格后，返回该空格的下标；没有则返回字符串长度
fn _first_word(str: &String) -> usize {
    let bytes = str.as_bytes();
    let mut idx = 0;
    for x in bytes {
        if *x == b' ' { // x 是一个 u8 类型的引用，所以此处需要进行解引用
            return idx;
        }
        idx += 1;
    }

    return bytes.len();
}



pub fn main() {
    _demo_1();
}