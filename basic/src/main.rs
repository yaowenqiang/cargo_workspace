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
    
    //println!("{}", s);

}
