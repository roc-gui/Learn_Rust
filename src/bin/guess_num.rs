use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut min = 1;
    let mut max = 100;
    loop {
        println!("Please enter your num(from {} to {}): ", min, max);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                min = guess;
            }
            Ordering::Greater => {
                println!("Too big!");
                max = guess;
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
