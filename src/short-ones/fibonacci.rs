fn main() {
    let (mut x, mut y, mut z) = (0, 1, 0);

    println!("Fibonacci");

    while x < 255 {
        println!("{}", x);

        z = x + y;
        x = y;
        y = z;
    }
}
