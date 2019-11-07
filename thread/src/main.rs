use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};
//use std::rc::Rc;

fn main() {
    let mut v =  vec![1,2,3];

    let handle = thread::spawn( move  || {
        for i in 1 .. 10 {
            v.push(i);
            println!("hi, number {} from the spawned thread!", i);
            println!("Here's a vector {:?}", v);
            thread::sleep(Duration::from_millis(1));
            println!("current thread id is : {:#?}", thread::current().id());
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
    mutexes();
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

fn mutexes () {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];
    for _ in 0 .. 10 {

        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move || {
            let mut  num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(handler);
    }

    /*
    let handle1 = thread::spawn(move || {
        let mut  num1 = counter.lock().unwrap();
        *num1 += 1;
    });
    handlers.push(handle1);

    let handle2 = thread::spawn(move || {
        let mut  num2 = counter.lock().unwrap();
        *num2 += 1;
    });
    handlers.push(handle2);
    */

    for handle in handlers {
        handle.join().unwrap();
    }

    println!("Result: {:?}", *counter.lock().unwrap());

}
