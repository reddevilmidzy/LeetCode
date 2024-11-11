impl Solution {
    pub fn prime_sub_operation(mut nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mv = *nums.iter().max().unwrap();
        let primes = Self::get_primes(mv as usize);

        let idx = primes.binary_search(&(nums[0] as usize));

        if idx.is_ok() && idx.unwrap() != 0 {
            nums[0] -= primes[idx.unwrap() - 1] as i32;
        } else if idx.is_err() && idx.unwrap_err() != 0 {
            nums[0] -= primes[idx.unwrap_err() - 1] as i32;
        }

        for i in 1..n {
            let idx = primes.binary_search(&((nums[i] - nums[i-1]) as usize));
            if idx.is_ok() && idx.unwrap() != 0 {
               nums[i] -= primes[idx.unwrap() - 1] as i32;
            } else if idx.is_err() && idx.unwrap_err() != 0 {
                nums[i] -= primes[idx.unwrap_err() - 1] as i32;
            }
        }

        nums.windows(2).all(|w| w[0] < w[1])
    }

    fn get_primes(k: usize) -> Vec<usize> {
        let mut is_prime: Vec<bool> = vec![true; k + 1];
        is_prime[0] = false;
        is_prime[1] = false;

        for i in 2..=((k as f32).sqrt().ceil() as usize) {
            if is_prime[i] {
                for j in (i*i..=k).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }

        (2..=k).filter(|&i| is_prime[i]).collect()
    }
}
