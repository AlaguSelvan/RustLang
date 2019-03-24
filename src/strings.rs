// Primitive str = Immutable fixed-length string somewhere
// String = Growable, heap-allocated data structure - Use when you need to modify or
// own string data

pub fn run() {
  let mut hello = String::from("Hello");
  
//   Get length
  println!("Length: {}", hello.len());

//   push char
  hello.push('W');

    // push string ""
  hello.push_str("orld!  ");

//   Capacity in bytes
    println!("Capacity: {}", hello.capacity());


    println!("Is Empty: {}", hello.is_empty());

    println!("contains 'World': {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    for word in hello.split_whitespace() {
        println!("whitespace:{}", word)
    }
  
//   Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    // Assertion testing
    assert_eq!(12, s.capacity());
  println!("s: {}", s);
}