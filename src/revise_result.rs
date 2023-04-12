
pub struct ReviseResult{
    count: usize,
}

impl ReviseResult {
    pub fn new(d: usize) -> ReviseResult {
        ReviseResult{count: d}
    }
    pub fn result(&self) -> bool {
        self.count == 0
    }
    pub fn value(&self) -> usize {
        self.count
    }
}
