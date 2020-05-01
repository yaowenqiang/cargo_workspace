use std::fmt;
use std::error::Error;

#[derive(Debug)]
enum OperationsError {
    DividedByZeroError,
}

impl fmt::Display for OperationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OperationsError::DividedByZeroError => f.write_str("Cannot divide by zero"),
        }
    }
}

impl Error for OperationsError {
    fn description(&self) -> &str{
        match *self {
            OperationsError::DividedByZeroError =>  "Cannot divide by zero",
        }
    }
}

//fn divide(dividend: u32, divisor: u32) -> Result<u32, OperationsError> {
fn divide(dividend: u32, divisor: u32) -> Option<u32> {
    if divisor == 0u32 {
        //Err(OperationsError::DividedByZeroError)
        None
    } else {
        //Ok(dividend / divisor)
        Some(dividend / divisor)
    }
}


fn main () {
    let result1 = divide(100, 0);
    match result1 {
        None => println!("Error occurred"),
        Some(result) => println!("The result is {}", result),
    }

    println!("{:?}", result1);

    let result2 = divide(100, 2);

    println!("{:?}", result2.unwrap());

}
