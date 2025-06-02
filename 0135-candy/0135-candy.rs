impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut nums = Vec::with_capacity(n);
        for i in 0..n {
            nums.push((ratings[i], i));
        }
        nums.sort_unstable();

        // println!("{:?}", nums);
        let min_val = nums[0].0;

        let mut res = vec![0; n];
        let mut i = 0;
        while i < n {
            if nums[i].0 == min_val {
                res[nums[i].1] = 1;
            } else {
                break;
            }
            i += 1;
        }

        while i < n {
            let (val, idx) = nums[i];
            let left = if idx == 0 { None } else { Some(ratings[idx - 1]) };
            let right = if idx == n-1 { None } else { Some(ratings[idx + 1]) };

            if left.is_some() && left.unwrap() < val {
                res[idx] = res[idx - 1] + 1;
            }
            if right.is_some() && right.unwrap() < val {
                res[idx] = res[idx].max(res[idx + 1] + 1);
            }
            if res[idx] == 0 {
                res[idx] = 1;
            }
            i += 1;
        }
        let mut ans = 0;
        for i in 0..n {
            ans += res[i];
        }
        ans
    }
}
