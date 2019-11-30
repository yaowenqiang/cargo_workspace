
fn main() {
    let mut v : Vec<i32>  = Vec::new();

    println!("{:?}", v);
    v.push(33);
    println!("{:?}", v);

    let vv = vec!["abc", "def"];
    println!("{:?}", vv);


    let vvi = vec![1,2,3,4];
    let third: &i32 = &vvi[2];
    println!("{}", third);

    match vvi.get(2) {
        Some(third) => println!("{}", third),
        None => println!("there is not third element")
    }


}
