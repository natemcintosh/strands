use std::fs;

use clap::Parser;
use smallvec::{smallvec, SmallVec};

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

    /// Minimum number of words
    #[arg()]
    min_words: usize,

    /// Maximum number of words
    #[arg()]
    max_words: usize,
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

/// Function to check if there is any overlap between the existing indices and new indices
fn bit_overlaps(existing: usize, new_indices: usize) -> bool {
    existing & new_indices != 0
}

/// Function to convert a &[usize] to a single usize representing the bits
fn indices_to_bits(indices: &[usize]) -> usize {
    indices.iter().fold(0, |acc, &idx| acc | (1 << idx))
}

fn solve(
    words_that_fit: &[Vec<(String, Vec<usize>)>],
    max_len: usize,
    board_w: usize,
    board_h: usize,
) -> Vec<String> {
    // Convert all the Vec<usize> into single usizes
    let condensed_words: Vec<usize> = words_that_fit
        .iter()
        .flat_map(|start_point| {
            start_point
                .iter()
                .map(|(_, indices)| indices_to_bits(indices))
        })
        .collect();

    // Get just the string out
    let flattened_words_that_fit: Vec<String> = words_that_fit
        .iter()
        .flat_map(|start_point| start_point.iter().map(|(word, _)| word.clone()))
        .collect();

    // Assume that these two are the same length
    assert_eq!(condensed_words.len(), flattened_words_that_fit.len());

    // Solver
    let mut selected_blocks: SmallVec<[usize; 12]> = smallvec![];
    let inds = inner_solve(
        0usize,
        &condensed_words,
        &mut selected_blocks,
        max_len,
        board_w,
        board_h,
    )
    .expect("Could not find a solution");

    // Get the words from the indices
    inds.iter()
        .filter_map(|ind| condensed_words.iter().position(|x| x == ind))
        .map(|ind| flattened_words_that_fit[ind].clone())
        .collect()
}

fn inner_solve(
    board: usize,
    blocks: &[usize],
    selected_blocks: &mut SmallVec<[usize; 12]>,
    max_len: usize,
    board_w: usize,
    board_h: usize,
) -> Option<SmallVec<[usize; 12]>> {
    // If we already have too many blocks, skips
    if selected_blocks.len() >= max_len {
        return None;
    }

    for (idx, block) in blocks.iter().enumerate() {
        // If this block can be placed
        if !bit_overlaps(*block, board) && no_diagonal_overlap(*block, board, board_w, board_h) {
            // Place the block
            let new_board = block | board;
            selected_blocks.push(*block);

            // If we've filled the board
            if new_board.count_ones() as usize == (board_h * board_w) {
                return Some(selected_blocks.clone());
            }

            // If we're at max len, we haven't yet filled the board. Remove the block
            // and skip to next word
            if selected_blocks.len() >= max_len {
                selected_blocks.pop();
                continue;
            }

            // Try to add another block
            if let Some(res) = inner_solve(
                new_board,
                &blocks[idx + 1..],
                selected_blocks,
                max_len,
                board_w,
                board_h,
            ) {
                return Some(res);
            }

            // Backtrack
            selected_blocks.pop();
        }
    }
    None
}

fn no_diagonal_overlap(block: usize, board: usize, board_w: usize, board_h: usize) -> bool {
    // Convert the block and board to sets of indices
    let block_indices = bits_to_indices(block, board_w, board_h);
    let board_indices = bits_to_indices(board, board_w, board_h);

    if board_indices.is_empty() {
        return true;
    }
    !crosses_existing_lines(&board_indices, &block_indices, board_w)
}

fn bits_to_indices(bits: usize, board_w: usize, board_h: usize) -> Vec<usize> {
    (0..(board_w * board_h))
        .filter(|&i| bits & (1 << i) != 0)
        .collect()
}

fn crosses_existing_lines(
    existing_indices: &[usize],
    new_block_indices: &[usize],
    board_w: usize,
) -> bool {
    fn index_to_coords(index: usize, board_w: usize) -> (isize, isize) {
        (
            index as isize / board_w as isize,
            index as isize % board_w as isize,
        )
    }

    for i in 0..new_block_indices.len() - 1 {
        let (x1, y1) = index_to_coords(new_block_indices[i], board_w);
        let (x2, y2) = index_to_coords(new_block_indices[i + 1], board_w);

        for j in 0..existing_indices.len() - 1 {
            let (x3, y3) = index_to_coords(existing_indices[j], board_w);
            let (x4, y4) = index_to_coords(existing_indices[j + 1], board_w);

            // Check if line (x1, y1) to (x2, y2) intersects with line (x3, y3) to (x4, y4)
            if lines_intersect((x1, y1), (x2, y2), (x3, y3), (x4, y4)) {
                return true;
            }
        }
    }

    false
}

