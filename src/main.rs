use anyhow::Result;
use rayon::prelude::*;
use serde::Deserialize;
use serde_json::from_str;
use std::env::args;
use std::fs::read_to_string;
use std::process::exit;

/// A single verse
#[derive(Deserialize, Debug)]
struct Scripture {
    chapter: i32,
    verse: i32,
    text: String,
    translation_id: String,
    book_id: String,
    book_name: String,
}

/// Lookup key to be created from command-line arguments
struct LookupKey {
    book: String,
    chapter: i32,
    verses: Vec<i32>,
}

fn main() -> Result<()> {
    if let Ok(key) = parse_args() {
        // Read in the json file of scriptures and store as a vector
        let data = read_to_string("./kjv.json")?;
        let bible: Vec<Scripture> = from_str(&data)?;

        // Search for the specified verses
        let scriptures = bible
            .into_par_iter()
            .filter(|scripture| {
                // Check if provided the book name matches the ID or full book name
                (scripture.book_name.eq_ignore_ascii_case(&key.book)
                    || scripture.book_id.eq_ignore_ascii_case(&key.book))
                    // Match if the chapter matches the supplied chapter
                    && scripture.chapter == key.chapter
                    // Check if the verse matches any of the supplied verses
                    && key.verses.par_iter().any(|&v| v == scripture.verse)
            })
            .collect();

        display(scriptures);

        Ok(())
    } else {
        println!("Error parsing scripture lookup key");
        println!("Please ensure you enter a valid book, chapter, and verse");
        exit(2);
    }
}

/// Parses command-line arguments
///
/// Displays a usage message if proper command-line arguments were not supplied.
/// Returns the book, chapter, and verse range to lookup
fn parse_args() -> Result<LookupKey> {
    let mut args: Vec<String> = args().collect();
    if args.len() < 4 {
        println!("Usage: `bibterm [book] [chapter] [verses]`");
        exit(1);
    }
    let book;

    // If the first argument is a number, concatenate it with the book
    // For example, `2 John`
    if args[1].parse::<i32>().is_ok() {
        book = format!("{} {}", args.remove(1), args[1]);
    } else {
        // Otherwise, the first argument is the book name
        book = args[1].clone();
    }

    // Chapters are simple- just a single number
    let chapter = args[2].parse::<i32>()?;

    // Verses can either be listed by spaces or ranged by a hyphen
    let mut verses: Vec<i32> = Vec::new();
    // If the user entered a verse range
    if args[3].contains("-") {
        // Obtain the upper and lower bounds
        let verse_args: Vec<&str> = args[3].split("-").collect();

        // Parse the bounds
        let start_verse = verse_args[0].parse::<i32>()?;
        let end_verse = verse_args[1].parse::<i32>()?;

        // Generate a verse range
        for verse in start_verse..end_verse + 1 {
            verses.push(verse);
        }
    } else {
        // Otherwise, just keep parsing verses as they were entered
        for verse in args[3..].iter() {
            verses.push(verse.parse::<i32>()?);
        }
    }

    Ok(LookupKey {
        book,
        chapter,
        verses,
    })
}

/// Displays the formatted scripture
fn display(scriptures: Vec<Scripture>) {
    if scriptures.len() > 0 {
        println!("{} {}:", scriptures[0].book_name, scriptures[0].chapter);
        for scripture in scriptures {
            println!("\t{}: {}", scripture.verse, scripture.text);
        }
    } else {
        println!("Could not find that scripture");
    }
}
