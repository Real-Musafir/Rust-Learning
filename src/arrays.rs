// Arrays -- Fixed list where element are the same data type

pub fn run(){
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    //Re-assaing value
    numbers[2] = 20;

    println!("{:?}", numbers);

    //Get single val
    println!("Single value {}", numbers[1]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get Slice

    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice); // it show on [1,2] because of slice
}