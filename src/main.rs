use anyhow::Result;
use rayon::prelude::*;
use serde::Deserialize;
use std::env::args;
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
    verse: i32,
}

fn main() -> Result<()> {
    if let Ok(key) = parse_args() {
        // Read in the json file of scriptures and store as a vector
        let data = std::fs::read_to_string("./kjv.json")?;
        let bible: Vec<Scripture> = serde_json::from_str(&data)?;

        // Search for the specified verse
        let scripture = bible.into_par_iter().find_first(|scripture| {
            (scripture.book_name.eq_ignore_ascii_case(&key.book)
                || scripture.book_id.eq_ignore_ascii_case(&key.book))
                && scripture.chapter == key.chapter
                && scripture.verse == key.verse
        });

        display(scripture);

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
    let args: Vec<String> = args().collect();
    if args.len() < 4 {
        println!("Usage: `bibterm [book] [chapter] [verse]`");
        exit(1);
    }
    let book;
    let chapter;
    let verse;

    // If the first argument is a number, concatenate it with the book
    // For example, `2 John`
    if args[1].parse::<i32>().is_ok() {
        book = format!("{} {}", args[1], args[2]);
        chapter = args[3].parse::<i32>()?;
        verse = args[4].parse::<i32>()?;
    } else {
        // Otherwise, the first argument is the book name
        book = args[1].clone();
        chapter = args[2].parse::<i32>()?;
        verse = args[3].parse::<i32>()?;
    }

    Ok(LookupKey {
        book,
        chapter,
        verse,
    })
}

/// Displays the formatted scripture
fn display(scripture: Option<Scripture>) {
    if let Some(scrip) = scripture {
        println!(
            "{} {}:{}\n\t{}",
            scrip.book_name, scrip.chapter, scrip.verse, scrip.text
        );
    } else {
        println!("Could not find that scripture");
    }
}
