use futures::executor::block_on;
//https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html
async fn hello_world() {
    println!("Hello world!");
}

fn main() {
    let future = hello_world();
    block_on(future);
}
/*
fn main() {
    let retval = my_fn().unwrap();
    println!("{}", retval);
}

fn my_fn() -> Result<u32, Box<error>> {
    Ok(100)
}

fn my_fut() -> impl Future< Item=u32, Error=Box<Error>> {
    ok(100)
}
*/
