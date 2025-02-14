use numru::arr;
use std::f64::consts::{E, PI, TAU};

fn main() {
    // Initialize arrays by directly passing values to the `arr!` macro
    let a = arr![42, -17, 256, 3, 99, -8];
    println!("a.shape() = {:?}", a.shape());
    a.visualize().execute();

    let b = arr![[TAU, -PI, 1.61], [E, 0.98, -7.42], [4.67, -0.45, 8.88]];
    println!("\nb.shape() = {:?}", b.shape());
    b.visualize().decimal_points(1).execute();

    let c = arr![
        [[101, 202, 303], [404, 505, 606]],
        [[-707, -808, -909], [111, 222, 333]]
    ];
    println!("\nc.shape() = {:?}", c.shape());
    c.visualize().execute();

    // Initialize arrays by passing vectors to the `arr!` macro
    let d = arr!(vec![42, -17, 256, 3, 99, -8]);
    println!("\nd.shape() = {:?}", d.shape());
    d.visualize().execute();

    let e = arr!(vec![
        vec![TAU, -PI, 1.61],
        vec![E, 0.98, -7.42],
        vec![4.67, -0.45, 8.88]
    ]);
    println!("\ne.shape() = {:?}", e.shape());
    e.visualize().decimal_points(1).execute();

    let f = arr!(vec![
        vec![vec![101, 202, 303], vec![404, 505, 606]],
        vec![vec![-707, -808, -909], vec![111, 222, 333]]
    ]);
    println!("\nf.shape() = {:?}", f.shape());
    f.visualize().execute();
}
