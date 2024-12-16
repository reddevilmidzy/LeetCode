impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut res = Vec::from(nums);
        let n : usize = res.len();

        for _ in 0..k {
            let min_val = res.iter().min().unwrap();
            for i in 0..n {
                if res[i] == *min_val {
                    res[i] *= multiplier;
                    break;
                }
            }
        }

        res
        
    }
}