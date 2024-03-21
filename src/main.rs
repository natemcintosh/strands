#![feature(test)]
extern crate test;
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

/// Goes from groups of six letters separated by a space, to a flat array
fn parse_flat_board(letters: &str) -> [char; 48] {
    let mut board: [char; 48] = [' '; 48];

    for (idx, c) in letters.replace(" ", "").char_indices() {
        board[idx] = c;
    }

    board
}

/// Return a list of neighbors. Works as like a 2d array of width `w` and height `h`.
/// Gets diagonal neighbors too.
///
/// Copied from
/// https://stackoverflow.com/questions/9355537/finding-neighbors-of-2d-array-when-represented-as-1d-array
///
/// Could perhaps write a version of this that takes in a mutable bit array, sets
/// everything to zero, then sets the right ones to true.
fn get_neighbors(i: usize, w: usize, h: usize) -> Vec<usize> {
    let size = w * h;
    let mut neighbors: Vec<usize> = Vec::new();

    if i.checked_sub(w).is_some() {
        neighbors.push(i - w); // north
    }

    if i % w != 0 {
        neighbors.push(i - 1); // west
    }

    if (i + 1) % w != 0 {
        neighbors.push(i + 1); // east
    }

    if (i + w) < size {
        neighbors.push(i + w); // south
    }

    if (i.checked_sub(w + 1).is_some()) & (i % w != 0) {
        neighbors.push(i - w - 1); // northwest
    }

    if ((i + 1).checked_sub(w).is_some()) & ((i + 1) % w != 0) {
        neighbors.push(i + 1 - w); // northeast
    }

    if ((i + w - 1) < size) & (i % w != 0) {
        neighbors.push(i + w - 1); // southwest
    }

    if ((i + w + 1) < size) & ((i + 1) % w != 0) {
        neighbors.push(i + w + 1); //southeast
    }

    neighbors
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
    use test::Bencher;

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

    #[test]
    fn test_parse_flat_board() {
        let letters = "olwish heucbl sykoda ecpeny sheyub ranngm ormora hscksh";
        let want: Vec<char> = letters.replace(" ", "").chars().collect();
        let got = parse_flat_board(letters).to_vec();

        assert_eq!(want, got);
    }

    #[test]
    fn test_get_neighbors() {
        let want = vec![
            vec![1, 3, 4],
            vec![0, 2, 4, 3, 5],
            vec![1, 5, 4],
            vec![0, 4, 6, 1, 7],
            vec![1, 3, 5, 7, 0, 2, 6, 8],
            vec![2, 4, 8, 1, 7],
            vec![3, 7, 4],
            vec![4, 6, 8, 3, 5],
            vec![5, 7, 4],
        ];
        for idx in 0..9 {
            let got = get_neighbors(idx, 3, 3);
            assert_eq!(want[idx], got, "failed for index {}", idx);
        }
    }

    #[bench]
    fn bench_get_neighbors_3x3(b: &mut Bencher) {
        b.iter(|| get_neighbors(4, 3, 3));
    }

    #[bench]
    fn bench_get_neighbors_8x6(b: &mut Bencher) {
        b.iter(|| get_neighbors(4, 6, 8));
    }
}
