use std::mem;
mod pattern_matching;

enum Color {
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8),
}

struct Point {
    x: i32,
    y: i32,
}

struct Line {
    a: Point,
    b: Point,
}

fn use_slices(slice: &[u8]) {
    println!("first elem={},len={}", slice[0], slice.len());
}

fn main() {
    pattern_matching::pm();
    //struct
    let line = Line {
        a: Point { x: 0, y: 0 },
        b: Point { x: 10, y: 10 },
    };
    println!(
        "Line @:({},{}):({},{})",
        line.a.x, line.a.y, line.b.x, line.b.y
    );

    //enum

    let c = Color::RGB(0, 0, 0);

    let clr = match c {
        Color::Red => "RED",
        Color::Blue => "BLUE",
        Color::Green => "GREEN",
        Color::RGB(0, 0, 0) => "BLACK",
        _ => "Undefined",
    };
    println!("{}", clr);

    //Option<T> Value

    let x = 3.0;
    let y = 2.0;
    let result = if y != 0.0 { Some(x / y) } else { None };
    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("Division by Zero"),
    }

    // Arrays

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!(
        "elem at [0] ={}, size of the array is {}",
        arr[0],
        arr.len()
    );
    arr[0] = 3;
    println!("elem at 0 has been changed to {}", arr[0]);
    println!("{:?}", arr);

    if arr != [1, 2, 3, 4, 5] {
        println!("not equal to [1,2,3,4,5]")
    }
    // bulk array
    let newArr = [1u8; 10];
    for i in newArr.iter() {
        print!("{}", i);
    }
    print!("\n[");
    for i in 0..newArr.len() {
        print!("{}", newArr[i]);
    }
    print!("]\n");
    println!("size of newArray is {} bytes", mem::size_of_val(&newArr));

    //matrix (arrays of arrays)
    let mtx: [[u8; 3]; 3] = [[1, 2, 3], [1, 2, 3], [1, 2, 3]];
    println!("matrix {:?}", mtx);
    println!("Diagonal value!");

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("[{}],[{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
    //Slices

    let mut data: [u8; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", data);
    use_slices(&data[1..3]);

    //tuples
    let x = 1;
    let y = 2;

    let sp = sum_and_prod(x, y);

    println!(
        "sum and prod of {0},{1} => {0}+{1}={2} - {0}*{1}={3}",
        x, y, sp.0, sp.1
    );
}

fn sum_and_prod(vala: u8, valb: u8) -> (u8, u8) {
    (vala + valb, vala * valb)
}
