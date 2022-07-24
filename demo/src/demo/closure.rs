/// 闭包的定义最终只会为参数、返回值推断出 **唯一** 具体的类型
mod demo1 {
    #[allow(unused)]
    pub fn main() {
        let closure = |x| x;    // 在闭包被定义时，编译器无法推导出闭包的类型
        let s = closure(String::from("hello")); // 在调用闭包时，传入了 String 类型，此时，编译器就可以推导出闭包类型了
        // let n = closure(5); // 当一个闭包只能处理一个类型，其不支持泛型处理，即：当编译器为闭包推导出类型后，该闭包只能处理该类型的数据
        // error[E0308]: mismatched types
        //  --> src\demo\closure.rs:8:25
        //   |
        // 8 |         let n = closure(5); //
        //   |                         ^- help: try using a conversion method: `.to_string()`
        //   |                         |
        //   |                         expected struct `String`, found integer
    }
}

// 这是一个模拟执行复杂算法的 demo，使用函数的方式，后续改为使用闭包的形式来优化
mod demo2 {
    use std::thread;
    use std::time::Duration;

    pub fn main() {
        let simulated_user_specified_value = 10;
        let simulated_random_number = 7;
        generate_workout(simulated_user_specified_value, simulated_random_number);
    }

    // 这个函数是用于模拟复杂算法
    // fn simulated_expensive_calculation(intensity: u32) -> u32 {
    //     println!("calculating slowly ...");
    //     thread::sleep(Duration::from_secs(2));
    //     intensity
    // }

    fn generate_workout(intensity: u32, random_number: u32) {
        // 使用闭包的方法进行优化， 闭包的作用就是仅需要的时候才调用，如果不需要则不会被调用
        // 一般情况下，闭包都不需要手动标注类型，编译器会自动推导出来，但也可以手动标注
        let expensive_closure = |num| -> u32{
            println!("calculating slowly ...");
            thread::sleep(Duration::from_secs(2));
            num
        };

        // 当程序未运行时，闭包返回的是单元类型，而单元类型是没有实现 Display 这个 trait 的， 所以会有红线。当使用了‘{:?}’ 格式化后，则不再强制要求实现 Display 这个 trait
        // 当手动对返回值进行标注时，因为 u32 是基本类型，已经实现了 Display 这个 trait 了
        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_closure(intensity));
            println!("Next, do {} situps!", expensive_closure(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!("Today, run for {} minutes!", expensive_closure(intensity));
            }
        }

        /*
        // simulated_expensive_calculation() 在此处要被调用两次，存在资源浪费
        // 方案一的优化是将该函数的结果保存，直接使用该结果即可
        // 此方案存在问题： 本来 random_number == 3 是不需要调用，但是现在该函数确要被调用了
        let expensive_result = simulated_expensive_calculation(intensity);
        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_result);
            println!("Next, do {} situps!", expensive_result);
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            }else {
                println!("Today, run for {} minutes!", expensive_result);
            }
        }
        */

        /*
         if intensity < 25 {
            println!("Today, do {} pushups!", simulated_expensive_calculation(intensity));
            println!("Next, do {} situps!", simulated_expensive_calculation(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            }else {
                println!("Today, run for {} minutes!", simulated_expensive_calculation(intensity));
            }
        }
        */
    }
}

// 闭包的唯一性
mod demo3 {
    pub fn main() {
        let example_closure = |x| x;
        // 此处是没有问题的，因为编译器已经为闭包绑定为 String 类型了
        let s = example_closure(String::from("hello"));

        // 这条语句则会出现 panic
        // 因为闭包已经被编译器绑定为了 String 类型，当传递给 int 类型时，该闭包是无法处理的
        // let n = example_closure(5);
    }
}


pub fn main() {
    // demo1::main();
    // demo2::main();
    demo3::main();
}