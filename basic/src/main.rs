fn main() {
    let heart_eyed_cat = '😻';
    println!("can you see a cat ?{}", heart_eyed_cat);
    {
    let s = "hello";
    println!("{}", s);
    let mut st = String::from("hello");
    st.push_str(", world");
    println!("{}", st);
    }

    let x = 5;
    let y = x;

    println!("{}", x);
    println!("{}", y);

    let s1 = String::from("hello world");
    let s2 = s1;
    let s3 = s2.clone();
    println!("{}", s2);
    println!("{}", s3);


    //println!("{}", s1);
    //println!("{}", s2);
    
    //println!("{}", s);
    /*

    let t1 = Table{
        value:1,
        next: &t2,
    };

    let t2 = Table {
        value:2,
        next: &t1,
    };
    */

    
}
/*
struct Table {
    value: i32,
    next: &Table,
}
*/
