use numru::arr;
use std::f64::consts::{E, PI, TAU};

fn main() {
    println!("#1 example:");
    let a = arr![42, -17, 256, 3, 99, -8];
    println!("a={:?}", a);

    println!("\n#2 example:");
    let b = arr![[TAU, -PI, 1.61], [E, 0.98, -7.42], [4.67, -0.45, 8.88]];
    // let b = arr![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("b={:?}", b);

    // println!("\n#3 example:");
    // let c = arr![
    //     [[101, 202, 303], [404, 505, 606]],
    //     [[-707, -808, -909], [111, 222, 333]]
    // ];
    // println!("c={:?}", c);
    //
    // println!("\n#4 example:");
    // let d = arr![1];
    // println!("d={:?}", d);
}
