// Primitive Types ---
// Integers: u8, i8, u16, i16, u32, i32, u64, u128, i128 (number of
// bits they take in memeory)
// Floats: f32, f64
// Boolean(bool)
// Characters(char)
// Tuples
// Arrays

pub fn run() {
    // By default it is "132"
    let x = 1;

    // By default it is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 23423424;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    // let is_active = true;
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 < 5;

    let a1 = 'a';
    let face = '\u{1F600}';

      println!("{:?}", (x,y,z, is_active, is_greater, a1, face));

}