fn lines_intersect(
    a1: (isize, isize),
    a2: (isize, isize),
    b1: (isize, isize),
    b2: (isize, isize),
) -> bool {
    // Calculate the direction of the points
    fn direction(a: (isize, isize), b: (isize, isize), c: (isize, isize)) -> isize {
        (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
    }

    let d1 = direction(a1, a2, b1);
    let d2 = direction(a1, a2, b2);
    let d3 = direction(b1, b2, a1);
    let d4 = direction(b1, b2, a2);

    if d1 != 0 && d2 != 0 && d3 != 0 && d4 != 0 {
        return (d1 < 0 && d2 > 0 || d1 > 0 && d2 < 0) && (d3 < 0 && d4 > 0 || d3 > 0 && d4 < 0);
    }

    false
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
    let all_words_that_fit: Vec<Vec<(String, Vec<usize>)>> = (0..(6 * 8))
        .map(|start_point| board.find_valid_words_from_start(start_point, &valid_words))
        .collect();
    let filter_time = filter_start.elapsed().as_millis();
    println!("Filtering words for all spots took {filter_time}ms");
    println!(
        "Found {} possible words",
        all_words_that_fit.iter().flatten().count()
    );

    // Find the solution
    let solve_start_time = std::time::Instant::now();
    let solution = solve(&all_words_that_fit, args.max_words, 6, 8);
    println!("\n\nFound solution!");
    println!("{solution:?}");
    let solve_time = solve_start_time.elapsed().as_secs_f64();
    println!("Solve took {solve_time:0.2}s");
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

    #[rstest]
    #[case(0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000, 6, 8, vec![])]
    #[case(0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001, 6, 8, vec![0])]
    #[case(0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0011, 6, 8, vec![0, 1])]
    #[case(0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0100, 6, 8, vec![2])]
    #[case(0b0000_0000_0000_0000_0000_0000_0000_0000_0000_1000_0000_0000, 6, 8, vec![11])]
    #[case(0b1111_1100_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000, 6, 8, vec![42, 43, 44, 45, 46, 47])]
    #[case(0b0_0000_0000, 3, 3, vec![])]
    #[case(0b0_0000_0001, 3, 3, vec![0])]
    #[case(0b0_0000_0010, 3, 3, vec![1])]
    #[case(0b0_0000_0100, 3, 3, vec![2])]
    #[case(0b0_0000_1000, 3, 3, vec![3])]
    #[case(0b0_0001_0000, 3, 3, vec![4])]
    #[case(0b0_0010_0000, 3, 3, vec![5])]
    #[case(0b0_0100_0000, 3, 3, vec![6])]
    #[case(0b0_1000_0000, 3, 3, vec![7])]
    #[case(0b1_0000_0000, 3, 3, vec![8])]
    #[case(0b1_1111_1111, 3, 3, vec![0, 1, 2, 3, 4, 5, 6, 7, 8])]
    #[case(0b1_0101_0101, 3, 3, vec![0, 2, 4, 6, 8])]
    #[case(0b0_1010_1010, 3, 3, vec![1, 3, 5, 7])]
    #[case(0b0_0100_1001, 3, 3, vec![0, 3, 6])]
    #[case(0b1_1000_0000, 3, 3, vec![7, 8])]
    fn test_bits_to_indices(
        #[case] bits: usize,
        #[case] board_w: usize,
        #[case] board_h: usize,
        #[case] expected: Vec<usize>,
    ) {
        let result = bits_to_indices(bits, board_w, board_h);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(vec![0, 1, 2, 3], vec![4, 5], 6, false)]
    #[case(vec![0, 6], vec![1, 7], 6, false)]
    #[case(vec![0, 1, 2], vec![4, 5], 6, false)]
    #[case(vec![0, 1, 2], vec![6, 7, 8], 6, false)]
    #[case(vec![0, 1, 2, 9], vec![6, 7, 8], 6, false)]
    #[case(vec![0, 6, 1], vec![7, 8], 6, false)]
    #[case(vec![0, 7], vec![1, 6], 6, true)]
    #[case(vec![0, 7], vec![6, 1], 6, true)]
    #[case(vec![0, 3, 4, 2], vec![1, 5, 8, 7, 6], 3, true)]
    #[case(vec![0, 3, 7], vec![6, 4], 3, true)]
    #[case(vec![1, 5], vec![2, 4], 3, true)]
    #[case(vec![0, 4, 8], vec![1, 3], 3, true)]
    #[case(vec![0, 4, 8], vec![1, 2], 3, false)]
    fn test_crosses_existing_lines(
        #[case] existing_indices: Vec<usize>,
        #[case] new_block_indices: Vec<usize>,
        #[case] board_w: usize,
        #[case] expected: bool,
    ) {
        let result = crosses_existing_lines(&existing_indices, &new_block_indices, board_w);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve() {
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

        let want: Vec<String> = vec!["talon".to_string(), "regs".to_string()];

        let got = solve(&words_that_fit, 2, 3, 3);

        assert_eq!(want, got);
    }

    #[test]
    fn test_solve_2() {
        let board = Board::parse_flat_board("tim lta ecl", 3, 3);

        let words =
            fs::read_to_string("american_english_dictionary.txt").expect("Unable to read file");
        let mut valid_words: Vec<&str> = words
            .lines()
            .filter(|s| !s.contains(char::is_uppercase))
            .filter(|&w| !w.ends_with("'s"))
            .filter(|&w| w.len() >= 4)
            .collect();
        valid_words.sort_unstable();
        valid_words.dedup();

        let words_that_fit: Vec<Vec<(String, Vec<usize>)>> = (0..(3 * 3))
            .map(|start_point| board.find_valid_words_from_start(start_point, &valid_words))
            .collect();

        let mut want: Vec<String> = "title clam"
            .split_ascii_whitespace()
            .map(std::string::ToString::to_string)
            .collect();
        want.sort_unstable();

        let mut got = solve(&words_that_fit, 2, 3, 3);
        got.sort_unstable();

        assert_eq!(want, got);
    }

    #[test]
    #[should_panic]
    fn test_solve_diag() {
        // This test covers the case where two words exist that cross on the diagonal,
        // which is not allowed
        // In this case, if diagonals were allowed, it would find "camp" and "dress"
        // but they are not, and it should panic
        let board = Board::parse_flat_board("cdp amr sse", 3, 3);

        let words =
            fs::read_to_string("american_english_dictionary.txt").expect("Unable to read file");
        let mut valid_words: Vec<&str> = words
            .lines()
            .filter(|s| !s.contains(char::is_uppercase))
            .filter(|&w| !w.ends_with("'s"))
            .filter(|&w| w.len() >= 4)
            .collect();
        valid_words.sort_unstable();
        valid_words.dedup();

        let words_that_fit: Vec<Vec<(String, Vec<usize>)>> = (0..(3 * 3))
            .map(|start_point| board.find_valid_words_from_start(start_point, &valid_words))
            .collect();

        let got = solve(&words_that_fit, 2, 3, 3);
        dbg!(got);
    }

    #[test]
    #[ignore = "too long"]
    fn test_solve_long() {
        let board = Board::parse_flat_board(
            "rdpcym umelab rtrcge ileuon agrsni nasgur etioob ltntam",
            6,
            8,
        );

        let words =
            fs::read_to_string("american_english_dictionary.txt").expect("Unable to read file");
        let mut valid_words: Vec<&str> = words
            .lines()
            .filter(|s| !s.contains(char::is_uppercase))
            .filter(|&w| !w.ends_with("'s"))
            .filter(|&w| w.len() >= 4)
            .collect();
        valid_words.sort_unstable();
        valid_words.dedup();

        let words_that_fit: Vec<Vec<(String, Vec<usize>)>> = (0..(6 * 8))
            .map(|start_point| board.find_valid_words_from_start(start_point, &valid_words))
            .collect();

        let mut want: Vec<String> = "drum triangle rattle percussion cymbal gong tambourine"
            .split_ascii_whitespace()
            .map(std::string::ToString::to_string)
            .collect();
        want.sort_unstable();

        let mut got = solve(&words_that_fit, 9, 6, 8);
        got.sort_unstable();

        assert_eq!(want, got);
    }

    #[rstest]
    #[case(0b0000, 0b0000, false)] // both empty
    #[case(0b0001, 0b0010, false)] // ones in different places
    #[case(0b0010, 0b0010, true)] // direct overlap
    #[case(0b1100, 0b0011, false)] // ones in different places
    #[case(0b1100, 0b0100, true)] // one overlap
    #[case(0b1010, 0b1001, true)] // one overlap
    #[case(0b1111, 0b0000, false)] // all of one or the other
    #[case(0b1111, 0b1111, true)] // all ones all the way
    fn test_bit_overlaps(
        #[case] existing: usize,
        #[case] new_indices: usize,
        #[case] expected: bool,
    ) {
        let result = bit_overlaps(existing, new_indices);
        assert_eq!(result, expected);
    }
}
