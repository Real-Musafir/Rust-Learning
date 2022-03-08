pub fn run(){
    println!("hello from the print rs file");
    
    println!("Number: {}", 1);

    println!("{} is from {}", "Tipu", "Chittagong");

    // Positional Aruments
    println!("{0} , {1} and {1} are close friend ", "Afif",  "Rafique");

    // Named Arguments
    println!("{name} likes to play {activity}", name="Jhon", activity="Baseball");


    // Placeholder traits
    println!("Binary:{:b} Hex: {:x} Octal: {:o} ",10,10,10);

    // Placeholder for debug trait
    println!("{:?}", (12,true, "hello"));

    //Basic math
    println!("10+10 = {}",10+10);
}