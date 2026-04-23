#[allow(dead_code, unused, unused_imports)]
use std::{ mem::MaybeUninit, ptr };
mod raw;
use raw::{ StaticVecRaw };

pub struct StaticVec<T, const CAP: usize> {
    pub data: [MaybeUninit<T>; CAP],
    pub len: usize,
}

impl<T, const CAP: usize> StaticVec<T, CAP> {
    pub fn new() -> Self {
        StaticVec {
            data: unsafe {
                MaybeUninit::uninit().assume_init()
            },
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        CAP
    }

    pub fn push(&mut self, value: T) {
        if self.len >= CAP {
            panic!("Vector is full");
        }

        unsafe {
            let slot = self.data.as_mut_ptr().add(self.len) as *mut T;
            ptr::write(slot, value);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                let slot = self.data.as_mut_ptr().add(self.len) as *mut T;
                Some(ptr::read(slot))
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            unsafe {
                let slot = &self.data[index];
                Some(slot.assume_init_ref())
            }
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            None
        } else {
            unsafe {
                let slot = &mut self.data[index];
                Some(slot.assume_init_mut())
            }
        }
    }
}

impl<T, const CAP: usize> Drop for StaticVec<T, CAP> {
    fn drop(&mut self) {
        for i in 0..self.len {
            unsafe {
                let slot = self.data.as_mut_ptr().add(i) as *mut T;
                ptr::drop_in_place(slot);
            }
        }
    }
}

impl<T, const CAP: usize> IntoIterator for StaticVec<T, CAP> {
    type Item = T;
    type IntoIter = IntoIter<T, CAP>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            vec: self,
            index: 0,
        }
    }
}

pub struct IntoIter<T, const CAP: usize> {
    vec: StaticVec<T, CAP>,
    index: usize,
}

impl<T, const CAP: usize> Iterator for IntoIter<T, CAP> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len {
            unsafe {
                let slot = self.vec.data.as_ptr().add(self.index) as *mut T;
                self.index += 1;
                Some(ptr::read(slot))
            }
        } else {
            None
        }
    }
}

impl<T, const CAP: usize> Drop for IntoIter<T, CAP> {
    fn drop(&mut self) {
        for i in self.index..self.vec.len {
            unsafe {
                let slot = self.vec.data.as_mut_ptr().add(i) as *mut T;
                ptr::drop_in_place(slot);
            }
        }
        self.vec.len = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_vec_is_empty() {
        let v: StaticVec<i32, 10> = StaticVec::new();
        assert_eq!(v.len(), 0);
        assert!(v.is_empty());
        assert_eq!(v.capacity(), 10);
    }

    #[test]
    fn push_and_pop() {
        let mut v = StaticVec::<i32, 5>::new();

        v.push(1);
        v.push(2);
        v.push(3);

        assert_eq!(v.len(), 3);
        assert_eq!(v.pop(), Some(3));
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.pop(), None);
    }

    #[test]
    fn get_and_get_mut() {
        let mut v = StaticVec::<String, 10>::new();

        v.push("hello".to_string());
        v.push("world".to_string());

        assert_eq!(v.get(0), Some(&"hello".to_string()));
        assert_eq!(v.get(2), None);

        if let Some(s) = v.get_mut(1) {
            s.push('!');
        }

        assert_eq!(v.get(1), Some(&"world!".to_string()));
    }

    #[test]
    #[should_panic(expected = "Vector is full")]
    fn push_beyond_capacity_panics() {
        let mut v = StaticVec::<i32, 2>::new();
        v.push(1);
        v.push(2);
        v.push(3); // fat jayega
    }

    #[test]
    fn drop_runs_on_elements() {
        use std::rc::Rc;
        use std::cell::Cell;

        struct DropCounter(Rc<Cell<usize>>);
        impl Drop for DropCounter {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }

        let counter = Rc::new(Cell::new(0));
        let mut v = StaticVec::<DropCounter, 5>::new();

        v.push(DropCounter(counter.clone()));
        v.push(DropCounter(counter.clone()));

        assert_eq!(counter.get(), 0);
        drop(v);
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn into_iter_works() {
        let mut v = StaticVec::<i32, 10>::new();
        v.push(1);
        v.push(2);
        v.push(3);

        let sum: i32 = v.into_iter().sum();
        assert_eq!(sum, 6);
    }

    // ===================
    // ===================
    // ===================

    #[test]
    fn new_vec_is_empty_raw() {
        let v: StaticVecRaw<i32, 10> = StaticVecRaw::new();
        assert_eq!(v.len(), 0);
        assert!(v.is_empty());
        assert_eq!(v.capacity(), 10);
    }

    #[test]
    fn push_and_pop_raw() {
        let mut v = StaticVecRaw::<i32, 5>::new();

        v.push(1);
        v.push(2);
        v.push(3);

        assert_eq!(v.len(), 3);
        assert_eq!(v.pop(), Some(3));
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.pop(), None);
    }

    #[test]
    fn get_and_get_mut_raw() {
        let mut v = StaticVecRaw::<String, 10>::new();

        v.push("hello".to_string());
        v.push("world".to_string());

        assert_eq!(v.get(0), Some(&"hello".to_string()));
        assert_eq!(v.get(2), None);

        if let Some(s) = v.get_mut(1) {
            s.push('!');
        }

        assert_eq!(v.get(1), Some(&"world!".to_string()));
    }

    #[test]
    #[should_panic(expected = "Vector is full")]
    fn push_beyond_capacity_panics_raw() {
        let mut v = StaticVecRaw::<i32, 2>::new();
        v.push(1);
        v.push(2);
        v.push(3); // fat jayega
    }

    #[test]
    fn drop_runs_on_elements_raw() {
        use std::rc::Rc;
        use std::cell::Cell;

        struct DropCounter(Rc<Cell<usize>>);
        impl Drop for DropCounter {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }

        let counter = Rc::new(Cell::new(0));
        let mut v = StaticVecRaw::<DropCounter, 5>::new();

        v.push(DropCounter(counter.clone()));
        v.push(DropCounter(counter.clone()));

        assert_eq!(counter.get(), 0);
        drop(v);
        assert_eq!(counter.get(), 2);
    }
}
