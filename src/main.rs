use levenshtein::levenshtein;
use rand::prelude::*;
use std::cmp::max;
use std::io;
use std::io::Write;
use std::process::exit;

fn get_guess(guess: &mut String, guessn: usize) {
    guess.clear();
    print!("Enter guess #{guessn}: ");
    io::stdout().flush().expect("Failed to flush output");
    io::stdin()
        .read_line(guess)
        .expect("Failed to read your guess");
    *guess = guess.trim().to_lowercase();
}

fn get_percentage_accuracy(lengths: (usize, usize), edit_distance: usize) -> f32 {
    let (a_len, b_len) = lengths;
    let longer_len = max(a_len, b_len) as f32;
    (longer_len - edit_distance as f32) / longer_len * 100.
}

fn add_show_index(show_indices: &mut Vec<usize>, remaining_indices: &mut Vec<usize>) {
    let remaining_index = rand::random::<usize>() % remaining_indices.len();
    let show_index = remaining_indices[remaining_index];
    show_indices.push(show_index);
    remaining_indices.remove(remaining_index);
}

fn show_hint(word: &str, show_indices: &[usize]) {
    print!("Here's a hint: ");
    for (index, letter) in word.char_indices() {
        if show_indices.contains(&index) {
            print!("{}", letter);
        } else {
            print!("_");
        }
    }
    println!();
}

fn main() {
    let content = include_str!("words.txt");
    let words: Vec<&str> = content.lines().collect();
    let word = words.choose(&mut thread_rng()).unwrap().to_string();
    let word_clone = word.clone();
    ctrlc::set_handler(move || {
        println!("\nThe word was {word_clone:?}. Exiting.");
        exit(0);
    })
    .expect("Failed to set Ctrl+C handler");
    let word_len = word.len();
    println!("A {}-letter word has been chosen.\n", word_len);
    let mut guess = String::new();
    let mut guessn: usize = 1;
    let mut guess_len: usize;
    let mut show_indices: Vec<usize> = Vec::new();
    let mut remaining_indices: Vec<usize> = (0..word.len()).collect();
    loop {
        get_guess(&mut guess, guessn);
        if guess.is_empty() {
            println!("Please enter a guess. To quit, press Ctrl+C.\n");
            continue;
        }
        guess_len = guess.len();
        let edit_distance = levenshtein(&word, &guess);
        if edit_distance == 0 {
            println!("The word was {word:?}. You win!");
            break;
        }
        let percentage_accuracy = get_percentage_accuracy((word_len, guess_len), edit_distance);
        println!("Your guess {guess:?} was a {percentage_accuracy:.1}% match.");
        if guessn % 10 == 0 {
            add_show_index(&mut show_indices, &mut remaining_indices);
            if remaining_indices.is_empty() {
                println!("The word was {word:?}. You lose!");
                break;
            }
            show_hint(&word, &show_indices);
        }
        println!();
        guessn += 1;
    }
}
