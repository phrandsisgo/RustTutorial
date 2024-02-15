use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn print_type_of<T>(_: T) -> (&'static str, &'static str) {
    ("{}", std::any::type_name::<T>())
}

fn main() {
    guessing_game();
}

fn guessing_game() {
    println!("Guess the number!");
    let secret_number: i32 = rand::thread_rng().gen_range(1, 101);
    println!("the secret number is: {}", secret_number);

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: i32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    println!("the type is {:?}", print_type_of(guess));
}