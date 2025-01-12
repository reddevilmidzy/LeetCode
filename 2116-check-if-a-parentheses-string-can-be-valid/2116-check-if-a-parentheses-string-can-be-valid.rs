impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let n: usize = s.len();
        if n%2 != 0 {
            return false;
        }
        let mut cnt = 0;
        let mut unlocked = 0;
        let locked: Vec<bool> = locked.chars().map(|x| x == '1').collect();

        let mut stk = Vec::new();
        let mut unlocked = Vec::new();

        for (idx, c) in s.chars().enumerate() {
            if !locked[idx] {
                unlocked.push(idx);
            } else if c == '(' {
                if idx == n-1 {
                    return false;
                }
                stk.push(idx); 
            } else if c == ')' {
                if !stk.is_empty() {
                    stk.pop();
                } else if !unlocked.is_empty() {
                    unlocked.pop();
                } else {
                    return false;
                }
            }
        }

        while !stk.is_empty() && !unlocked.is_empty() && *stk.last().unwrap() < *unlocked.last().unwrap() {
            stk.pop();
            unlocked.pop();
        }

        stk.is_empty()
    }
}