// TODO: There is a non-thread-save error in this code
// Change the iterator to always take the next element and not create a list at first and then do something else
mod multi_hash_element;
mod multi_hash_set_iterator;

use multi_hash_element::MultiHashElement;
use multi_hash_set_iterator::MultiHashSetIterator;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct MultiHashSet<V: Hash + PartialEq + Clone> {
    size: usize,
    expansion_factor: f32,
    used: usize,
    content: Vec<Option<MultiHashElement<V>>>,
}

impl<V: Hash + PartialEq + Clone> MultiHashSet<V> {
    pub fn new() -> Self {
        let standard_size = 11;
        let default_expansion_factor = 0.75;

        let empty_vec: Vec<Option<MultiHashElement<V>>> =
            (0..standard_size).map(|_| None).collect();
        Self {
            size: standard_size,
            expansion_factor: default_expansion_factor,
            content: empty_vec,
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

        if self.content[pos].is_none() {
            self.content[pos] = Some(new_content);
            self.used += 1;
        } else if let Some(element) = self.content[pos].as_mut() {
            element.append(new_content);
        }
    }

    pub fn remove(&mut self, value: &V) -> bool {

        let pos = {
            let hashed = self.hash(value) as usize;
            hashed % self.size
        };

        if let Some(content) = &mut self.content[pos] {
            if &content.value == value {
                if content.next.is_null() {
                    self.content[pos] = None;
                    return true;
                }

                // This get's the "next" element of the HashElement. It instanciates it
                self.content[pos] = {
                    let next_element = unsafe { &*content.next };

                    let mut new_element = MultiHashElement::new(next_element.value.clone());
                    new_element.next = next_element.next;
                    new_element.count = next_element.count;
                    Some(new_element)
                };

                return true;
            }
            return content.remove(value);
        }
        false
    }

    pub fn count(&self, lookup: &V) -> usize {
        let pos = {
            let hashed = self.hash(lookup) as usize;
            hashed % self.size
        };

        if let Some(element) = &self.content[pos] {
            if let Some(found) = element.get(lookup) {
                return found.count;
            }
        }
        0
    }

    pub fn contains(&self, lookup: &V) -> bool {
        let pos = {
            let hashed = self.hash(lookup) as usize;
            hashed % self.size
        };

        if let Some(element) = &self.content[pos] {
            return  element.get(lookup).is_some();
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
