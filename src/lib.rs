enum Guess {
    Wrong(char),
    Correct(char, Vec<usize>),
}

pub enum GuessResult {
    Wrong,
    Correct,
    Repeat,
}

pub struct Game {
    word: String,
    guesses: Vec<Guess>,
    tries_left: u32,
}

impl Game {
    pub fn new(word: &str, tries: u32) -> Self {
        Self {
            word: word.to_string(),
            guesses: Vec::new(),
            tries_left: tries,
        }
    }

    pub fn guess(&mut self, ch: char) -> GuessResult {
        if !self.is_new_guess(ch) {
            return GuessResult::Repeat;
        }

        if self.tries_left == 0 {
            panic!("No tries left! Guess not allowed!");
        }

        let mut indexes = Vec::new();

        for (idx, character) in self.word.chars().enumerate() {
            if character == ch {
                indexes.push(idx);
            }
        }

        if indexes.len() > 0 {
            self.guesses.push(Guess::Correct(ch, indexes));
            GuessResult::Correct
        } else {
            self.guesses.push(Guess::Wrong(ch));
            self.tries_left -= 1;

            GuessResult::Wrong
        }
    }

    pub fn tries_left(&self) -> u32 {
        self.tries_left
    }

    pub fn word_found(&self) -> bool {
        if self.guessed_word().contains(&'_') {
            false
        } else {
            true
        }
    }

    pub fn word(&self) -> &str {
        &self.word
    }

    pub fn guessed_word(&self) -> Vec<char> {
        let mut guessed_word = Vec::with_capacity(self.word.len());

        for _ in &mut self.word.chars() {
            guessed_word.push('_');
        }

        for guess in &self.guesses {
            match guess {
                Guess::Wrong(_) => {}
                Guess::Correct(ch, positions) => {
                    for position in positions {
                        guessed_word[*position] = *ch;
                    }
                }
            }
        }

        guessed_word
    }

    fn is_new_guess(&self, ch: char) -> bool {
        for guess in &self.guesses {
            if ch == guess.character() {
                return false;
            }
        }

        true
    }
}

impl Guess {
    pub fn character(&self) -> char {
        match self {
            Self::Wrong(ch) => *ch,
            Self::Correct(ch, _) => *ch,
        }
    }
}
