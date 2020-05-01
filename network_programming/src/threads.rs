use std::thread;
use std::time::Duration;

fn main() {
    for i in 1..10 {
        let handle = thread::spawn(move || {
            println!("Hello from thread number: {}", i);
            //thread::sleep(Duration::from_millis(1000));
        });
        let _ = handle.join();
    }

}
