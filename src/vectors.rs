// Arrays - Fixed list where elements are the same data types
use std::mem;

fn test(vector: Vec<i32>) {
    println!("{:?}", vector);
}

pub fn run() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // test(numbers);

    // let c = move || println!("{numbers:?}"); // move numbers into closure
    // c();

    // borrow numbers
    let c = |numbers| println!("{:?}", numbers);
    c(numbers);

    // test(numbers);

    let mut numbers = vec![1, 2, 3, 4, 5];
    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get single val
    println!("Single Value: {}", numbers[0]);

    // Get vector length
    println!("Vector Length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}
