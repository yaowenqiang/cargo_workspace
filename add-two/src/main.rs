use crate::List::{Cons, Nil};

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


    

}
