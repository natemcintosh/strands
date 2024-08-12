use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use strands::two_words_no_diag_overlap;

// Function to generate a mock block and board configuration.
// These are just placeholders; you may want to use actual configurations relevant to your use case.
fn generate_mock_data(board_width: usize, board_height: usize) -> (usize, usize) {
    // Generate mock data where all bits are set to 1
    // This is just for demonstration purposes
    let block = (1 << (board_width * board_height)) - 1;
    let board = (1 << (board_width * board_height)) - 1;
    (block, board)
}

fn bench_no_diagonal_overlap(c: &mut Criterion) {
    let mut group = c.benchmark_group("no_diagonal_overlap");

    // Loop through different board sizes from 2x2 to 6x8
    for &(width, height) in &[(2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (6, 8)] {
        // Generate mock data for the given board dimensions
        let (block, board) = generate_mock_data(width, height);

        // Create a benchmark ID for each board dimension
        group.bench_with_input(
            BenchmarkId::new("Board Size", format!("{}x{}", width, height)),
            &(block, board),
            |b, &(block, board)| {
                b.iter(|| {
                    // Benchmark the no_diagonal_overlap function with the given block and board
                    two_words_no_diag_overlap(block, board, width, height)
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_no_diagonal_overlap);
criterion_main!(benches);
