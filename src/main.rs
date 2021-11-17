use hangman_lib::Game;
use std::io::{self, Write};
use rpassword;

fn main() {
    println!("Welcome to the hangman game!");

    let word = ask_search_word();
    let mut game = Game::new(&word, 5);

    while (game.tries_left() > 0) && (!game.word_found()) {
        let guesses = ask_letters();

        handle_guesses(&mut game, guesses);

        print_stats(&game);
    }

    if game.word_found() {
        println!("Congratulations, you found the word '{}'", game.word());
    } else {
        println!("Oooohh, you didn't find the word '{}'", game.word());
    }
}

fn ask_search_word() -> String {
    println!("Enter a word to search: ");

    let buffer = rpassword::read_password().unwrap();

    buffer.to_lowercase().trim().to_string()
}

fn ask_letters() -> Vec<char> {
    let mut buffer = String::new();
    print!("Enter one or more letters: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut buffer).unwrap();

    buffer.to_lowercase().trim().chars().into_iter().collect()
}

fn print_stats(game: &Game) {
    let guessed_word: String = game.guessed_word().into_iter().collect();

    println!();
    println!("Tries left: {}", game.tries_left());
    println!("{}", guessed_word);
    println!();
}

fn handle_guesses(game: &mut Game, guesses: Vec<char>) {
    for ch in guesses {
        if game.tries_left() == 0 {
            break;
        }

        match game.guess(ch) {
            hangman_lib::GuessResult::Wrong => println!("{} is not used in the word", ch),
            hangman_lib::GuessResult::Correct => println!("{} is found!", ch),
            hangman_lib::GuessResult::Repeat => println!("{} is already used as guess.", ch),
        }
    }
}
