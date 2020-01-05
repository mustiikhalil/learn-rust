// data with fixed size will be allocated into the stack
// data that might have different sizes / or doesnt have a known size at compile time in the future will be stored in the heap

// ownership rules
// 1- each value in rust has it's own owner
// 2- there can be only one owner at a time
// 3- when the owner goes out of scope the value will be dropped
fn main() {
    println!("Hello, world!");

    let s = String::from("Hello");
    let s2 = s; // this called a move and it invalidates s
    let s1 = s2.clone(); // clones the data on the heap and creates a new pointer to it
    println!("{}", s2);
    println!("{}", s1);
}
// at the end of the scope all the values will get dropped
// even s, it will be dropped when rust calls `drop` which is similar to Resource Acquisition Is Initialization (RAII)
