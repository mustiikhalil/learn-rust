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
    let (s3, s_size) = transfer_ownership(s);
    println!("size: {}", s_size);
    takes_ownership(s3); // this function takes the ownership of the string

    let ss = String::from("Hello world again!!");

    let size = caluclate_size(&ss);
    println!("still alive since pass by ref {} with size {}", ss, size);

    let i = 42;
    makes_copy(i);
    println!("{}", i);
    let mut new_s = ss.clone();
    add_meow(&mut new_s);

    println!("is here: {}", new_s);
    // can't point into more than one mutating pointer
    // this is fine, since the value is being mutated and referenced only once
    let _xyz = &mut new_s;

    // we can't use a pointer that will change the value of ss since
    // we are using a and b as a none mutating values after declaring c
    // if c is not commented out the code will not run
    let a = &ss;
    let b = &ss;
    // let c = &mut ss; 
    println!("{} {}", a, b); // c
    // however we can do the following
    let a = &new_s;
    let b = &new_s;
    println!("{} {}", a, b);
    // since a and b are not in use anymore we can use c mut to point into new_s
    let c = &mut new_s; 
    println!("{}", c);

    // also check dangling values
    // a dangling value is a value that will by returned by reference however the actual value is going out of scope
    // to avoid dangling values, just return the actual value

    let mut some_word = String::from("Welcome to heartbreak!");
    let word = first_word(&some_word);
    println!("first word: {}", word);
    some_word.clear();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &itr) in bytes.iter().enumerate() {
        if itr == b' ' {
            return &s[0..index];
        }
    }
    &s[..]
}


// transfering ownership would work like this since we are returning a string into a new value
fn transfer_ownership(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn add_meow(s: &mut String) {
    s.push_str("... meowww!");
}

// passing a value to a function by a reference is called borrowing

fn caluclate_size(s: &String) -> usize {
    s.len()
    // we cant edit a borrowed val without the &mut String
    // s.push_str(", meow!");
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(value: i32) {
    println!("int: {}", value);
}
// at the end of the scope all the values will get dropped
// even s, it will be dropped when rust calls `drop` which is similar to Resource Acquisition Is Initialization (RAII)
