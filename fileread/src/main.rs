use std::env;
use std::fs;
#[derive(Debug)]
enum myList {
    ONE(String, String ,String ,String),
    TWO
}
fn main() {
    let contents = fs::read_to_string("my.txt").
        expect("failed to read file");
    println!("read result: {}", contents);
    /*
    let service = vec![
        myList::TWO(String::from("aaa"),String::from("two")),
        myList::ONE(String::from("aaa")),
    ];
    
    for i in service.iter() {
        println!("i");
    }
    */

    if let myList::ONE("abc".to_string(),"tow".to_string(),"three".to_string(),"four".to_string()) = value {
    };
}
