extern crate redis;
use crate::redis::Commands;
fn main() {
    do_something();
}

fn do_something() ->redis::RedisResult<()> {
    let client = match (redis::Client::open("redis://127.0.0.1/")) {
        Ok(c) => c,
        Err(e) => panic!("connection error! {}", e)
    };
    println!("helo redis");
    let mut con = client.get_connection()?;
    let count : i32 = con.get("a")?;
    println!("{}", count);
    let name : String  = con.get("name")?;
    println!("{}", name);
    let () = con.set("key1", "value1")?;
    Ok(())
}
