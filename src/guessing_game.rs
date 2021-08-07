use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("\nGuess a number b/w [1-100]");
    println!("{}", "You will have 7 tries at max.".cyan());

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut tries: i32 = 7;

    // println!("The secret number is: {}", secret_number);
    loop {
        if tries == 0 {
            println!("{}", "U Loooser".red().bold());
            break;
        }
        println!("Please input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                tries -= 1;
                num
            }
            Err(_) => continue,
        };

        print!("You guessed: {} ", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", " [ Too small! ]".red()),
            Ordering::Greater => println!("{}", " [ Too big! ]".red()),
            Ordering::Equal => {
                println!("\n{}", "Hurray, You win!".green());
                break;
            }
        }
        println!(
            "{} {} {}",
            "You have".yellow(),
            tries,
            "tries left.".yellow()
        );
    }
}
