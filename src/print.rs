pub fn run() {
	// Print to console
	println!("Hello From the print.rs");

	// Basic Formatting
	println!("{} is from {}", "Brad", "Mass");

	// Positional Arguments
	println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

	// Named Arguments
	println!("{name} likes to play {activity}", name="john", activity="baseball");
	
	// Placeholder traits
	println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
	
	// Placeholder debug traits
	println!("{:?}", (12, true, "hello"));

	// Basic math
	println!("10 + 10= {}", 10 + 10);
}