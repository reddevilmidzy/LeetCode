impl Solution {
    pub fn calc_time(ranks: &Vec<i64>, target: i64, cars: i64) -> bool {
        let mut tot = 0;
        for rank in ranks {
            let tmp = ((target / *rank) as f64).sqrt().floor() as i64;
            tot += tmp;
        }
        tot >= cars
    }

    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let ranks: Vec<i64> = ranks.iter().map(|x| *x as i64).collect();
        let cars = cars as i64;
        let mut low: i64 = 1;
        let mut high: i64 = 100_000_000_000_000;
        
        let mut res = 0;

        while low <= high {
            let mid = (low + high) / 2;
            if Self::calc_time(&ranks, mid, cars) {
                high = mid - 1;
                res = mid;
            } else {
                low = mid + 1;
            }
        }

        res
    }
}
