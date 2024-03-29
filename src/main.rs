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

#[derive(Debug, PartialEq)]
struct Board {
    letters: Vec<char>,
    w: usize,
    h: usize,
}

impl Board {
    /// Goes from groups of six letters separated by a space, to a flat array
    fn parse_flat_board(letters: &str, width: usize, height: usize) -> Board {
        let mut bletters = Vec::with_capacity(width * height);

        for c in letters.replace(" ", "").chars() {
            bletters.push(c);
        }

        Board {
            letters: bletters,
            w: width,
            h: height,
        }
    }

    /// Return a list of neighbors. Works as like a 2d array of width `w` and height `h`.
    /// Gets diagonal neighbors too.
    ///
    /// Copied from
    /// https://stackoverflow.com/questions/9355537/finding-neighbors-of-2d-array-when-represented-as-1d-array
    ///
    /// Could perhaps write a version of this that takes in a mutable bit array, sets
    /// everything to zero, then sets the right ones to true.
    fn get_neighbors(&self, i: usize) -> Vec<usize> {
        let size = self.w * self.h;
        let mut neighbors: Vec<usize> = Vec::new();

        if i.checked_sub(self.w).is_some() {
            neighbors.push(i - self.w); // north
        }

        if i % self.w != 0 {
            neighbors.push(i - 1); // west
        }

        if (i + 1) % self.w != 0 {
            neighbors.push(i + 1); // east
        }

        if (i + self.w) < size {
            neighbors.push(i + self.w); // south
        }

        if (i.checked_sub(self.w + 1).is_some()) & (i % self.w != 0) {
            neighbors.push(i - self.w - 1); // northwest
        }

        if ((i + 1).checked_sub(self.w).is_some()) & ((i + 1) % self.w != 0) {
            neighbors.push(i + 1 - self.w); // northeast
        }

        if ((i + self.w - 1) < size) & (i % self.w != 0) {
            neighbors.push(i + self.w - 1); // southwest
        }

        if ((i + self.w + 1) < size) & ((i + 1) % self.w != 0) {
            neighbors.push(i + self.w + 1); //southeast
        }

        neighbors
    }

    /// From a given starting point on the board, what words can be formed?
    fn find_valid_words_from_start<'a>(
        &self,
        start_point: usize,
        words: &[&'a str],
    ) -> Vec<String> {
        let mut result = Vec::new();

        let start_spot = [start_point];
        let new_words: Vec<&str> = words
            .iter()
            .filter(|w| w.starts_with(self.letters[start_point]))
            .copied()
            .collect();

        result.extend(self.find_next(&new_words, &start_spot, start_point));
        result
    }

    /// A recursive method for finding valid words
    fn find_next<'a>(
        &self,
        words: &[&'a str],
        start_spots: &[usize],
        current_board_position: usize,
    ) -> Vec<String> {
        // If no more words, end
        if words.is_empty() {
            return vec![];
        }

        // Otherwise, loop over the neighbors, and return the results
        let mut result = Vec::new();
        let nbr_inds = self.get_neighbors(current_board_position);
        for nbr_idx in nbr_inds {
            // If this letter is already seen in the `start_spots`, continue
            if start_spots.contains(&nbr_idx) {
                continue;
            }
            // What word is created by adding this neighbor?
            let word = self.make_word_from_inds(start_spots, nbr_idx);

            // If adding this neighbor makes a complete word, push to result
            if words.contains(&&word.as_str()) {
                result.push(word.clone());
            }

            // What words are left for this word?
            let rem_words: Vec<&str> = words
                .iter()
                .filter(|w| w.starts_with(&word))
                .copied()
                .collect();

            // Quit if none left
            if rem_words.is_empty() {
                continue;
            }

            // Call again from this neighbor position and push to the the result
            let mut new_spots: Vec<usize> = start_spots.iter().copied().collect();
            new_spots.push(nbr_idx);
            result.extend(self.find_next(&rem_words, &new_spots, nbr_idx));
        }
        result
    }

    fn make_word_from_inds(&self, inds_so_far: &[usize], new_ind: usize) -> String {
        let mut word: String = inds_so_far.iter().map(|idx| self.letters[*idx]).collect();
        word.push(self.letters[new_ind]);
        word
    }
}

fn main() {
    let args = Args::parse();

    let board = Board::parse_flat_board(&args.letters, 6, 8);

    let words = fs::read_to_string(args.dictionary_file).expect("Unable to read file");
    let mut valid_words: Vec<&str> = words
        .lines()
        .filter(|s| !s.contains(char::is_uppercase))
        .filter(|&w| !w.ends_with("'s"))
        .filter(|&w| w.len() >= 4)
        .collect();
    valid_words.sort_unstable();
    valid_words.dedup();

    let filter_start = std::time::Instant::now();
    let all_words_that_fit: Vec<Vec<String>> = (0..6 * 8)
        .into_iter()
        .map(|start_point| board.find_valid_words_from_start(start_point, &valid_words))
        .collect();
    let filter_time = filter_start.elapsed().as_millis();
    println!("Filtering words for all spots took {}ms", filter_time);
    for start_pt in all_words_that_fit {
        println!("{:?}", start_pt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_parse_flat_board() {
        let letters = "olwish heucbl sykoda ecpeny sheyub ranngm ormora hscksh";
        let want_letters: Vec<char> = letters.replace(" ", "").chars().collect();
        let want = Board {
            letters: want_letters,
            w: 6,
            h: 8,
        };
        let got = Board::parse_flat_board(letters, 6, 8);

        assert_eq!(want, got);
    }

    #[rstest]
    #[case(0, vec![1, 3, 4])]
    #[case(1, vec![0, 2, 4, 3, 5])]
    #[case(2, vec![1, 5, 4])]
    #[case(3, vec![0, 4, 6, 1, 7])]
    #[case(4, vec![1, 3, 5, 7, 0, 2, 6, 8])]
    #[case(5, vec![2, 4, 8, 1, 7])]
    #[case(6, vec![3, 7, 4])]
    #[case(7, vec![4, 6, 8, 3, 5])]
    #[case(8, vec![5, 7, 4])]
    fn test_get_neighbors(#[case] idx: usize, #[case] want: Vec<usize>) {
        let board = Board::parse_flat_board("abc def ghi", 3, 3);
        let got = board.get_neighbors(idx);
        assert_eq!(want, got);
    }

    #[rstest]
    #[case(0, vec!["talon"])]
    #[case(1, vec!["argon"])]
    #[case(2, vec!["long","lose"])]
    #[case(3, vec!["rage"])]
    #[case(4, vec![])]
    #[case(5, vec!["ogre"])]
    #[case(6, vec!["ergo"])]
    #[case(7, vec!["solar"])]
    #[case(8, vec!["nose"])]
    fn test_find_valid_words_from_start(#[case] start_point: usize, #[case] want: Vec<&str>) {
        let board = Board::parse_flat_board("tal rgo esn", 3, 3);

        let words = vec![
            "talon", "ogre", "sunny", "batch", "solar", "argon", "ergo", "lose", "long", "rage",
            "tart", "nose",
        ];

        let mut got = board.find_valid_words_from_start(start_point, &words);
        got.sort_unstable();

        assert_eq!(want, got);
    }
}
