// Goal:
// Write a program to play the higher or lower guessing game.
// 1. The program generates a random number between 1 and 100.
// 2. The program prompts the user to guess the number.
// 3. The program tells the user if their guess was too high, too low or correct.
// 4. Repeat steps 2 and 3 until the user guesses the correct number.

// Bonus:
// 1. Make the program keep track of the number of guesses the user has made.
// 2. After each game, ask the user if they want to play again.

use rand::prelude::*;
use std::io;

fn main() {
    let mut buffer = String::new();
    let mut number = 0;
    let mut guess = 0;
    let mut count = 0;
    let mut play = true;

    while play {
        number = thread_rng().gen_range(1..101);
        println!("Guess a number between 1 and 100: ");
        loop {
            io::stdin().read_line(&mut buffer);
            guess = buffer.trim().parse::<i32>().unwrap();
            count += 1;
            if guess > number {
                println!("Too high! Guess again: ");
            } else if guess < number {
                println!("Too low! Guess again: ");
            } else {
                println!("Correct! You guessed {} times", count);
                break;
            }
            if count == 10 {
                println!("You ran out of guesses! The number was {}", number);
                break;
            }
            buffer.clear();
        }
        buffer.clear();
        println!("Play again? (y/n)");
        io::stdin().read_line(&mut buffer);
        if buffer.trim() == "n" {
            return;
        }
        buffer.clear();
        count = 0;
    }
}