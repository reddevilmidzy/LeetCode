impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let num1_ones = num1.count_ones();
        let mut num2_ones = num2.count_ones();

        if num1_ones == num2_ones {
            return num1;
        }
        if num1_ones > num2_ones {
            // 큰거만 일단 같게하기
            let mut res = 0;
            let mut tmp = num1;
            for i in (0..31).rev() {
                if (1<<i) <= tmp {
                    tmp -= (1 << i);
                    res += (1 << i);
                    num2_ones -= 1;
                    if num2_ones == 0 {
                        break;
                    }
                }
            }
            return res;
        }
            
        // 같은건 일단 같게 하고, 중간에 0 이 있다면 아래부터 추가
        let mut bits = vec![false; 31];
        let mut tmp = num1;
        for i in (0..31).rev() {
            if (1<<i) <= tmp {
                tmp -= (1 << i);
                bits[i] = true;
            }
        }
        let mut cnt = num2_ones - num1_ones;
        let mut res = num1;
        for i in 0..31 {
            if !bits[i] {
                res += (1<<i);
                cnt -= 1;
                if cnt == 0 {
                    break;
                }
            }
        }
        res
    }
}