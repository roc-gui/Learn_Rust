fn main() {
    // Counter variable
    let mut n = 1;

    // Cycle when `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        // Counter value plus 1
        n += 1;
    }
}
