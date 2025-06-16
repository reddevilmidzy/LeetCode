impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        // ìíê°ë¥
        // ë²ë¨ ëë ëª¨ë¸í¤ì¤íì¼ë¡ ë¹ë¹ë¤.

        let mut stk = Vec::new();
        let mut res = -1;

        for num in nums {
            while !stk.is_empty() && *stk.last().unwrap() >= num {
                stk.pop();
            }
            stk.push(num);

            if stk.len() > 1 {
                res = res.max(stk[stk.len()-1] - stk[0]);
            }
        }
        
        res
    }
}
