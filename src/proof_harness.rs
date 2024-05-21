#[cfg(kani)]
mod proof_harness {
    fn bit_overlaps(existing: usize, new_indices: usize) -> bool {
        existing & new_indices != 0
    }

    #[kani::proof]
    fn check_bit_overlaps() {
        let existing: usize = kani::any();
        let new_indices: usize = kani::any();

        let result = bit_overlaps(existing, new_indices);

        // Property 1: If existing and new_indices have any common bits, bit_overlaps should return true
        if (existing & new_indices) != 0 {
            assert!(result, "Expected true for existing: {:b}, new_indices: {:b}", existing, new_indices);
        }

        // Property 2: If existing and new_indices have no common bits, bit_overlaps should return false
        if (existing & new_indices) == 0 {
            assert!(!result, "Expected false for existing: {:b}, new_indices: {:b}", existing, new_indices);
        }

        // Edge Case 1: If new_indices is zero, the result should be false
        if new_indices == 0 {
            assert!(!result, "Expected false for existing: {:b} when new_indices is zero", existing);
        }

        // Edge Case 2: If existing is zero, the result should be false
        if existing == 0 {
            assert!(!result, "Expected false for new_indices: {:b} when existing is zero", new_indices);
        }
    }
}
