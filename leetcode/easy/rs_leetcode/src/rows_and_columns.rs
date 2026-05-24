struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();

        for i in 0..n {
            let mut row = HashSet::new();
            let mut col = HashSet::new();
            for j in 0..n {
                row.insert(matrix[i][j]);
                col.insert(matrix[j][i]);
            }
            if row.len() != n || col.len() != n {
                return false;
            }
        }

        true
    }
}
