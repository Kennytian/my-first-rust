use std::str::Lines;

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

fn main() {
    println!("Hello, world!");
    greet_world();
    hard_style();
    hello_add();
    var_destructure();
    var_assignment();
    var_shadowing();
    var_type_parse();
}
