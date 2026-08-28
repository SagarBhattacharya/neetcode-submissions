struct MyHashSet {
    array: Vec<Vec<i32>>
}

impl MyHashSet {
    pub fn new() -> Self {
        Self {
            array: vec![vec![]; 1000]
        }
    }

    fn hash(key: i32) -> i32 {
        (key % 1000) as i32
    }

    pub fn add(&mut self, key: i32) {
        let index = Self::hash(key) as usize;
        if !self.array[index].contains(&key) {
            self.array[index].push(key);
        }
    }

    pub fn remove(&mut self, key: i32) {
        let index = Self::hash(key) as usize;
        if let Some(pos) = self.array[index].iter().position(|x| *x == key) {
            self.array[index].remove(pos);
        }
    }

    pub fn contains(&self, key: i32) -> bool {
        let index = Self::hash(key) as usize;
        self.array[index].contains(&key)
    }
}
