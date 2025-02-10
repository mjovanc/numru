use numru::zeros;

fn main() {
    let a = zeros!(i64, 3);
    println!("a.shape() = {:?}", a.shape());
    a.visualize();

    let b = zeros!(f64, 3);
    println!("b.shape() = {:?}", b.shape());
    b.visualize();
}
