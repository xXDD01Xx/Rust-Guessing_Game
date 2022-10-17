use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main()
{
    guess_and_compare();
}


fn guess_and_compare()
{
    println!("Welcome, Guess the Number Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}!");           //Debug

    loop
    {
        println!("Please input your guess between 1 & 100...");

        let mut user_guess = String::new();

        io::stdin().read_line(&mut user_guess)                          //same as 'std::io::Stdin'
            .expect("Failed to read user input!");

        let user_guess: u32 =
            match user_guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {user_guess}");

        match user_guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal =>
                {
                    println!("You guessed right!");
                    break;
                }
        }
    }
}


