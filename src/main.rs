use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("=================");
    println!("Guess the number!");
    println!("=================");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please type your guess between 1 and 100:");
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("The secret number is: {secret_number}");
                break;
            }
        }
    }
}

mod tests {
    #[test]
    fn test_main() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_main2() {
        panic!("This test should fail")
    }
}
