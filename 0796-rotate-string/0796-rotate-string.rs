impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        return s.len() == goal.len() && (s.clone() + &s).contains(&goal)
    }
}
