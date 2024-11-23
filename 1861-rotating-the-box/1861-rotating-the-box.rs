impl Solution {
    pub fn rotate_the_box(b: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let n : usize = b.len();
        let m : usize = b[0].len();
        let mut res : Vec<Vec<char>> = vec![vec!['.'; n]; m];
        for i in 0..n {
            let mut k = m - 1;
            for j in (0..=k).rev() {
                if b[i][j] == '*' {
                    res[j][n-1-i] = b[i][j];
                    k = j - 1;
                } else if b[i][j] == '#' {
                    res[k][n-1-i] = b[i][j];
                    k -= 1;
                }
            }
        }

        res
    }
}
