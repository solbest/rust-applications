use rand::Rng;

fn main() {
    println!("Generatin secret number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is : {secret_number}")
}
