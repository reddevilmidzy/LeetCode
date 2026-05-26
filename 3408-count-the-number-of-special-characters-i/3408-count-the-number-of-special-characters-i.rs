impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut lo = vec![false; 26];
        let mut up = vec![false; 26];

        for c in word.chars() {
            if c.is_lowercase() {
                let i = (c as u8 as usize) - ('a' as u8 as usize);
                lo[i] = true;
            } else {
                let i = (c as u8 as usize) - ('A' as u8 as usize);
                up[i] = true;
            };
        }
        
        (0..26).filter(|x| lo[*x] && up[*x]).count() as i32
    }
}
