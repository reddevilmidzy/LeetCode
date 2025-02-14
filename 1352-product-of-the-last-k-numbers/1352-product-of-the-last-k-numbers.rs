struct ProductOfNumbers {
    list: Vec<i32>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {

    fn new() -> Self {
        Self { list: Vec::new() }
    }
    
    fn add(&mut self, num: i32) {
        self.list.push(num);
    }
    
    fn get_product(&self, k: i32) -> i32 {
        let mut res = 1;
        let n = self.list.len();

        for i in 0..k as usize {
            res *= self.list[n - i - 1];
        }

        res
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */