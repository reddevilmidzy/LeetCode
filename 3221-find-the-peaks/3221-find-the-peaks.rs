impl Solution {
    pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
        let n: usize = mountain.len();
        let mut res: Vec<i32> = Vec::new();

        for i in 1..n-1 {
            if mountain[i-1] < mountain[i] && mountain[i] > mountain[i+1] {
                res.push(i as i32);
            }
        }
        res
    }
}
