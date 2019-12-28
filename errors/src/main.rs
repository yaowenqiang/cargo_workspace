use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;
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

    print!("{:?}\n", f);

    let f2 = File::open("world.txt").unwrap_or_else(|error | {
        if error.kind() == ErrorKind::NotFound {
            File::create("world.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file {:?}", error) ;       
            })
        } else {
            panic!("Problem opening the file {:?}", error);
        }
    });
    print!("{:?}\n", f2);


    let f3 = File::open("hello.txt").unwrap();
    print!("{:?}\n", f3);


    let f4 = File::open("hello.txt").expect("Failed to open hello.txt");
    print!("{:?}\n", f4);
    let username = read_username_from_file().expect("get username failed");
    print!("{:?}", username);





}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello3.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }


}


