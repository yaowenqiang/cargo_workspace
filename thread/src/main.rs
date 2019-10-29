use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::Mutex;

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
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];;
        for v in vals {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        //println!("val is {}", val);
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("Are"),
            String::from("you"),
            String::from("OK"),
            String::from("Budy"),
        ];;
        for v in vals {
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        //println!("val is {}", val);
    });
    for  received in rx {
        println!("Got: {}", received);
    }

    mutex();
}

fn bad_thread_usage() {
    let v = vec![1,2,3];

    let handle = thread::spawn( move || {
        println!("Here's a vector: {:?}", v); 
    });

    //drop(v);
    handle.join().unwrap();

}

fn mutex () {
    let m = Mutex::new(5);
    {
        let mut num  = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

}
