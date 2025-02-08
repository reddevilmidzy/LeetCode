use std::collections::{BTreeSet, HashMap};

struct NumberContainers {
    map: HashMap<i32, i32>,
    sets: HashMap<i32, BTreeSet<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {

    fn new() -> Self {
        Self {
            map: HashMap::new(),
            sets: HashMap::new(),
        }
    }
    
    fn change(&mut self, index: i32, number: i32) {
        if let Some(num) = self.map.insert(index, number) {
            self.sets.get_mut(&num).unwrap().remove(&index);
        }
        self.sets.entry(number).or_default().insert(index);
    }
    
    fn find(&self, number: i32) -> i32 {
        self.sets
            .get(&number)
            .and_then(BTreeSet::first)
            .map_or(-1, i32::to_owned)
    }
}

/**
 * Your NumberContainers object will be instantiated and called as such:
 * let obj = NumberContainers::new();
 * obj.change(index, number);
 * let ret_2: i32 = obj.find(number);
 */