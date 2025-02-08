use numru::arr;
use std::f64::consts::{E, PI, TAU};

fn main() {
    let a = arr![42, -17, 256, 3, 99, -8];
    let a_max = a.max().compute();
    println!("a.max() = {:?}", a_max);
    // Note: For 1D arrays, axis doesn't make sense, but this is for demonstration
    let a_max = a.max().axis(0).compute();
    println!("a.max().axis(0) = {:?}", a_max);

    let b = arr![[TAU, -PI, 1.61], [E, 0.98, -7.42], [4.67, -0.45, 8.88]];
    let b_max = b.max().compute();
    println!("b.max() = {:?}", b_max);
    let b_max = b.max().axis(0).compute();
    println!("b.max().axis(0) = {:?}", b_max);
    let b_max = b.max().axis(1).compute();
    println!("b.max().axis(1) = {:?}", b_max);

    let c = arr![
        [[101, 202, 303], [404, 505, 606]],
        [[-707, -808, -909], [111, 222, 333]]
    ];
    let c_max = c.max().compute();
    println!("c.max() = {:?}", c_max);
    let c_max = c.max().axis(0).compute();
    println!("c.max().axis(0) = {:?}", c_max);
    let c_max = c.max().axis(1).compute();
    println!("c.max().axis(1) = {:?}", c_max);
    let c_max = c.max().axis(2).compute();
    println!("c.max().axis(2) = {:?}", c_max);
}
