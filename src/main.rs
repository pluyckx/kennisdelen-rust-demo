use hangman_lib::Game;
use std::io;

fn main() {
    println!("Welcome to the hangman game!");
    println!("Enter a word to search: ");

    let mut buffer = String::new();
    let result = io::stdin().read_line(&mut buffer);

    if let Err(err) = result {
        panic!("{}", err.to_string());
    }

    buffer = buffer.to_lowercase();
    buffer = buffer.trim().to_string();
    let mut game = Game::new(&buffer, 5);

    while (game.tries_left() > 0) && (!game.word_found()) {
        println!("Enter one or more letters: ");
        buffer.clear();
        let result = io::stdin().read_line(&mut buffer);

        if let Err(err) = result {
            panic!("{}", err.to_string());
        }

        buffer = buffer.to_lowercase();

        for ch in buffer.trim().chars() {
            if game.tries_left() == 0 {
                break;
            }

            match game.guess(ch) {
                hangman_lib::GuessResult::Wrong => println!("{} is not used in the word", ch),
                hangman_lib::GuessResult::Correct => println!("{} is found!", ch),
                hangman_lib::GuessResult::Repeat => println!("{} is already used as guess.", ch),
            }
        }

        let guessed_word: String = game.guessed_word().into_iter().collect();
        println!();
        println!("Tries left: {}", game.tries_left());
        println!("{}", guessed_word);
        println!();
    }

    if game.word_found() {
        println!("Congratulations, you found the word '{}'", game.word());
    } else {
        println!("Oooohh, you didn't find the word '{}'", game.word());
    }
}
