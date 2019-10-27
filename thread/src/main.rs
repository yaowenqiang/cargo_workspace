use std::thread;
use std::time::Duration;
use std::sync::mpsc;

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
    let (tx, rx) = mpsc::channel();
    // tx.send(()).unwrap();
    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn bad_thread_usage() {
    let v = vec![1,2,3];

    let handle = thread::spawn( move || {
        println!("Here's a vector: {:?}", v); 
    });

    //drop(v);
    handle.join().unwrap();

}
