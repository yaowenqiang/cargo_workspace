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


    let mut ss = String::from("hello world");
    ss.push_str(", are you ok?");
    println!("{}", ss);
    let s = String::from("Hello");
    take_ownership(s);
    //println!("after take_ownership, {}", s);, won't work,
    //

    let sss = String::from("I am back");
    let sss1 = takes_and_gives_back(sss);
    println!("{}", sss1);

    let x = 5;

    make_copy(x);

    let a_string = give_ownership();

    println!("{}", a_string);

    let (ss2, len) = calculate_length(ss);

    println!("the length of {} is {}", ss2, len);
    //println!("{}", ss);
    
    println!("length of ss2 is {}", calculate_lengthv2(&ss2))



    
}
/*
struct Table {
    value: i32,
    next: &Table,
}
*/

fn take_ownership(some_string: String) {
    println!("take_ownership: {}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("make_copy: {}", some_integer);
}

fn takes_and_gives_back(a_string: String)  -> String {
    a_string
}

fn give_ownership() ->String {
    let a_string = String::from("hello world a string");
    a_string
}

fn calculate_length(s: String) -> (String, usize) 
{
    let length = s.len();
    (s,length)
}


fn calculate_lengthv2(s: &String) -> usize 
{
    s.len()
}
