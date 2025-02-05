impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let arr: Vec<_> = s1.chars().zip(s2.chars())
            .filter(|(a,b)| a != b)
            .collect();
        
        match arr.len() {
            0 => true,
            2 => arr[0].0 == arr[1].1 && arr[0].1 == arr[1].0,
            _ => false,
        }
    }
}