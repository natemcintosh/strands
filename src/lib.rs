#[inline]
pub fn two_words_no_diag_overlap(
    word1: usize,
    word2: usize,
    board_width: usize,
    board_height: usize,
) -> bool {
    for y in 0..board_height - 1 {
        for x in 0..board_width - 1 {
            let i1 = y * board_width + x; // Top-left
            let i2 = i1 + 1; // Top-right
            let i3 = i1 + board_width; // Bottom-left
            let i4 = i3 + 1; // Bottom-right

            // word 1 bits that are 1 in the 2x2 square
            let b1 = ((word1 >> i1) & 1) != 0;
            let b2 = ((word1 >> i2) & 1) != 0;
            let b3 = ((word1 >> i3) & 1) != 0;
            let b4 = ((word1 >> i4) & 1) != 0;

            // word 2 bits that are 1 in the 2x2 square
            let e1 = ((word2 >> i1) & 1) != 0;
            let e2 = ((word2 >> i2) & 1) != 0;
            let e3 = ((word2 >> i3) & 1) != 0;
            let e4 = ((word2 >> i4) & 1) != 0;

            // Check for an x-shape in the 2x2 grid
            if (b1 && b4 && e2 && e3) || (e1 && e4 && b2 && b3) {
                return false;
            }
        }
    }
    true
}

/// For each word in `existing_words`, check if `new_word` doesn't
/// cross any of them diagonally
pub fn no_diagonal_overlap(
    existing_words: &[usize],
    new_word: usize,
    board_width: usize,
    board_height: usize,
) -> bool {
    existing_words
        .iter()
        .all(|&word| two_words_no_diag_overlap(word, new_word, board_width, board_height))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0b0011, 0b1100, 2, 2, true)] // 2x2 board rows, no crossing
    #[case(0b0101, 0b1010, 2, 2, true)] // 2x2 board no cols, no crossing
    #[case(0b1001, 0b0110, 2, 2, false)] // 2x2 board x crossing
    #[case(0b000000000, 0b000000000, 3, 3, true)] // No filled spots
    #[case(0b000000000, 0b111111111, 3, 3, true)] // No overlaps, fully filled board
    #[case(0b000000001, 0b000000001, 3, 3, true)] // Same filled spot
    #[case(0b000000011, 0b000000010, 3, 3, true)] // No diagonal overlap
    #[case(0b000000111, 0b000000001, 3, 3, true)] // One overlapping spot
    #[case(0b000001000, 0b000000001, 3, 3, true)] // Non-overlapping single spots
    #[case(0b000110001, 0b000001110, 3, 3, false)] // two crossing words
    #[case(0b010000000, 0b001000000, 3, 3, true)] // Non-overlapping column
    #[case(0b100100010, 0b010010100, 3, 3, false)] // two crossing words
    #[case(0b111111111, 0b000000000, 3, 3, true)] // Fully filled block, empty board
    fn test_no_diagonal_overlap(
        #[case] block: usize,
        #[case] board: usize,
        #[case] board_width: usize,
        #[case] board_height: usize,
        #[case] expected: bool,
    ) {
        assert_eq!(
            two_words_no_diag_overlap(block, board, board_width, board_height),
            expected
        );
    }
}
