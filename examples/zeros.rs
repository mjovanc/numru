use numru::zeros;

fn main() {
    let a = zeros!(i64, 3);
    println!("a.shape() = {:?}", a.shape());
    a.visualize().execute();

    let b = zeros!(f64, 3);
    println!("b.shape() = {:?}", b.shape());
    b.visualize().decimal_points(10).execute();
}
