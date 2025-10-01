impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut res = num_bottles;
        let mut num = num_bottles;

        while num > 0 {
            res += num / num_exchange;
            num = (num / num_exchange) + (num % num_exchange);
            
            if num < num_exchange {
                break;
            }
        }

        res
    }
}
