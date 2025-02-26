use rand::Rng;

fn generate_password(length: u32, characters_map: &str, rng: &mut impl rand::Rng) -> String {
    let characters_map_length = characters_map.len() as u32;
    let mut password = String::new();
    for _ in 0..length {
        let index = rng.random_range(0..characters_map_length);
        let character = characters_map.chars().nth(index as usize).unwrap();
        password.push(character);
    }
    password
}

fn main() {
    println!("Hello, please give me the characters map you will use:");
    let mut characters_map = String::new();
    let mut rng = rand::rng();
    std::io::stdin().read_line(&mut characters_map).expect("Failed to read line");
    let characters_map = characters_map.trim();
    println!("You will use the following characters map: {}", characters_map);
    println!("Please give me the length of the password you want:");
    let mut length = String::new();
    std::io::stdin().read_line(&mut length).expect("Failed to read line");
    let length: u32 = length.trim().parse().expect("Please type a number!");
    println!("You want a password of length: {}", length);
    let password = generate_password(length, characters_map, &mut rng);
    println!("Your password is: {}", password);
}