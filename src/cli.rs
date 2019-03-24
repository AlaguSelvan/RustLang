use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name: &str = "Brad";
    let status: &str = "100%";
    // println!("Args: {:?}", command)
    if command == "hello" {
        println!("Hi {}, How are you", name);
    } else if command == "status" {
        println!("status is {}", status);
    } else {
        println!("That is not a valid command");
    }
}
