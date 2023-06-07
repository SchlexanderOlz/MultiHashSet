use std::hash::Hash;

pub struct MultiHashElement<V: Hash + PartialEq + Clone> {
    pub next: *mut MultiHashElement<V>,
    pub count: usize,
    pub value: V,
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

        if self.next.is_null() {
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
        if self.next.is_null() {
            return false;
        }
        let next_element = unsafe { &mut *self.next };
        if &next_element.value == value {
            self.next = next_element.next;
            return true;
        }
        next_element.remove(value);
        false
    }
}
