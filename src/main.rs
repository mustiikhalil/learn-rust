// data with fixed size will be allocated into the stack
// data that might have different sizes / or doesnt have a known size at compile time in the future will be stored in the heap

// ownership rules
// 1- each value in rust has it's own owner
// 2- there can be only one owner at a time
// 3- when the owner goes out of scope the value will be dropped
fn main() {
    println!("Hello, world!");
}
