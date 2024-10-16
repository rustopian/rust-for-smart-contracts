// Flip the Table Exercise

/// Flips (transposes) a square table (vector of vectors).
/// The table is generic over T and must implement Clone.
///
/// # Arguments
///
/// * `table` - A vector of vectors representing the table to flip.
///
/// # Returns
///
/// A new table that is the transposed version of the input table.
pub fn flip_table<T: Clone>(table: Vec<Vec<T>>) -> Vec<Vec<T>> {
    // TODO: Implement this function
    unimplemented!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_table() {
        let table = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let flipped = flip_table(table);
        assert_eq!(
            flipped,
            vec![
                vec![1, 4, 7],
                vec![2, 5, 8],
                vec![3, 6, 9],
            ]
        );
    }
}