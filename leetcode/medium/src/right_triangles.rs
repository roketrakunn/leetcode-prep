struct Solution;

impl Solution {
    pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
 
        let rows = grid.len();
        let cols = grid[0].len();

        let mut row_count = vec![0i64; rows];
        let mut colum_count = vec![0i64; cols];
        let mut ans  = 0;

        for i in 0..rows { 
            for j in 0..cols { 
                if grid[i][j] == 1 { 
                    row_count[i] += 1;
                    colum_count[j] +=1;
                }
            }
        }

        for i in 0..rows { 
            for j in 0..cols { 
                if grid[i][j] == 1 { 
                    ans += (row_count[i] -1)  * (colum_count[j] -1);
                }
            }
        }
        ans
    }
}
