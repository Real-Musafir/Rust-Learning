//  Vectors - Resizable arrays

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4];

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    //Re-assaing value
    numbers[2] = 20;

    println!("{:?}", numbers);

    //Get single val
    println!("Single value {}", numbers[1]);

    // Get Vector length
    println!("Vector Length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get Slice

    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice); // it show on [1,2] because of slice

    // Loop through vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    // Loop & mutate values

    for x in numbers.iter_mut(){
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers); //Now is value multiply by 2
}