// TODO: There is a non-thread-save error in this code
// Change the iterator to always take the next element and not create a list at first and then do something else
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::Deref;

pub struct MultiHashSet<V: Hash + PartialEq + Clone> {
    size: usize,
    expansion_factor: f32,
    used: usize,
    content: Vec<Option<MultiHashElement<V>>>
}

struct MultiHashElement<V: Hash + PartialEq + Clone> {
    next: *mut MultiHashElement<V>,
    count: usize,
    value: V
}

pub struct MultiHashSetIterator<V: Hash + PartialEq + Clone> {
    content: Vec<V>,
    current_index: usize,
}


impl<V: Hash + PartialEq + Clone> MultiHashSet<V> {

    pub fn new() -> Self {
        let standard_size = 10;
        let default_expansion_factor = 0.75;

        let empty_vec: Vec<Option<MultiHashElement<V>>> = (0..standard_size).map(|_| None).collect();
        Self { size: standard_size, expansion_factor: default_expansion_factor, content: empty_vec, used: 0 }
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
        self.resize_check();
        let hashed = self.hash(&value);
        let pos = hashed as usize % self.size;
        let new_content = MultiHashElement::new(value);
        self.put_or_append(pos, new_content);
    }

    fn put_or_append(&mut self, position: usize, value: MultiHashElement<V>) {
        if self.content[position].is_none() {
            self.content[position] = Some(value);
            self.used += 1;
        } else if let Some(element) = self.content[position].as_mut() {
            element.append(value);
        }
    }


    pub fn remove(&mut self, value: &V) -> bool {
        let hashed = self.hash(value) as usize;
        let pos = hashed % self.size;

        if let Some(content) = &mut self.content[pos] {
            if &content.value == value {
                if content.next.is_null() {
                    self.content[pos] = None;
                    return true;
                }

                let next_element = unsafe { &*content.next };

                let mut new_element = MultiHashElement::new(next_element.value.clone());
                new_element.next = next_element.next;
                new_element.count = next_element.count;

                self.content[pos] = Some(new_element);
                return true;
            }
            return content.remove(value);
        }
        false
    }


    pub fn count(&self, lookup: &V) -> usize {
        let hashed = self.hash(lookup);
        let pos = hashed as usize % self.size;
    
        if let Some(element) = &self.content[pos] {
            if let Some(found) = element.get(lookup) {
                return found.count;
            }
        }
        0
    }


    pub fn contains(&self, lookup: &V) -> bool {
        let hashed = self.hash(lookup);
        let pos = hashed as usize % self.size;

        if let Some(element) = &self.content[pos] {
            if let Some(_) = element.get(lookup) {
                return true
            }
        }
        false
        
    }


    fn resize_check(&mut self) { // TODO: Make this function scalable (potential overflow because of f64)
        if self.used as f64 > self.size as f64 * self.expansion_factor as f64 {
            self.size *= 2; // TODO: Change this to some more sensfull value

            self.reallocate_positions();
        }
    }

    fn reallocate_positions(&mut self) { // TODO: Look for a more efficient sollution
        let content: Vec<V> = self.traverse();
        self.content = (0..self.size).map(|_| None).collect();
        self.used = 0;

        for element in content {
            self.put(element)
        }
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

impl<V: Hash + PartialEq + Clone> MultiHashElement<V> {
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

    pub fn get(&self, lookup: &V) -> Option<&MultiHashElement<V>> {
        if &self.value == lookup {
            return Some(self);
        }

        if self.next.is_null() {
            return None;
        }

        let next_element = unsafe { &*self.next };
        next_element.get(lookup)
        
    }

    pub fn cummulate(&self, buffer: &mut Vec<V>) {
        buffer.push(self.value.clone());
        if self.next.is_null() {
            return;
        }
        let next_element = unsafe { &*self.next };
        next_element.cummulate(buffer);
    }


    pub fn remove(&mut self, value: &V) -> bool {
        if self.next.is_null() {return false;}
        let next_element = unsafe { &mut *self.next };
        if &next_element.value == value {
            self.next = next_element.next;
            return true;
        }
        next_element.remove(value);
        false
    }

}
