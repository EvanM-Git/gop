use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    let secret_number = rng.random_range(1..10);
    println!("Hello, world! The secret number is: {}", secret_number);
}