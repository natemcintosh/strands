use std::fs;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Each row of letters, separated by a space. E.g. "abc def ghi".
    /// There should be exactly 8 groups of 6 letters
    #[arg()]
    letters: String,

    /// The dictionary file to use. By default, use the american english dictionary file
    #[arg(short = 'd', long, default_value = "american_english_dictionary.txt")]
    dictionary_file: String,
}

/// Goes from groups of six letters separated by a space, to a fixed-size board
fn parse_board(letters: &str) -> [[char; 6]; 8] {
    let mut board: [[char; 6]; 8] = [[' '; 6]; 8];

    for (ridx, row) in letters.split_whitespace().enumerate() {
        for (cidx, c) in row.char_indices() {
            board[ridx][cidx] = c;
        }
    }

    board
}

fn main() {
    let args = Args::parse();

    let board = parse_board(&args.letters);
    println!("{:?}", board);

    let words = fs::read_to_string(args.dictionary_file).expect("Unable to read file");
    let mut valid_words: Vec<&str> = words
        .lines()
        .filter(|s| !s.contains(char::is_uppercase))
        .filter(|&w| !w.ends_with("'s"))
        .collect();
    valid_words.sort_unstable();
    valid_words.dedup();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_board() {
        let letters = "olwish heucbl sykoda ecpeny sheyub ranngm ormora hscksh";
        let want = [
            ['o', 'l', 'w', 'i', 's', 'h'],
            ['h', 'e', 'u', 'c', 'b', 'l'],
            ['s', 'y', 'k', 'o', 'd', 'a'],
            ['e', 'c', 'p', 'e', 'n', 'y'],
            ['s', 'h', 'e', 'y', 'u', 'b'],
            ['r', 'a', 'n', 'n', 'g', 'm'],
            ['o', 'r', 'm', 'o', 'r', 'a'],
            ['h', 's', 'c', 'k', 's', 'h'],
        ];

        let got = parse_board(letters);
        assert_eq!(want, got);
    }

    #[test]
    #[should_panic]
    fn test_parse_board_bad1() {
        // Note that the first group has one more letter than it should
        let letters = "olwishd heucbl sykoda ecpeny sheyub ranngm ormora hscksh";
        let want = [
            ['o', 'l', 'w', 'i', 's', 'h'],
            ['h', 'e', 'u', 'c', 'b', 'l'],
            ['s', 'y', 'k', 'o', 'd', 'a'],
            ['e', 'c', 'p', 'e', 'n', 'y'],
            ['s', 'h', 'e', 'y', 'u', 'b'],
            ['r', 'a', 'n', 'n', 'g', 'm'],
            ['o', 'r', 'm', 'o', 'r', 'a'],
            ['h', 's', 'c', 'k', 's', 'h'],
        ];

        let got = parse_board(letters);
        assert_eq!(want, got);
    }

    #[test]
    #[should_panic]
    fn test_parse_board_bad2() {
        // Note that the first group has one more letter than it should
        let letters = "olwish heucbl sykoda ecpeny sheyub ranngm ormora hscksh abcdef";
        let want = [
            ['o', 'l', 'w', 'i', 's', 'h'],
            ['h', 'e', 'u', 'c', 'b', 'l'],
            ['s', 'y', 'k', 'o', 'd', 'a'],
            ['e', 'c', 'p', 'e', 'n', 'y'],
            ['s', 'h', 'e', 'y', 'u', 'b'],
            ['r', 'a', 'n', 'n', 'g', 'm'],
            ['o', 'r', 'm', 'o', 'r', 'a'],
            ['h', 's', 'c', 'k', 's', 'h'],
        ];

        let got = parse_board(letters);
        assert_eq!(want, got);
    }
}
