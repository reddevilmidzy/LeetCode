impl Solution {
    pub fn distant_subarrays(nums: Vec<i32>, goal: i32, k: i32) -> i64 {

        fn solve(nums: &mut [i64], lo: i64, hi: i64) -> i64 {
            if nums.len() <= 1 {
                return 0;
            }
            if lo == hi {
                return 0;
            }

            let mid = nums.len() >> 1;

            let mut res = solve(&mut nums[..mid], lo, hi) + solve(&mut nums[mid..], lo, hi);
            let mut k = 0;
            let mut j = 0;

            for r in mid..nums.len() {
                let x = nums[r];
                while k < mid && nums[k] < x - lo {
                    k += 1;
                }

                while j < mid && nums[j] <= x - hi {
                    j += 1;
                }

                res += (j - k) as i64;
            }
            let n = nums.len();
            let mut tmp = Vec::with_capacity(n);

            let mut i = 0;
            let mut j = mid;

            while i < mid && j < n {
                if nums[i] < nums[j] {
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
        let n = nums.len();
        let mut pre = Vec::with_capacity(n + 1);
        pre.push(0i64);
        for i in 0..n {
            pre.push(pre[i] + nums[i] as i64);
        }

        let goal = goal as i64;
        let k = k as i64;

        let n = n as i64;

        (n * (n + 1) / 2) + solve(&mut pre, goal - k, goal + k)
    }
}
