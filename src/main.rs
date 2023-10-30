use levenshtein::levenshtein;
use rand::prelude::*;
use std::cmp::max;
use std::io;
use std::io::Write;

fn main() {
    let content = include_str!("words.txt");
    let words: Vec<&str> = content.lines().collect();
    let word = words.choose(&mut thread_rng()).unwrap();
    let word_len = word.len();
    println!("A {}-letter word has been chosen.\n", word_len);
    let mut guess = String::new();
    let mut guessn: u8 = 1;
    let mut show_indices: Vec<u8> = Vec::new();
    let mut remaining_indices: Vec<u8> = (0..word.len() as u8).collect();
    loop {
        guess.clear();
        print!("Enter guess #{guessn}: ");
        _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your guess");
        guess = guess.trim().to_lowercase();
        let edit_distance = levenshtein(word, &guess);
        if edit_distance == 0 {
            println!("The word was {word:?}. You win!");
            break;
        }
        let guess_len = guess.len();
        let longer_len = max(word_len, guess_len) as f32;
        let percentage_accuracy = (longer_len - edit_distance as f32) / longer_len * 100.;
        println!("Your guess {guess:?} was a {percentage_accuracy:.1}% match.");
        if guessn % 10 == 0 {
            let remaining_index =
                (rand::random::<f32>() * remaining_indices.len() as f32).floor() as usize;
            let show_index = remaining_indices[remaining_index];
            show_indices.push(show_index);
            remaining_indices.remove(remaining_index);
            if remaining_indices.is_empty() {
                println!("The word was {word:?}. You lose!");
                break;
            }
            print!("Here's a hint: ");
            for (index, letter) in word.char_indices() {
                if show_indices.contains(&(index as u8)) {
                    print!("{}", letter);
                } else {
                    print!("_");
                }
            }
            println!();
        }
        println!();
        guessn += 1;
    }
}
