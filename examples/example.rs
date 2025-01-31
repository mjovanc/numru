use numru::array;
use std::f64::consts::{PI, TAU, E};

fn main() {
    let a = array![42, -17, 256, 3, 99, -8];
    println!("{:?}", a);

    let b = array![
        [TAU, -PI, 1.61],
        [E,  0.98, -7.42],
        [4.67, -0.45, 8.88],
    ];
    println!("{:?}", b);

    let c = array![
        [[101, 202, 303], [404, 505, 606]],
        [[-707, -808, -909], [111, 222, 333]]
    ];
    println!("{:?}", c);
}