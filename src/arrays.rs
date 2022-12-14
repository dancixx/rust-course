// Arrays - Fixed list where elements are the same data types
use std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Get single val
    println!("Single Value: {}", numbers[0]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
