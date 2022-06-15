// fn main() {
//     // error[E0282]: type annotations needed
//     // 在编译器推导中，42解析成的数字有u32，i32等多个数据类型可以标注
//     // 所以必须手动指定该变量的类型
//     // let guess = "42".parse().expect("Not a number");
//     let guess: u32 = "42".parse().expect("Not a number");
//     println!("guess value: {}", guess);
// }

// Tuple
// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     let (x, y, z) = tup; // 使用模式匹配来解构 Tuple，从而获取到元素的值
//     println!("{}, {}, {}", tup.0, tup.1, tup.2); // 使用点标记法来访问元素的值
//     println!("{}, {}, {}", x, y, z);
// }

// 数组
fn main() {
    // let ary = [1,2,3,4,5,6];
    // let ary2:[i32; 3] = [1,2,3]; // 显式的声明数组类型[type; len]
    // let ary3 =[0;3]; // 相当于 int ary[3] = {0};

    // 数组越界访问，编译器会通过，运行时则报错
    let ary =[1,2,3,4,5,6,7,8,9,10,11,12];
    let index = [12,13,14,15];
    let num = ary[index[1]]; // 此处编译器则检查不到数组下标的具体值
    println!("{}", num);
}