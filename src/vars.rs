// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run(){
    let name = "Brad";
    let mut age = 25; // mut is for mutable varible
    println!("My name is {} and I am {}", name, age);
    age = 26;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32=001; //i32=iniger 32 bit
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}