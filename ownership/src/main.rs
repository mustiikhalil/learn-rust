// data with fixed size will be allocated into the stack
// data that might have different sizes / or doesnt have a known size at compile time in the future will be stored in the heap

// ownership rules
// 1- each value in rust has it's own owner
// 2- there can be only one owner at a time
// 3- when the owner goes out of scope the value will be dropped
fn main() {
    println!("Hello, world!");

    let s = String::from("Hello");
    let s2 = s; // this called a move and it invalidates the old value s
                // see figure 4.4 in the rust book
    let s1 = s2.clone(); // clones the data on the heap and creates a new pointer to it
    println!("{}", s2);
    println!("{}", s1);

    let x = 5.2;
    let y = x;
    println!("x = {} y = {}", x, y);
    // this works since the known size values are stored in the stack directly
    // this would allow us to just copy the value instead of referencing it

    // Copy values are any Scalars and maybe tuples if they are all scalars (i32, i32) and not (i32, String)

    let s = String::from("Hello, world!!");
    let (s3, sSize) = transfer_ownership(s);
    println!("size: {}", sSize);
    takes_ownership(s3); // this function takes the ownership of the string

    let ss = String::from("Hello world again!!");

    passes_refrence(&ss);
    println!("still alive since pass by ref{}", ss);

    let i = 42;
    makes_copy(i);
    println!("{}", i);
}
// transfering ownership would work like this since we are returning a string into a new value
fn transfer_ownership(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn passes_refrence(s: &String) {
    println!("value: {}", s);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(value: i32) {
    println!("int: {}", value);
}
// at the end of the scope all the values will get dropped
// even s, it will be dropped when rust calls `drop` which is similar to Resource Acquisition Is Initialization (RAII)
