use numru::zeros;

fn main() {
    let a = zeros!(i64, 3);
    println!("a.shape() = {:?}", a.shape());
    a.visualize().execute();

    let b = zeros!(f64, 3);
    println!("b.shape() = {:?}", b.shape());
    b.visualize().decimal_points(10).execute();

    let c = zeros!(f64, 2, 3);
    println!("c.shape() = {:?}", c.shape());
    c.visualize().decimal_points(2).execute();

    let d = zeros!(i64, 2, 3, 4);
    println!("d.shape() = {:?}", d.shape());
    d.visualize().execute();
}
