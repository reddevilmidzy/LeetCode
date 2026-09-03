impl Solution {
    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        fn solve(nums: &mut [i32]) -> i32 {
            let n = nums.len();
            if n <= 1 {
                return 0;
            }
            
            let mid = n >> 1;

            let mut res = solve(&mut nums[..mid]) + solve(&mut nums[mid..]);
            // [1, 2,3] , [1, 3]

            let mut j = 0;

            for r in mid..n {
                let x = nums[r] as i64;

                while j < mid && 2 * x >= nums[j] as i64 {
                    j += 1;
                }
                res += (mid - j) as i32;
            }

            let mut tmp = Vec::with_capacity(n);

            let mut i = 0;
            let mut j = mid;

            while i < mid && j < n {
                if nums[i] <= nums[j] {
                    tmp.push(nums[i]);
                    i += 1;
                } else {
                    tmp.push(nums[j]);
                    j += 1;
                }
            }
            while i < mid {
                tmp.push(nums[i]);
                i += 1;
            }
            while j < n {
                tmp.push(nums[j]);
                j += 1;
            }
            
            nums.copy_from_slice(&tmp);

            res
        }
        
        solve(&mut nums)
    }
}
