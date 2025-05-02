impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let n = dominoes.len();
        let mut center = vec![0; n];

        for (i, c) in dominoes.chars().enumerate() {
            if c == 'R' {
                center[i] -= 1;
            } else if c == 'L' {
                center[i] += 1;
            }
        }

        let mut r = 0;
        let mut right = vec![-1; n];

        let mut l = 0;
        let mut left = vec![-1; n];
        
        for i in 0..n {
            if center[i] == -1 {
                r = 1;
            } else if r > 0 && center[i] == 0 {
                r += 1;
            } else {
                r = 0;
            }
            right[i] = r;

            if center[n-i-1] == 1 {
                l = 1;
            } else if l > 0 && center[n-i-1] == 0 {
                l += 1;
            } else {
                l = 0;
            }
            left[n-i-1] = l;
        }

        let mut res = String::new();

        for i in 0..n {
            if left[i] == right[i] {
                res.push('.');
            } else if left[i] == 0 {
                res.push('R');
            } else if right[i] == 0 || left[i] < right[i] {
                res.push('L');
            } else {
                res.push('R');
            }
        }
        
        res
    }
}
