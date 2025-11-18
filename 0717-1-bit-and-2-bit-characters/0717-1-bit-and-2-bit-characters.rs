impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let n = bits.len();

        let mut i = 0;
        while i < n {
            if bits[i] == 0 {
                i += 1;
            } else if i + 2 < n && bits[i + 1] == 0 { // 10
                i += 2;
            } else if i + 1 < n && bits[i + 1] == 1 { // 11
                i += 2;
            } else {
                return false;
            }
        }

        true
    }
}