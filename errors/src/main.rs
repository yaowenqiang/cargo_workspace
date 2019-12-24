use std::fs::File;
fn main() {
    //panic!("Crash and burn");

    let v = vec![1,2,3];
    //v[90];

    let f  = File::open("foo");
    let f = match f {
        Ok(file) =>  file,
        Err(error) => {
            panic!("failed to open the file {:?}", error)
        } 
    };

}
