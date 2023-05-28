
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct MultiHashSet<V: Hash + PartialEq> {
    size: usize,
    expansion_factor: f32,
    used: usize,
    content: Vec<Option<MultiHashElement<V>>>
}

struct MultiHashElement<V: Hash + PartialEq> {
    next: *mut MultiHashElement<V>,
    count: usize,
    value: V
}

pub struct MultiHashSetIterator<'a, V: Hash + PartialEq> {
    content: &'a Vec<Option<MultiHashElement<V>>>,
    current_index: usize,
}



impl<V: Hash + PartialEq> MultiHashSet<V> {

    pub fn new() -> Self {
        let standard_size = 10;
        let default_expansion_factor = 0.75;

        let empty_vec: Vec<Option<MultiHashElement<V>>> = (0..standard_size).map(|_| None).collect();
        Self { size: standard_size, expansion_factor: default_expansion_factor, content: empty_vec, used: 0 }
    }


    pub fn size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }


    pub fn expansion_factor(mut self, factor: f32) -> Self {
        self.expansion_factor = factor;
        self
    }

    pub fn iter(&self) -> MultiHashSetIterator<'_, V> {
        MultiHashSetIterator {
            content: &self.content,
            current_index: 0,
        }
    }

    pub fn put(&mut self, value: V) -> bool {
        self.resize_check();
        let hashed = self.hash(&value);
        let pos = hashed as usize % self.size;
    
        let new_content = MultiHashElement::new(value);
        if self.content[pos].is_none() {
            self.content[pos] = Some(new_content);
            self.used += 1;
            true
        } else if let Some(element) = self.content[pos].as_mut() {
            element.append(new_content);
            true
        } else {
            false
        }
    }

    pub fn count(&mut self, lookup: &V) -> usize {
        let hashed = self.hash(&lookup);
        let pos = hashed as usize % self.size;
    
        if let Some(element) = &mut self.content[pos] {
            if let Some(found) = element.get(&lookup) {
                return found.count;
            }
        }
        0
    }

    fn resize_check(&mut self) { // TODO: Make this function scalable (potential overflow because of f64)
        if self.used as f64 > self.size as f64 * self.expansion_factor as f64 {
            self.size *= 1 + self.expansion_factor as usize;

            let num_elements_to_add = self.size - self.content.len();
            self.content.extend((0..num_elements_to_add).map(|_| None));
        }
    }


    fn hash(&self, value: &V) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

}


impl<'a, V: Hash + PartialEq> Iterator for MultiHashSetIterator<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_index < self.content.len() {
            if let Some(element) = &self.content[self.current_index] {
                self.current_index += 1;
                return Some(&element.value);
            }
            self.current_index += 1;
        }
        None
    }
}

impl<V: Hash + PartialEq> MultiHashElement<V> {
    pub fn new(val: V) -> Self {
        Self {
            next: std::ptr::null_mut(),
            count: 1,
            value: val,
        }
    }

    pub fn append(&mut self, next: MultiHashElement<V>) {
        if self.value == next.value {
            self.count += 1;
            return;
        }

        if self.next.is_null(){
            self.next = Box::into_raw(Box::new(next));
            return;
        }
        let next_element = unsafe { &mut *self.next };
        next_element.append(next);
    }

    pub fn get(&mut self, lookup: &V) -> Option<&MultiHashElement<V>> {
        if &self.value == lookup {
            return Some(self)
        }

        if self.next == std::ptr::null_mut() {
            return None
        }

        let next_element = unsafe { &mut *self.next };
        next_element.get(lookup)
        
    }
}
