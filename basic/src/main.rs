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
    println!("length of ss2 is {}", calculate_lengthv2(&ss2));
   let mut ss3 = String::from("hello");
    println!("{}", ss3);
    change(&mut ss3);
    println!("{}", ss3);


    let mut ssss = String::from("mutable string");

    {
        let r1 = &mut ssss;
        //let r2 = &mut ssss; error, only one mutable reference  at the same scope is allowed
        r1.push_str("add mutable  string to the end\n");
        println!("{}", r1);
    }

    let r2 = &mut ssss;
    r2.push_str("add mutable  string to the end  the second time.");
    println!("{}", r2);


    //let reference_to_nothing = dangle();


    let slice1 = String::from("hello world");
    let slice2  = &slice1[0..5];
    let slice3  = &slice1[0..11];

    println!("{}", slice2);
    println!("{}", slice3);


    let slice4 = String::from("12345678");
    let slice5 = &slice4[..2];
    println!("{}", slice5);
    let slice5 = &slice4[..];
    println!("{}", slice5);
    let slice5 = &slice4[..1];
    println!("slice4[..1] = {}", slice5);

    let slice5 = &slice4[0..1];
    println!("slice4[0..1] = {}", slice5);
    //1

    let slice5 = &slice4[1..1];
    println!("slice4[1..1] = {}", slice5);
    //1 ??
    let slice5 = &slice4[2..2];
    println!("slice4[2..2] = {}", slice5);
    //"" ??

    let mut words = String::from("hello world");
    let mut first_word = first_word(&words);
    println!("{}", first_word);
    //words.clear();
    //println!("{}", first_word);

    let string_literal = "hello world";
    println!("{}", string_literal);


    
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


fn change(s: &mut String)  {
    s.push_str("haha");
}


/*
fn dangle() -> &String {
    let s = String::from("Hello");
    &s
}

*/

fn first_word(s: &str) ->  &str  {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn used_function() {
}




#[allow(dead_code)]
fn unused_function() {
}

