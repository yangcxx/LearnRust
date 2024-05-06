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
    let mut b = 30i32;
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
