use std::hash::Hash;
use std::sync::Arc;

use crate::HashSetError;

pub struct MultiHashElement<V: Hash + PartialEq + Clone> {
    pub next: Option<Arc<MultiHashElement<V>>>,
    pub count: usize,
    pub value: V,
}

impl<V: Hash + PartialEq + Clone> MultiHashElement<V> {
    pub fn new(val: V) -> Self {
        Self {
            next: None,
            count: 1,
            value: val,
        }
    }

    pub fn append(&mut self, next: MultiHashElement<V>) {
        if self.value == next.value {
            self.count += 1;
            return;
        }

        if let Some(mut next_element) = self.next.as_mut() {
            let next_element = Arc::get_mut(&mut next_element).unwrap();
            next_element.append(next);
        } else {
            self.next = Some(Arc::new(next));
        }
    }

    pub fn get(&self, lookup: &V) -> Option<&MultiHashElement<V>> {
        if &self.value == lookup {
            return Some(self);
        }

        if let Some(next_element) = &self.next {
            return next_element.get(lookup);
        } else {
            return None;
        }
    }

    pub fn cummulate(&self, buffer: &mut Vec<V>) {
        buffer.push(self.value.clone());
        if let Some(next_element) = &self.next {
            next_element.cummulate(buffer);
        }
    }

    pub fn remove(&mut self, value: V) -> Result<(), HashSetError> {
        if self.next.is_none() {
            return Err(HashSetError::RemoveError);
        }
        if let Some(mut next_element) = self.next.as_mut() {
            let next_element = Arc::get_mut(&mut next_element).unwrap();
            if next_element.value == value {
                if let Some(over_next) = &next_element.next {
                    self.next = Some(Arc::clone(&over_next));
                } else {
                    self.next = None;
                }
                return Ok(());
            }
            return next_element.remove(value);
        }
        return Err(HashSetError::RemoveError);
    }
}
