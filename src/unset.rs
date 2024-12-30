/*use std::mem::swap;

#[derive(Default)]
pub struct Unset<T>(Vec<T>);

impl<T> Unset<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn push(&mut self, x: T) {
        self.0.push(x);
    }
    pub fn remove(&mut self, idx: usize) {
        swap(&mut self.0[idx], &mut self.0.last().unwrap());
        self.0.pop();
    }
}*/
