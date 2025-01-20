use std::collections::HashMap;

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();

        let mut pos = vec![(0usize, 0usize); n * m + 1];

        for i in 0..n {
            for j in 0..m {
                pos[mat[i][j] as usize] = (i, j);
            }
        }
        let mut row = vec![0; n];
        let mut col = vec![0; m];

        for i in 0..(n*m) {
            let (y,x) = pos[arr[i] as usize];
            row[y] += 1;
            col[x] += 1;
            
            if row[y] == m {
                return i as i32;
            }
            if col[x] == n {
                return i as i32;
            }
        }

        (n * m) as i32
    }
}