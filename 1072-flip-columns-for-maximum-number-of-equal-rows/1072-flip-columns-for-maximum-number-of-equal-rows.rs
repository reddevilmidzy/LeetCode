use std::collections::HashMap;
use std::ops::BitXor;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let mut cnt : HashMap<Vec<i32>, i32> = HashMap::new();

        for i in 0..n {
            cnt.insert(matrix[i].to_vec(), cnt.get(&matrix[i].to_vec()).unwrap_or(&0) + 1);
            let tmp : Vec<i32> = matrix[i].iter().map(|x| x.bitxor(1)).collect();
            cnt.insert(tmp.to_vec(), cnt.get(&tmp.to_vec()).unwrap_or(&0) + 1);
        }

        *cnt.values().max().unwrap()
    }
}