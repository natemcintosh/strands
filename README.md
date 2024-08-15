# Strands

This CLI program attempts to solve the [strands](https://www.nytimes.com/games/strands) game by:
1. finding words that can exist on the board
1. finding the right combination of words that cover the board with no overlap, or crossing each other.

It can do these things reasonably quickly, depending on the number of words that can exist on the board. I have seen it run in anywhere from 20 seconds to >10 minutes, but usually less than a minute.

**NB:** This solver knows nothing about the theme words, only what words in the dictionary can all fit on the board nicely. That means that it is entirely possible it will find a set of words that perfectly fills the board, but does not actually win the game. In fact, this is what happens most of the time: it finds a solution that uses most, but not all, of the words that are the actual solution to the game. 

## Usage
- Clone this repo.
- `cargo build --release` to build the project with optimizations turned on.
- `./target/release/strands -h` to get the help message that explains how to run the binary.

## How it works

### Steps
The algorithm is designed to efficiently place words on a rectangular board while checking for overlap and crossings. Here's a high-level overview of the process:

1. **Identify Potential Words:** Begin by identifying all words that could possibly fit on the board. For a given start point on the board, what words can be made from that point, using only the letters next to it?
1. **Initialize the Board:** Start with an empty board where each spot can either be filled with a letter or remain empty.
1. **Place Words Sequentially:** Words are placed on the board one by one, starting with the first, and moving forward recursively.
1. **Check for Valid Placement:**
   - Ensure that the new word does not overlap (use the same spot on the board) with any existing words.
   - Ensure that the new word does not cross (not spot overlap, but the crossing of two words) any existing word.
1. **Complete the Board:** Continue placing words and removing ones that don't fit, until a solution that fully fills the board is found, or all possible solutions have been examined.


### Optimizations
To optimize both memory usage and performance, the board is represented using bit packing. This approach allows the entire state of the board to be stored in a single `usize` integer, which is both compact and fast to manipulate.

- **Bit Packing Representation:**
  - Each spot on the board is represented by a single bit in a `usize` integer. A bit value of `1` indicates that the spot is filled (i.e., it contains a part of a word), while a bit value of `0` indicates that the spot is empty.
  - For example, on a 3x3 board, the integer `0b101110001` would represent a board where certain spots are filled, and others are empty.

`0b101110001` represented on a 3x3 board:

| 1 | 0 | 0 |
|---|---|---|
| 0 | 1 | 1 |
| 1 | 0 | 1 |

Note that the last bit in the integer (starting at the ones place) represents the first position on the board, and the spots are indexed row-wise.

- **Overlap Checking Algorithm:**
  - The algorithm checks for overlaps by comparing the corresponding bits in the `block` (which represents the new word to be placed) and the `board` (which represents the current state of the board). 
  - For a word to be placed without invalid overlap, its corresponding bits in the `block` must not collide with filled spots in the `board`.

- **Word Crossing Detection:**
  - The algorithm uses the positions of the bits to detect crossings. A crossing occurs when the lines formed by two words form an `X`, overlapping eachother. Imagine you're playing snake: the snake cannot cross itself. Likewise, no two words can cross.
  - By checking the bits around the intended placement of a word, the algorithm ensures that no illegal crossings occur.

This bit-packed representation and algorithm allow for efficient checking of word placement, even on relatively large boards, ensuring that the puzzle is both compact in memory and operations run quickly.
