#[inline]
pub fn no_diagonal_overlap(
    block: usize,
    board: usize,
    board_width: usize,
    board_height: usize,
) -> bool {
    for y in 0..board_height - 1 {
        for x in 0..board_width - 1 {
            let i1 = y * board_width + x; // Top-left
            let i2 = i1 + 1; // Top-right
            let i3 = i1 + board_width; // Bottom-left
            let i4 = i3 + 1; // Bottom-right

            // Block bits
            let b1 = (block >> i1) & 1;
            let b2 = (block >> i2) & 1;
            let b3 = (block >> i3) & 1;
            let b4 = (block >> i4) & 1;

            // Board bits
            let e1 = (board >> i1) & 1;
            let e2 = (board >> i2) & 1;
            let e3 = (board >> i3) & 1;
            let e4 = (board >> i4) & 1;

            // Check for overlapping conditions:
            // 1. If block and board overlap at corners
            // 2. Ensure that lines don't cross diagonally
            if (b1 & e4 != 0)
                || (b2 & e3 != 0)
                || (b3 & e2 != 0)
                || (b4 & e1 != 0)
                || (b1 & b2 != 0 && (e3 | e4) != 0)
                || (b3 & b4 != 0 && (e1 | e2) != 0)
                || (b1 & b3 != 0 && (e2 | e4) != 0)
                || (b2 & b4 != 0 && (e1 | e3) != 0)
            {
                return false;
            }
        }
    }
    true
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
    #[case(0b000000111, 0b000000001, 3, 3, false)] // Overlapping diagonal
    #[case(0b000001000, 0b000000001, 3, 3, true)] // Non-overlapping single spots
    #[case(0b000011000, 0b000001000, 3, 3, false)] // Overlapping row
    #[case(0b010000000, 0b001000000, 3, 3, true)] // Non-overlapping column
    #[case(0b111000000, 0b000111000, 3, 3, false)] // Overlapping diagonal across rows
    #[case(0b000111000, 0b000000111, 3, 3, false)] // Overlapping across columns
    #[case(0b001010000, 0b000010100, 3, 3, false)] // Overlapping diagonally
    #[case(0b111111111, 0b000000000, 3, 3, true)] // Fully filled block, empty board
    fn test_no_diagonal_overlap(
        #[case] block: usize,
        #[case] board: usize,
        #[case] board_width: usize,
        #[case] board_height: usize,
        #[case] expected: bool,
    ) {
        assert_eq!(
            no_diagonal_overlap(block, board, board_width, board_height),
            expected
        );
    }
}
