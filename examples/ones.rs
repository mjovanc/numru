use std::f64::consts::{E, PI, TAU};

use numru::{arr, ones};

fn main() {
    let a = ones!(i64, 3);
    println!("a.shape() = {:?}", a.shape());
    a.visualize().execute();

    let b = ones!(f64, 3);
    println!("b.shape() = {:?}", b.shape());
    b.visualize().decimal_points(10).execute();

    let c = ones!(f64, 2, 3);
    println!("c.shape() = {:?}", c.shape());
    c.visualize().decimal_points(2).execute();

    let mut d = arr![[TAU, -PI, 1.61], [E, 0.98, -7.42], [4.67, -0.45, 8.88]];
    d.ones();
    println!("d.shape() = {:?}", d.shape());
    d.visualize().execute();

    let mut e = arr![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    e.ones();
    println!("e.shape() = {:?}", e.shape());
    e.visualize().execute();
}
