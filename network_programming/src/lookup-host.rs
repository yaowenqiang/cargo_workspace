#![feature(lookup_host)]
use std::env;
use std::net::lookup_host;

fn main() {
    let args:Vec<> = env::args().collect();
    if args.len() != 2  {
        eprintln!("Please provide only one host name.");
        std::process::exit(1);
    } else {
        let addresses = lookup-host(&args[1]).unwrqp(); 
        for address in addresses {
            println!("{}", address.ip());
        }
    }


}
