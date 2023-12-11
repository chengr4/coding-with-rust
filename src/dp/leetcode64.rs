pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let cols = grid[0].len();
    let rows = grid.len();

    let mut res_matrix: Vec<Vec<i32>> = vec![vec![i32::MAX; cols + 1]; rows + 1];
    res_matrix[rows][cols - 1] = 0;

    for row in (0..rows).rev() {
        for col in (0..cols).rev() {
            res_matrix[row][col] = grid[row][col] + res_matrix[row + 1][col].min(res_matrix[row][col + 1]);
        }
    }

    res_matrix[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_path_sum() {
        assert_eq!(
            min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
        assert_eq!(min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]), 12);
    }
}
