pub fn swap_numbers_unsafe(num1: &mut i32, num2: &mut i32) {
    // we need to swap these two wihout using the third pointer
    // basically just use the raw pointers

    unsafe {
        // let temp = ptr::read(num1);
        // ptr::write(num1, ptr::read(num2));

        // ptr::write(num2, temp);
        // this was using three variables

        let a = num1 as *mut i32;
        let b = num2 as *mut i32;

        // SAFETY:  if they point to the same memory .. XOR swap will zero out the value
        if a == b {
            return;
        }

        *a = *a ^ *b;
        *b = *a ^ *b;
        *a = *a ^ *b;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_swap() {
        let mut x = 5;
        let mut y = 10;
        swap_numbers_unsafe(&mut x, &mut y);
        assert_eq!(x, 10);
        assert_eq!(y, 5);
    }

    #[test]
    fn xor_neg_swap() {
        let mut x = 10;
        let mut y = -5;
        swap_numbers_unsafe(&mut x, &mut y);
        assert_eq!(x, -5);
        assert_eq!(y, 10);
    }

    #[test]
    fn same_location_does_nothing() {
        let mut x = 42;
        let ref1 = &mut x;

        let ref2 = unsafe {
            let ptr = ref1 as *mut i32;
            &mut *ptr
        };

        // Both ref1 and ref2 now point to the exact same i32 in memory.
        assert_eq!(ref1 as *mut i32, ref2 as *mut i32, "Pointers should be the same");
        swap_numbers_unsafe(ref1, ref2);
        // x should STILL be 42.
        assert_eq!(x, 42, "Value should remain unchanged ");
    }
}
