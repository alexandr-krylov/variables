fn main() {
    println!("MAX_POINTS is: {}", MAX_POINTS);
    const MAX_POINTS: u32 = 100_000;
    print();
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
    let spaces = "     ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    let value = 98_222_000;
    println!("value = {}", value);

    let value = 0xff;
    println!("value = {}", value);

    let value = 0o77;
    println!("value = {}", value);

    let value = 0b1111_0000;
    println!("value = {}", value);

    let value = b'A';
    println!("value = {}", value);

    let value:i64 = 9223372036854775807;
    println!("value = {}", value);
}

fn print() {
    println!("MAX_POINTS is: {}", MAX_POINTS);
}

const MAX_POINTS: u32 = 200_000;