macro_rules! numin {
    () => {{
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().parse().unwrap()
    }};
}

fn main() {
    let num1: i32 = numin!();
    let num2: i32 = numin!();
    let num = num1 + num2;
    println!(
        "{:>width$}+{:>width$}={:>width$}",
        num1,
        num2,
        num,
        width = 6
    );
}
