fn how_many(value: i32) -> &'static str {
    match value {
        0 => "no",
        1 | 2 => "one or two",
        12 => "dozen",
        9...11 => "lots of",
        _ if (value % 2 == 0) => "some",
        _ => "many",
    }
}

pub fn pm() {
    for x in 0..13 {
        println!("{} I have {} oranges", x, how_many(x));
    }
    let point = (3, 4);
    match point {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y= {}", y),
        (x, 0) => println!("y axis, x ={}", x),
        (x, y) => println!("({},{})", x, y),
    }
}
