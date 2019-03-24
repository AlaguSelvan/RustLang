// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
  let mut numbers: [i32; 5] = [0, 1, 2, 3, 4];


  // Re-assign value
  numbers[2] = 20;

  println!("{:?}", numbers);
  
//   get single val
  println!("Single Value: {}", numbers[0]);

//   Get array length
    println!("Array Length: {}", numbers.len());

// Arrays are stack allocated
    println!("Array occupes {} bytes", mem::size_of_val(&numbers));
    // get slice
    let slice: & [i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}