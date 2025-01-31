use numru::array;
use std::f64::consts::{PI, TAU, E};

fn main() {
    println!("#1 example:");
    let a = array![42, -17, 256, 3, 99, -8];
    println!("a={:?}", a);
    println!("DataType={:?}", a.dtype());
    println!("Shape={:?}", a.shape());

    println!("\n#2 example:");
    let b = array![
        [TAU, -PI, 1.61],
        [E,  0.98, -7.42],
        [4.67, -0.45, 8.88],
    ];
    println!("b={:?}", b);
    println!("Shape={:?}", b.shape());

    println!("\n#3 example:");
    let c = array![
        [[101, 202, 303], [404, 505, 606]],
        [[-707, -808, -909], [111, 222, 333]]
    ];
    println!("c={:?}", c);
    println!("Shape={:?}", c.shape());
}