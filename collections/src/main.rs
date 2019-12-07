
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

    for i in &vvi {
        println!("{}", i);
    }




    let ss = "Hello world";
    let hello =  &ss[0..=4];
    let hello2 =  &ss[0..4];
    let world =  &ss[0..=10];

    println!("{}", hello);
    println!("{}", hello2);
    println!("{}", world);


    match vvi.get(2) {
        Some(third) => println!("{}", third),
        None => println!("there is not third element")
    }


}
