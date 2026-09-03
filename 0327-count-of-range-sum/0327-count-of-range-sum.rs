impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {

        fn solve(nums: &mut [i64], lo: i64, hi: i64) -> i64 {
            if nums.len() <= 1 {
                return 0;
            }

            let mid = nums.len() >> 1;

            let mut ans = solve(&mut nums[..mid], lo, hi) + solve(&mut nums[mid..], lo, hi);
            let mut k = 0;
            let mut j = 0;

            for r in mid..nums.len() {
                let x = nums[r];
                while k < mid && nums[k] < x - hi {
                    k += 1;
                }

                while j < mid && nums[j] <= x - lo {
                    j += 1;
                }

                ans += (j - k) as i64;
            }
            let n = nums.len();
            let mut tmp: Vec<i64> = Vec::with_capacity(n);

            let mut i = 0;
            let mut j = mid;
            // println!("before {:?}, {:?}", &nums[..mid], &nums[mid..]);
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
            // println!("after {nums:?}");

            ans
        }

        let n = nums.len();
        let mut pre: Vec<i64> = Vec::with_capacity(n + 1);
        pre.push(0);
        for i in 0..n {
            pre.push(pre[i] + nums[i] as i64);
        }
        // println!("{pre:?}");
        solve(&mut pre, lower as i64, upper as i64) as i32
    }
}
