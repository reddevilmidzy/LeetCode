use std::collections::HashMap;

struct RandomizedCollection {
    n: usize,
    cnt: HashMap<i32, i32>,
    nums: Vec<i32>,
    removed: HashMap<i32, i32>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedCollection {

    fn new() -> Self {
        Self {
            n: 0,
            cnt: HashMap::new(),
            nums: Vec::new(),
            removed: HashMap::new(),
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        self.n += 1;
        let cur = self.cnt.entry(val).or_default();
        self.nums.push(val);

        *cur += 1;
        *cur == 1
    }
    
    fn remove(&mut self, val: i32) -> bool {
        let cur = self.cnt.entry(val).or_default();
        if *cur == 0 {
            return false;
        }

        self.n -= 1;

        let rem = self.removed.entry(val).or_default();
        *rem += 1;

        *cur -= 1;
        if *cur == 0 {
            self.cnt.remove(&val);
        }
        true
    }
    
    fn get_random(&mut self) -> i32 {
        use rand::prelude::*;
        let old_n = self.nums.len();
        let idx: usize = rand::random::<usize>() % self.n;

        let val = self.nums[idx];
        if old_n == self.n {
            return val;
        }

        let rem = self.removed.entry(val).or_default();
        // ëë¤ì¼ë¡ ê±¸ë¦°ê² ì ê±° ëìì´ë©´ ì´ì  ì ê±°.
        if *rem != 0 {
            *rem -= 1;
            // ëì´ë ìì¹ ë°ê¿ì ì ê±°
            self.nums[idx] = self.nums[old_n - 1];
            self.nums.pop();
            self.get_random()
        } else {
            return val;
        }
    }
}

/**
 * Your RandomizedCollection object will be instantiated and called as such:
 * let obj = RandomizedCollection::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */