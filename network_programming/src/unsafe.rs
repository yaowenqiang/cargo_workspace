fn main() {
    let num: u32 = 42;
    let p : *const u32 = &num;

    //assert_eq!(*p, num);

    unsafe {
        assert_eq!(*p, num);
    }
}
