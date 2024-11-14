impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let mut st = 1;
        let mut ed = 100_000;
    
        while st < ed {
            let mid = st + (ed - st) / 2;
            if Self::count(mid, quantities.to_owned(), n) {
                ed = mid;
            } else {
                st = mid + 1;
            }
        }
        st
    }

    pub fn count(k: i32, nums: Vec<i32>, target: i32) -> bool {
        nums.iter().map(|&num| num / k + (num % k != 0) as i32).sum::<i32>() <= target
    }
}
