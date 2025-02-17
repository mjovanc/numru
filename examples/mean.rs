use numru::arr;
use std::f64::consts::{E, PI, TAU};

fn main() {
    let a = arr![42, -17, 256, 3, 99, -8];
    let a_mean = a.mean().compute();
    println!("a.mean() = {:?}", a_mean);
    // Note: For 1D arrays, axis doesn't make sense, but this is for demonstration
    let a_mean_axis = a.mean().axis(0).compute();
    println!("a.mean().axis(0) = {:?}", a_mean_axis);

    let b = arr![[TAU, -PI, 1.61], [E, 0.98, -7.42], [4.67, -0.45, 8.88]];
    let b_mean = b.mean().compute();
    println!("b.mean() = {:?}", b_mean);
    let b_mean_axis_0 = b.mean().axis(0).compute();
    println!("b.mean().axis(0) = {:?}", b_mean_axis_0);
    let b_mean_axis_1 = b.mean().axis(1).compute();
    println!("b.mean().axis(1) = {:?}", b_mean_axis_1);

    let c = arr![
        [[101, 202, 303], [404, 505, 606]],
        [[-707, -808, -909], [111, 222, 333]]
    ];
    let c_mean = c.mean().compute();
    println!("c.mean() = {:?}", c_mean);
    let c_mean_axis_0 = c.mean().axis(0).compute();
    println!("c.mean().axis(0) = {:?}", c_mean_axis_0);
    let c_mean_axis_1 = c.mean().axis(1).compute();
    println!("c.mean().axis(1) = {:?}", c_mean_axis_1);
    let c_mean_axis_2 = c.mean().axis(2).compute();
    println!("c.mean().axis(2) = {:?}", c_mean_axis_2);
} 