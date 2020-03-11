extern crate redis;

fn main() {
    do_something();
    println!("Hello, world!");
}

fn do_something() ->redis::RedisResult<()> {
    let client = redis::Client::open("redis:://127.0.0.1:6379/")?;
    let mut con = client.get_connection()?;
    con.set("my_key", 123);
    Ok(())
}
