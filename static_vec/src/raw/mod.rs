use core::panic;
use std::mem::MaybeUninit;

pub struct StaticVecRaw<T, const CAP: usize> {
    data: [MaybeUninit<T>; CAP],
    len: usize,
}

// implementing some methods on the StaticVecRaw
impl<T, const CAP: usize> StaticVecRaw<T, CAP> {
    pub fn new() -> Self {
        StaticVecRaw { data: unsafe { MaybeUninit::uninit().assume_init() }, len: 0 }
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
    //push
    pub fn push(&mut self, value: T) {
        if self.len >= CAP {
            panic!("The Vector is full");
        }

        // let slot = self.data.as_mut_ptr() as *mut T;
        // unsafe {
        //     *slot = value;
        // }
        // self.len += 1;
        self.data[self.len].write(value);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;

        // let slot = self.data.as_ptr().add(self.len) as *const T;
        // Some(slot.read())
        Some(unsafe { self.data[self.len].assume_init_read() })
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            // let slot = &self.data[index];
            // Some(&*(slot as *const MaybeUninit<T> as *const T))
            Some(unsafe { self.data[index].assume_init_ref() })
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            None
        } else {
            // unsafe {
            //     let slot = &mut self.data[index];
            //     Some(&mut *(slot as *mut MaybeUninit<T> as *mut T))
            // }
            Some(unsafe { self.data[index].assume_init_mut() })
        }
    }
}

impl<T, const CAP: usize> Drop for StaticVecRaw<T, CAP> {
    fn drop(&mut self) {
        for i in 0..self.len {
            unsafe {
                self.data[i].assume_init_drop();
            }
        }
    }
}
