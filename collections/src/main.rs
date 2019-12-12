use std::collections::HashMap;
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

    let mut vv2 = vec![1,2,3,4,5];

    for i in &mut vv2 {
        *i += 60;
    }


    for i in &vv2 {
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

    let vvvv = vec![1,1,1,1,];

    for i in vvvv { 
        println!("{}", i);
    }

    /*
    for j in vvvv { 
        println!("{}", j);
    }
    */

    let vec1 = vec![1,2,3,4];
    let vec2 = vec![5,6,7,8];

    println!("2 in vec1  {}", vec1.iter().any(|&x|  x == 2));
    println!("2 in vec2  {}", vec1.into_iter().any(|x|  x == 2));


    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("Yello"), 10);

    println!("{:?}", scores);


    let teams = vec![String::from("blue"), String::from("yellow")];

    let initial_scores = vec![20, 30];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);












}
