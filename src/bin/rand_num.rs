use rand::Rng;

fn main() {
    println!("Get a random number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The random number is: {}", secret_number);
}
