fn main() {
    // println!("Hello, world!");
    basic_grammer();
    println!("=======================");
    greet_no_good();
    println!("=======================");
    greet_world();
}

fn basic_grammer() {
    let a = 10;
    // 1. 可以在数值中带上类型:30i32表示数值是30，类型是i32
    // 2. c是可变的，mut是mutable的缩写
    let b = 30i32;

    // 还能在数值和类型中间添加一个下划线，让可读性更好
    let c = 30_i32;

    // 跟其它语言一样，可以使用一个函数的返回值来作为另一个函数的参数
    let d = add(add(a, b), add(b, c));

    // println!是宏调用，看起来像是函数但是它返回的是宏定义的代码块
    // 该函数将指定的格式化字符串输出到标准输出中(控制台)
    // {}是占位符，在具体执行过程中，会把d的值代入进来
    print!("(a+b)+(b+c) = {}", d);
}

fn add(a: i32, b: i32) -> i32 {
    // return a + b;
    // 返回相加值，可以省略return；同时可以没有分号
    a + b
}

fn greet_no_good() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        if cfg!(debug_assertions) {
            eprint!("debug:{:?} -> {:?}", record, fields);
        }
        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            // 输出到标准输出
            println!("{}, {}cm", name, length);
        }
    }
}

fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";

    let regions = [southern_germany, chinese, english];
    // 不能直接进行循环，需要使用迭代器a
    for region in regions.iter() {
        // 不需要输出占位符，编译器自行判定
        println!("{}", &region)
    }
}
