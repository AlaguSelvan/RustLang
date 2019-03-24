pub fn run() {
	let name = "selvan";
	let mut age = 22;
	println!("My name is {} and I am {}", name, age);
	age=23;
	println!("My name is {} and I am {}", name, age);

	// Define constant
	const ID: i32 = 001;
	println!("ID: {}", ID);

	// Assign multiple vars
	let ( my_name, my_age ) = ("Brad", 37);
	println!("name is {}, {}", my_name, my_age);
}