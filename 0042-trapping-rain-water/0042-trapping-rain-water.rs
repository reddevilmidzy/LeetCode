impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut max_left = vec![0; n];
        let mut max_right = vec![0; n];

        for i in 1..n {
            max_left[i] = max_left[i-1].max(height[i-1]);
            max_right[n-i-1] = max_right[n-i].max(height[n-i]);
        }

        let mut res = 0;

        for i in 0..n {
            let val = max_left[i].min(max_right[i]);
            if val >= height[i] {
                res += val - height[i];
            }
        }
        res
    }
}
