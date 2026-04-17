use std::mem::MaybeUninit;
use std::ptr;

fn main() {
    // Step 1- UNINITIALIZED array on stack.
    // The memory is allocated, but contains whatever random bits were there before.
    let mut buffer: [MaybeUninit<i32>; 5] = unsafe {
        // MaybeUninit::uninit() tells the compiler "skip initialization"
        // We assert the array type, then call .assume_init() which is UNSAFE
        // because the memory is full of garbage bits.
        MaybeUninit::uninit().assume_init()
    };

    // Step 2- Get a raw pointer to the start of the buffer.
    // This is safe because we're just getting an address, not reading anything.
    // if we read the compiler will screan on us
    let mut ptr = buffer.as_mut_ptr() as *mut i32;

    // Step 3- Fill the slots one by one.
    // We use unsafe because we're writing to memory that currently holds
    // uninitialized garbage.
    unsafe {
        // First slot
        ptr::write(ptr, 10);
        ptr = ptr.add(1);

        // Second slot
        ptr::write(ptr, 20);
        ptr = ptr.add(1);

        // Third slot
        ptr::write(ptr, 30);
        ptr = ptr.add(1);

        // Fourth slot
        ptr::write(ptr, 40);
        ptr = ptr.add(1);

        // Fifth slot
        ptr::write(ptr, 50);
        // No need to advance ptr after the last one
    }

    // Step 4- Convert to a normal array for safe use.
    // This is the magical part: we tell the compiler "every slot has been
    // properly initialized, you can treat it as a normal [i32; 5] now."
    let initialized_array = unsafe {
        // std::mem::transmute is the nuclear option.
        // Better approach: use pointer casting and std::ptr::read
        let array_ptr = &buffer as *const _ as *const [i32; 5];
        ptr::read(array_ptr)
    };

    println!("{:?}", initialized_array);
}
