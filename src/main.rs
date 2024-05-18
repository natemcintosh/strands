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

        for c in letters.replace(' ', "").chars() {
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
    /// <https://stackoverflow.com/questions/9355537/finding-neighbors-of-2d-array-when-represented-as-1d-array>
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
            neighbors.push(i + self.w + 1); // southeast
        }

        neighbors
    }

    /// From a given starting point on the board, what words can be formed?
    fn find_valid_words_from_start(
        &self,
        start_point: usize,
        words: &[&str],
    ) -> Vec<(String, Vec<usize>)> {
        let mut result: Vec<(String, Vec<usize>)> = Vec::new();

        let start_spot = vec![start_point];
        let new_words: Vec<&str> = words
            .iter()
            .filter(|w| w.starts_with(self.letters[start_point]))
            .copied()
            .collect();

        result.extend(self.find_next(&new_words, &start_spot, start_point));
        result
    }

    /// A recursive method for finding valid words
    fn find_next(
        &self,
        words: &[&str],
        start_spots: &[usize],
        current_board_position: usize,
    ) -> Vec<(String, Vec<usize>)> {
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
            if words.contains(&word.as_str()) {
                let mut positions = start_spots.to_vec();
                positions.push(nbr_idx);
                result.push((word.clone(), positions));
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
            let mut new_spots: Vec<usize> = start_spots.to_vec();
            new_spots.push(nbr_idx);
            result.extend(self.find_next(&rem_words, &new_spots, nbr_idx));
        }
        result
    }

    /// Given a set of indices, create a new word from the letters at those indices
    fn make_word_from_inds(&self, inds_so_far: &[usize], new_ind: usize) -> String {
        let mut word: String = inds_so_far.iter().map(|idx| self.letters[*idx]).collect();
        word.push(self.letters[new_ind]);
        word
    }
}

/// This takes a collection of words, paired with the indices they live at, and finds
/// the combination of words that covers the most of the board, while not covering
/// each other.
fn combo_with_most_coverage(
    words_that_fit: &[Vec<(String, Vec<usize>)>],
) -> Vec<(String, Vec<usize>)> {
    // Helper function to check if a word overlaps with any existing words in the current combo
    fn overlaps(existing: &[usize], new_indices: &Vec<usize>) -> bool {
        for &index in new_indices {
            if existing.contains(&index) {
                return true; // Overlap found
            }
        }
        false
    }

    // Recursive function to find the best combination
    fn find_best_combo(
        words_that_fit: &[Vec<(String, Vec<usize>)>],
        current_combo: &mut Vec<(String, Vec<usize>)>,
        best_combo: &mut Vec<(String, Vec<usize>)>,
        covered_indices: &mut Vec<usize>,
        start_idx: usize,
    ) {
        // If we have reached the end of the list, check if the current combo is the best one
        if start_idx >= words_that_fit.len() {
            if covered_indices.len() > best_combo.iter().flat_map(|(_, indices)| indices).count() {
                best_combo.clone_from(current_combo);
            }
            return;
        }

        // Try each word in the current position
        for (word, indices) in &words_that_fit[start_idx] {
            if !overlaps(covered_indices, indices) {
                // Add this word to the current combo
                current_combo.push((word.clone(), indices.clone()));
                covered_indices.extend(indices.iter());

                // Recur to find the next word
                find_best_combo(
                    words_that_fit,
                    current_combo,
                    best_combo,
                    covered_indices,
                    start_idx + 1,
                );

                // Backtrack
                current_combo.pop();
                for index in indices {
                    covered_indices.retain(|&x| x != *index);
                }
            }
            println!("{start_idx}");
        }

        // Also consider the case where we skip the current position
        find_best_combo(
            words_that_fit,
            current_combo,
            best_combo,
            covered_indices,
            start_idx + 1,
        );
    }

    let mut best_combo = Vec::new();
    let mut current_combo = Vec::new();
    let mut covered_indices = Vec::new();

    find_best_combo(
        words_that_fit,
        &mut current_combo,
        &mut best_combo,
        &mut covered_indices,
        0,
    );

    best_combo
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
    let all_words_that_fit: Vec<Vec<(String, Vec<usize>)>> = (0..6 * 8)
        .map(|start_point| board.find_valid_words_from_start(start_point, &valid_words))
        .collect();
    let filter_time = filter_start.elapsed().as_millis();
    println!("Filtering words for all spots took {filter_time}ms");
    println!(
        "Found {} possible words",
        all_words_that_fit.iter().flatten().count()
    );
    let best_answer = combo_with_most_coverage(&all_words_that_fit);
    for (word, _) in &best_answer {
        println!("{word}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_parse_flat_board() {
        let letters = "olwish heucbl sykoda ecpeny sheyub ranngm ormora hscksh";
        let want_letters: Vec<char> = letters.replace(' ', "").chars().collect();
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
    #[case(0, vec![("talon".to_string(), vec![0, 1, 2, 5, 8])])]
    #[case(1, vec![("argon".to_string(), vec![1, 3, 4, 5, 8])])]
    #[case(2, vec![("long".to_string(), vec![2, 5, 8, 4]), ("lose".to_string(), vec![2, 5, 7,6])])]
    #[case(3, vec![("rage".to_string(), vec![3, 1, 4, 6])])]
    #[case(4, vec![("glare".to_string(), vec![4, 2, 1, 3, 6])])]
    #[case(5, vec![("ogre".to_string(), vec![5, 4, 3, 6])])]
    #[case(6, vec![("ergo".to_string(), vec![6, 3, 4, 5])])]
    #[case(7, vec![("solar".to_string(), vec![7, 5, 2, 1, 3])])]
    #[case(8, vec![("nose".to_string(), vec![8, 5, 7, 6])])]
    fn test_find_valid_words_from_start(
        #[case] start_point: usize,
        #[case] want: Vec<(String, Vec<usize>)>,
    ) {
        let board = Board::parse_flat_board("tal rgo esn", 3, 3);

        let words = vec![
            "talon", "ogre", "sunny", "batch", "solar", "argon", "ergo", "lose", "long", "rage",
            "tart", "nose", "glare",
        ];

        let mut got = board.find_valid_words_from_start(start_point, &words);
        got.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        assert_eq!(want, got);
    }

    #[test]
    fn test_combo_with_most_coverage() {
        let words_that_fit: Vec<Vec<(String, Vec<usize>)>> = vec![
            vec![("talon".to_string(), vec![0, 1, 2, 5, 8])],
            vec![("argon".to_string(), vec![1, 3, 4, 5, 8])],
            vec![
                ("long".to_string(), vec![2, 5, 8, 4]),
                ("lose".to_string(), vec![2, 5, 7, 6]),
            ],
            vec![
                ("rage".to_string(), vec![3, 1, 4, 6]),
                ("regs".to_string(), vec![3, 6, 4, 7]),
            ],
            vec![("glare".to_string(), vec![4, 2, 1, 3, 6])],
            vec![("ogre".to_string(), vec![5, 4, 3, 6])],
            vec![("ergo".to_string(), vec![6, 3, 4, 5])],
            vec![("solar".to_string(), vec![7, 5, 2, 1, 3])],
            vec![("nose".to_string(), vec![8, 5, 7, 6])],
        ];

        let want: Vec<(String, Vec<usize>)> = vec![
            ("talon".to_string(), vec![0, 1, 2, 5, 8]),
            ("regs".to_string(), vec![3, 6, 4, 7]),
        ];

        let got = combo_with_most_coverage(&words_that_fit);

        for (w, g) in want.iter().zip(got.iter()) {
            assert_eq!(w, g);
        }
    }
}
