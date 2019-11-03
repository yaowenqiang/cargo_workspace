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

    let mut slice_demo = vec![1,2,3,4];
    let s = &mut slice_demo[..];
    let (cc, dd) = split_at_mut(s, 2);
    println!("{:?} {:?}",cc, dd);

    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];
    let (a,b) = r.split_at_mut(3);
    println!("{:?}", a);
    println!("{:?}", b);
    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}",  abs(-3));
    }
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

extern "C" {
    fn abs(input: i32) -> i32;
}


