// TODO: There is a non-thread-save error in this code
// Change the iterator to always take the next element and not create a list at first and then do something else
mod multi_hash_element;
mod multi_hash_set_iterator;

use multi_hash_element::MultiHashElement;
use multi_hash_set_iterator::MultiHashSetIterator;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

const STD_SIZE: usize = 11;
const EXPANSION: f32 = 0.75;

pub struct MultiHashSet<V: Hash + PartialEq + Clone> {
    size: usize,
    expansion_factor: f32,
    used: usize,
    content: Vec<Option<Arc<MultiHashElement<V>>>>,
}

impl<V: Hash + PartialEq + Clone> MultiHashSet<V> {
    pub fn new() -> Self {
        Self {
            size: STD_SIZE,
            expansion_factor: EXPANSION,
            content: (0..STD_SIZE).map(|_| None).collect(),
            used: 0,
        }
    }

    pub fn size(mut self, size: usize) -> Self {
        self.size = size;
        self.content = (0..self.size).map(|_| None).collect();
        self
    }

    pub fn expansion_factor(mut self, factor: f32) -> Self {
        self.expansion_factor = factor;
        self
    }

    pub fn get_size(&self) -> usize {
        return self.size;
    }

    pub fn iter(&self) -> MultiHashSetIterator<V> {
        let traverse_result = self.traverse();
        MultiHashSetIterator {
            content: traverse_result,
            current_index: 0,
        }
    }

    pub fn put(&mut self, value: V) {
        fn next_prime(start: u64) -> u64 {
            fn is_prime(n: u64) -> bool {
                if n <= 1 {
                    return false;
                }
                if n <= 3 {
                    return true;
                }
                if n % 2 == 0 || n % 3 == 0 {
                    return false;
                }
                let mut i = 5;
                while i * i <= n {
                    if n % i == 0 || n % (i + 2) == 0 {
                        return false;
                    }
                    i += 6;
                }
                true
            }

            let mut current = start + 1;
            while !is_prime(current) {
                current += 1;
            }
            current
        }

        // Resize check
        // Checks if the size of the Set needs to change
        if self.used as f64 > self.size as f64 * self.expansion_factor as f64 {
            self.size = next_prime(self.size as u64) as usize;

            let content: Vec<V> = self.traverse();
            self.content = (0..self.size).map(|_| None).collect();
            self.used = 0;

            for element in content {
                self.put(element)
            }
        }

        let pos = {
            let hashed = self.hash(&value);
            hashed as usize % self.size
        };

        let new_content = MultiHashElement::new(value);

        if let Some(element) = self.content[pos].as_mut() {
            let element = Arc::get_mut(element).unwrap();
            element.append(new_content);
        } else {
            self.content[pos] = Some(Arc::new(new_content));
            self.used += 1;
        }
    }

    pub fn remove(&mut self, value: V) -> Result<(), HashSetError> {
        let pos = {
            let hashed = self.hash(&value) as usize;
            hashed % self.size
        };

        if let Some(content) = self.content[pos].as_mut() {
            let content = Arc::get_mut(content).unwrap();
            if content.value == value {
                content.count -= 1;
                if content.count > 0 {
                    return Ok(());
                }

                if let Some(next_element) = &content.next {
                    self.content[pos] = Some(Arc::clone(next_element));
                } else {
                    self.content[pos] = None;
                }
                return Ok(());
            }
            return content.remove(value);
        }
        Err(HashSetError::RemoveError)
    }

    pub fn count(&self, lookup: V) -> usize {
        let pos = {
            let hashed = self.hash(&lookup) as usize;
            hashed % self.size
        };

        if let Some(element) = &self.content[pos] {
            if let Some(found) = element.get(&lookup) {
                return found.count;
            }
        }
        0
    }

    pub fn contains(&self, lookup: V) -> bool {
        let pos = {
            let hashed = self.hash(&lookup) as usize;
            hashed % self.size
        };

        if let Some(element) = &self.content[pos] {
            return element.get(&lookup).is_some();
        }
        false
    }

    fn hash(&self, value: &V) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    fn traverse(&self) -> Vec<V> {
        let mut content: Vec<V> = vec![];
        for element in &self.content {
            if let Some(exists) = &element {
                exists.cummulate(&mut content);
            }
        }
        content
    }
}

#[derive(Debug)]
pub enum HashSetError {
    RemoveError,
}
