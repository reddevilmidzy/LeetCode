impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut neg = 0;
        let mut min_num : i64 = i64::MAX;
        let mut tot : i64 = 0;

        for row in matrix {
            for val in row {
                if val < 0 {
                    neg += 1;
                }
                tot += val.abs() as i64;
                min_num = min_num.min(val.abs() as i64);
            }
        }
        if neg % 2 != 0 {
            tot -= min_num*2;
        }
        tot
    }
}
