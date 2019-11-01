use std::slice;

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let address = 0x012345usize;
    let r = address as *const i32;
    
    unsafe {
        println!("r1 is: {} ", *r1);
        println!("r2 is: {} ", *r2);
    }


    println!("Hello, world!");

    unsafe {
        unsafe_function();
    }

    slice_demo = vec![1,2,3,4];
    slice_2 = split_at_mut(slice_demo, 2);
    println!("{} {}",a, b);
}

unsafe fn unsafe_function() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let address = 0x012345usize;
    let r = address as *const i32;
    
    //unsafe {
        println!("r1 is: {} ", *r1);
        println!("r2 is: {} ", *r2);
    //}


    println!("Hello, world!");

}

fn split_at_mut(slice: &mut [i32], mid: usize,) -> (&mut [i32], &mut[i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);
    /*
    (&mut slice[..mid],
     &mut slice[mid..]
        )
    */
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid)
        )
    }
}
