impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let n = differences.len();
        let mut tmp = vec![0; n + 1];    

        tmp[0] = lower as i64;
        let mut min_val = lower as i64;
        let mut max_val = lower as i64;

        for i in 1..=n {
            tmp[i] = tmp[i-1] + differences[i-1] as i64;
            min_val = min_val.min(tmp[i]);
            max_val = max_val.max(tmp[i]);
        }

        if max_val - min_val > (upper - lower) as i64 {
            return 0;
        }
        let diff = lower.abs_diff(min_val as i32) as i32;
        
        upper.abs_diff(max_val as i32 + diff) as i32 + 1
    }
}
