impl Solution {
    pub fn max_valid_splits(mut nums: Vec<i32>) -> i32 {
        fn solve(nums: &Vec<i32>) -> i32 {
            let mut res = 0;
            let n = nums.len();
            let mut forw = vec![0; n]; // n - 1 ê¹ì§ ê°ë©´ ìë¨
            let mut back = vec![0; n]; // 0 ê¹ì§ ê°ë©´ ìë¨

            let mut f = nums[0];
            let mut b = nums[n - 1];
            for i in 0..n {
                f = gcd(f, nums[i]);
                forw[i] = f;
                b = gcd(b, nums[n - i - 1]);
                back[n - i - 1] = b;
            }

            for i in 0..n - 1 {
                if forw[i] == back[i + 1] {
                    res += 1;
                }
            }

            res
        }
    
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                (a, b) = (b, a % b)
            }
            a
        }
        use std::collections::HashSet;
        let n = nums.len();
        let set = nums.iter().collect::<HashSet<_>>();
        if set.len() == 1  {
            return (n - 1) as i32;
        }

        //  no ì ê±°
        let mut res = solve(&nums);

        for del in 0..n {
            let rem = nums.remove(del);
            res = res.max(solve(&nums));
            nums.insert(del, rem);
        }

        res
    }
}
