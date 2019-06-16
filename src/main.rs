fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup = {:?}", tup);

    let (x, y, z) = tup;
    println!("values (x, y, z) = ({}, {}, {})", x, y, z);
    println!("values (x, y, z) = ({}, {}, {})", tup.0, tup.1, tup.2);
}