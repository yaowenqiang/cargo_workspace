use std::thread;
use std::time::Duration;

fn main() {
    let mut v =  vec![1,2,3];

    let handle = thread::spawn( move  || {
        for i in 1 .. 10 {
            v.push(i);
            println!("hi, number {} from the spawned thread!", i);
            println!("Here's a vector {:?}", v);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi, number {} from the main thread!", i);
        //thread::sleep(Duration::from_millis(1));
    }
        //handle.join().unwrap();
}
