use numru::array;

fn main() {
    let a = array![1, 2, 3];
    println!("{:?}", a);

    let b = array![ [1,2], [2,4] ];
    println!("{:?}", b);

    let c = array![
        [[1,1,1], [2,2,2]],
        [[3,3,3], [4,4,4]]
    ];
    println!("{:?}", c);
}