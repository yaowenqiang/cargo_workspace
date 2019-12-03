#![crate_type = "lib"]
#![crate_name = "rary"]

pub fn public_function() {
    println!("called rary's public_function()");
}


fn private_function() {
    println!("called rary's private_function()");
}


pub fn indicate_access() {
    println!("called rary's indicate_access , that \n ");
    private_function();
}

//rustc lib.rs
