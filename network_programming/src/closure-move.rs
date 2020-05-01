fn main() {
    let mut times = 2;
    {
        let mut borrow = | x | times += x;
        borrow(5);
        borrow(5);
        borrow(5);
        borrow(5);
        borrow(5);
    }
    //times = 10;

    //assert_eq!(times, 7);

    //copy times to closure
    let mut own = move | x | times += x;println!("{}", times);
    own(5);
    own(5);
    own(5);
    own(5);
    //assert_eq!(times, 7);

    println!("{}", times);
}
