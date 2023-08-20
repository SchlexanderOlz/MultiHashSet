use std::hash::Hash;

pub struct MultiHashSetIterator<V: Hash + PartialEq + Clone> {
    pub content: Vec<V>,
    pub current_index: usize,
}

impl<'a, V: Hash + PartialEq + Clone> Iterator for MultiHashSetIterator<V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.content.len() {
            self.current_index += 1;
            return Some(self.content[self.current_index].clone());
        }
        None
    }
}
