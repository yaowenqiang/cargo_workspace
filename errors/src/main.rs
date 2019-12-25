use std::fs::File;
use std::io::ErrorKind;
fn main() {
    //panic!("Crash and burn");

    let v = vec![1,2,3];
    //v[90];

    let f  = File::open("foo");
    /*
    let f = match f {
        Ok(file) =>  file,
        Err(error) => {
            panic!("failed to open the file {:?}", error)
        } 
    };
    */

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other_error => panic!("Problem open the file {:?}", other_error)
        }
    };

    print!("{:?}", f);

}
