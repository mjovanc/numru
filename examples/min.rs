use numru::arr;
use std::f64::consts::{E, PI, TAU};

fn main() {
    let a = arr![42, -17, 256, 3, 99, -8];
    let a_min = a.min().compute();
    println!("a.min() = {:?}", a_min);
    // Note: For 1D arrays, axis doesn't make sense, but this is for demonstration
    let a_min_axis = a.min().axis(0).compute();
    println!("a.min().axis(0) = {:?}", a_min_axis);

    let b = arr![[TAU, -PI, 1.61], [E, 0.98, -7.42], [4.67, -0.45, 8.88]];
    let b_min = b.min().compute();
    println!("b.min() = {:?}", b_min);
    let b_min_axis_0 = b.min().axis(0).compute();
    println!("b.min().axis(0) = {:?}", b_min_axis_0);
    let b_min_axis_1 = b.min().axis(1).compute();
    println!("b.min().axis(1) = {:?}", b_min_axis_1);

    let c = arr![
        [[101, 202, 303], [404, 505, 606]],
        [[-707, -808, -909], [111, 222, 333]]
    ];
    let c_min = c.min().compute();
    println!("c.min() = {:?}", c_min);
    let c_min_axis_0 = c.min().axis(0).compute();
    println!("c.min().axis(0) = {:?}", c_min_axis_0);
    let c_min_axis_1 = c.min().axis(1).compute();
    println!("c.min().axis(1) = {:?}", c_min_axis_1);
    let c_min_axis_2 = c.min().axis(2).compute();
    println!("c.min().axis(2) = {:?}", c_min_axis_2);
}
