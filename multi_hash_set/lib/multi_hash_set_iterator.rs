use std::hash::Hash;

pub struct MultiHashSetIterator<V: Hash + PartialEq + Clone> {
    pub content: Vec<V>,
    pub current_index: usize,
}

impl<'a, V: Hash + PartialEq + Clone> Iterator for MultiHashSetIterator<V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_index < self.content.len() {
            if let element = &self.content[self.current_index] {
                self.current_index += 1;
                return Some(element.clone());
            }
            self.current_index += 1;
        }
        None
    }
}
