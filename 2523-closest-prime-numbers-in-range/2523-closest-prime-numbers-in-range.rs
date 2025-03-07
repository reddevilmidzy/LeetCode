impl Solution {
    pub fn sieve(m: usize) -> Vec<i32> {
        let mut res = Vec::new();
        let mut is_prime = vec![true; m + 1];
        is_prime[0] = false;
        is_prime[1] = false;

        for i in 2..=(m as f32).sqrt().ceil() as usize {
            if is_prime[i] {
                for j in (i*2..=m).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }

        for i in 2..=m {
            if is_prime[i] {
                res.push(i as i32);
            }
        }

        res
    }

    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let primes = Self::sieve(right as usize);

        let l = match primes.binary_search(&left) {
            Ok(i) => i,
            Err(j) => j,
        };

        let r = match primes.binary_search(&right) {
            Ok(i) => i + 1,
            Err(j) => j,
        };
        let mut res = vec![-1, -1];
        let mut val = i32::MAX;
        if r-l < 1 {
            return res;
        }
        for i in l..r-1 {
            if primes[i+1] - primes[i] < val {
                val = primes[i+1] - primes[i]; 
                res = vec![primes[i], primes[i+1]];
            }
        }
        res
    }
}
