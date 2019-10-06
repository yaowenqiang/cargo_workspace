use crate::List::{Cons, Nil};
use std::ops::Deref;

/*
enum List {
    Cons(i32, List),
    Nil,
}
*/
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}


struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    //let list = Cons(3, Cons(2, Cons(3, Nil)));
    
    let list = Cons(1, 
        Box::new(Cons(2, 
            Box::new(Cons(3, 
                Box::new(Nil))))));

    println!("{:?}", list);

    let x = 5;
    let y = &x;
    let z = Box::new(x);
    assert_eq!(5 ,x);
    //assert_eq!(5 ,y);
    assert_eq!(5 ,*y);
    //assert_eq!(5 ,z);
    assert_eq!(5 ,*z);



    let xx  = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *xx);



    

}
