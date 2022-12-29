use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1..=100);

    println!("Number Guessing Game");
    loop {
        println!("Enter A number:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read the stdin");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Won!");
                break;
            }
        }
    }
}
