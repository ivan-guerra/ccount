use clap::{Parser, ValueEnum};
use itertools::Itertools;
use std::{collections::HashMap, io::Read};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "Input text")]
    text: Option<String>,

    #[arg(short, long, help = "Sort by character or count")]
    sort_by: Option<SortBy>,
    // TODO: Add option to print percentage of each character.
    // TODO: Add option to print only the top N characters.
    // TODO: Add option to print only the characters that appear more than N times.
    // TODO: Add option to print only the characters that appear less than N times.
    // TODO: Add option to print only the characters that appear exactly N times.
}

#[derive(Debug, Clone, ValueEnum)]
enum SortBy {
    Char,
    Count,
}

fn read_text(text: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    match text {
        Some(text) => Ok(text),
        None => {
            let mut text = String::new();
            std::io::stdin().read_to_string(&mut text)?;
            Ok(text)
        }
    }
}

fn create_counter(text: &str) -> HashMap<char, usize> {
    let mut counter = HashMap::new();
    text.chars()
        .filter(|c| !c.is_whitespace())
        .for_each(|c| *counter.entry(c).or_default() += 1);
    counter
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let text = read_text(args.text)?;
    let counter = create_counter(&text);
    counter
        .iter()
        .sorted_by(|a, b| match args.sort_by {
            Some(SortBy::Char) => a.0.cmp(b.0),
            Some(SortBy::Count) => b.1.cmp(a.1),
            None => a.0.cmp(b.0),
        })
        .for_each(|(c, count)| println!("{}: {}", c, count));

    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let counter = create_counter("");
        assert!(counter.is_empty());
    }

    #[test]
    fn test_single_character() {
        let counter = create_counter("a");
        assert_eq!(counter.get(&'a'), Some(&1));
        assert_eq!(counter.len(), 1);
    }

    #[test]
    fn test_multiple_same_characters() {
        let counter = create_counter("aaa");
        assert_eq!(counter.get(&'a'), Some(&3));
        assert_eq!(counter.len(), 1);
    }

    #[test]
    fn test_different_characters() {
        let counter = create_counter("abc");
        assert_eq!(counter.get(&'a'), Some(&1));
        assert_eq!(counter.get(&'b'), Some(&1));
        assert_eq!(counter.get(&'c'), Some(&1));
        assert_eq!(counter.len(), 3);
    }

    #[test]
    fn test_with_whitespace() {
        let counter = create_counter("a b c");
        assert_eq!(counter.get(&'a'), Some(&1));
        assert_eq!(counter.get(&'b'), Some(&1));
        assert_eq!(counter.get(&'c'), Some(&1));
        assert_eq!(counter.len(), 3);
    }

    #[test]
    fn test_case_sensitivity() {
        let counter = create_counter("aAaA");
        assert_eq!(counter.get(&'a'), Some(&2));
        assert_eq!(counter.get(&'A'), Some(&2));
        assert_eq!(counter.len(), 2);
    }

    #[test]
    fn test_special_characters() {
        let counter = create_counter("a!@#$%^&*()");
        assert_eq!(counter.get(&'a'), Some(&1));
        assert_eq!(counter.get(&'!'), Some(&1));
        assert_eq!(counter.get(&'@'), Some(&1));
        assert_eq!(counter.get(&'#'), Some(&1));
        assert_eq!(counter.len(), 11);
    }

    #[test]
    fn test_unicode_characters() {
        let counter = create_counter("Hello, 世界！🌍");
        assert_eq!(counter.get(&'H'), Some(&1));
        assert_eq!(counter.get(&'世'), Some(&1));
        assert_eq!(counter.get(&'界'), Some(&1));
        assert_eq!(counter.get(&'！'), Some(&1));
        assert_eq!(counter.get(&'🌍'), Some(&1));
        assert_eq!(counter.len(), 9);
    }

    #[test]
    fn test_mixed_unicode_and_ascii() {
        let counter = create_counter("café☕️");
        assert_eq!(counter.get(&'c'), Some(&1));
        assert_eq!(counter.get(&'a'), Some(&1));
        assert_eq!(counter.get(&'f'), Some(&1));
        assert_eq!(counter.get(&'é'), Some(&1));
        assert_eq!(counter.get(&'☕'), Some(&1));
        assert_eq!(counter.len(), 6);
    }
}
