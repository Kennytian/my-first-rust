// mod nine_nine;
// mod calc;
// mod revert_string;

use std::str::Lines;

use num::complex::Complex;

static REGIONS: [&str; 3] = ["Grüß Gott!", "世界，你好", "World, hello"];

struct Struct {
    e: i32,
}

fn greet_world() {
    // let southern_germany = "Grüß Gott!";
    // let chinese = "世界，你好";
    // let english = "World, hello";
    // let regions = [southern_germany, chinese, english];
    // for region in regions.iter() {
    //     println!("country name: {}", region);
    // }

    // optimized v2
    for region in REGIONS {
        println!("country name: {}", region);
    }
}

fn hard_style() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records: Lines = penguin_data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        if cfg!(debug_assertions) {
            eprintln!("debug {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length)
        }
    }
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn hello_add() {
    let a = 10;
    // 主动指定b的类型为i32
    let b: i32 = 20;
    // 这里有两点值得注意：
    // 1. 可以在数值中带上类型:30i32表示数值是30，类型是i32
    // 2. c是可变的，mut是mutable的缩写
    let c = 30i32;
    // 还能在数值和类型中间添加一个下划线，让可读性更好
    let d = 30_i32;

    let e = add(add(a, b), add(c, d));

    let _y = 1;
    println!("(a + b) + (c + d) = {}", e);
}

fn var_destructure() {
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b={:?}", a, b);

    b = true;
    assert_eq!(a, b);

    b = false;
    assert_ne!(a, b);
}

fn var_assignment() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // .. 表示忽略 0 或多个，_ 表示忽略一个
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    let (a, b, c) = (1, 2, 3); // a = 1, b = 2, c = 3
    println!("a:{} b:{} c:{}", a, b, c);

    let (a, ..) = (1, 2, 3); // a = 1, 2,3 被忽略
    println!("a:{}", a);

    let (.., c) = (1, 2, 3); // c = 3, 1,2 被忽略
    println!("c:{}", c);

    let (a, .., c) = (1, 2, 3, 4, 5); // a = 1, c = 5, 中间的被忽略
    println!("a:{} c:{}", a, c);
}

fn var_type_parse() {
    let guess: i32 = "32".parse().expect("Not a number");
    println!("guess: {}", guess);
    let guess2 = "43".parse::<f32>().expect("Not a number");
    println!("guess2: {}", guess2);
}

fn var_shadowing() {
    let x = 6;
    let x = x + 2;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    // 字符串类型
    let spaces = "   ";
    // usize数值类型
    let spaces = spaces.len();

    println!("{}", spaces);
}

fn var_add() {
    // assert!(0.1 + 0.2 == 0.3);
}

fn for_loop() {
    for i in 1..5 {
        println!("i:{}", i);
    }
}

fn for_loop2() {
    for j in 1..=5 {
        println!("j:{}", j);
    }
}

fn complex_calc() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im);
}

fn mem_size() {
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&x));
}

fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1;
    let y: i32 = y + 1;
    x + y
}

fn express() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}


#[allow(dead_code)]
fn cli_enter() {
    let mut args = std::env::args().skip(1);
    let action = args.next().expect("Please specify an action");
    let item = args.next().expect("Please specify an item");

    println!("{:?}, {:?}", action, item);
}

fn main() {
    println!("Hello, world!");
    greet_world();
    hard_style();
    hello_add();
    var_destructure();
    var_assignment();
    var_shadowing();
    var_type_parse();
    var_add();
    for_loop();
    for_loop2();
    complex_calc();
    mem_size();
    add_with_extra(3, 2);
    express();
    // cli_enter();
}
