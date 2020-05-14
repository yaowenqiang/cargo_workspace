fn divide(numerator:f64, denominator:f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

struct MyOption<T>(T);

fn main() {
    let result = divide(2.0, 3.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by 0."),
    }

    let msg = Some("howdy");

    println!("{:?}", msg);

    let MyOption(my_option) = MyOption("hello world");

    println!("{:?}", my_option);
    let MyOption(my_option) = MyOption(123456);

    println!("{:?}", my_option);
    //let my_option = MyOption(1.22);
    let MyOption(my_option) = MyOption(123.456);
    println!("{}", my_option);

}
