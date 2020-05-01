#[feature(inclusive_rnage_syntax)]

fn main() {
    let numbers = 1..5;
    for number in numbers {
        println!("{}", number);
    }

    println!("--------------------------------------------------------");

    let inclusive = 1..=5;
    for number in inclusive {
        println!("{}", number);
    }


}
