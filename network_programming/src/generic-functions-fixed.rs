use std::ops::Add;


struct Tuple<T> {
    first: T,
    second: T,
}

fn main() {
    let tuple_u32: Tuple<u32> = Tuple{first: 4u32, second: 2u32};
    let tuple_u64: Tuple<u64> = Tuple{first: 5u64, second: 6u64};

    println!("{}", sum(tuple_u32));
    println!("{}", sum(tuple_u64));

    let tuple: Tuple<String> = Tuple{first: "one".to_owned(), second: "Tow".to_owned()};
    println!("{}", sum(tuple));
}

fn sum<T: Add<Output = T>>(tuple : Tuple<T>) -> T{

    tuple.first + tuple.second
}
