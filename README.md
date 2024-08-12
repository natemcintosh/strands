# Strands

This CLI program attempts to solve the [strands](https://www.nytimes.com/games/strands) game by:
1. finding words that can exist on the board
1. finding the right combination of words that cover the board with no overlap, or crossing each other.

It can do these things reasonably quickly, depending on the number of words that can exist on the board.

**NB:** This solver knows nothing about the theme words, only what words in the dictionary can all fit on the board nicely. That means that it is entirely possible it will find a set of words that perfectly fills the board, but does not actually win the game.
