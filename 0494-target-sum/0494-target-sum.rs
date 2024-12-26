impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let n : usize = nums.len();
        let m : i32 = nums.iter().sum();
        let mut dp : Vec<i32> = vec![0; (m * 2 + 1) as usize];

        dp[(nums[0] + m) as usize] = 1;
        dp[(m - nums[0]) as usize] += 1;

        for i in 1..n {
            let mut nxt_dp = vec![0; (m * 2 + 1) as usize];
            for j in -m..=m {
                if dp[(j + m) as usize] > 0 {
                    nxt_dp[(j + nums[i] + m) as usize] += dp[(j + m) as usize];
                    nxt_dp[(j - nums[i] + m) as usize] += dp[(j + m) as usize];
                }
            }
            dp = nxt_dp;
        }
        
        if target.abs() > m {
            return 0;
        }
        dp[(m + target) as usize]
    }
}
