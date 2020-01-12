#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

// tuple struct 
struct Color(u32, u32, u32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

impl Rectangle {
    // Associated Functions since they dont use (self)
    // these are still functions and not methods
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    println!("Hello, world!");

    let user = User { // let mut user to enable mutating values in teh struct
        username: String::from("mustiikhalil"),
        email: String::from("example@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    let second_user = build_user(String::from("example2@gmail.com"), String::from("secondUsername"));
    // user.username = String::from("mustii2"); we can only mutate values in a struct
    // if they were tagged as mutable

    // println!("{} is a user", user);

    let third_user = User {
        username: String::from("test"),
        email: String::from("test@test.com"),
        // sepecifies that rest of the fields should be the save as the given instance since
        // we didn't explicitly change them
        ..second_user
    };

    println!("user: {:#?}", third_user);
    let black = Color(0, 0, 0);

    let rect = Rectangle { width: 30, height: 50};
    let square = Rectangle::square(5);

    println!("square is {:#?}", square);
    println!("area is {}", rect.area());
    println!("can hold {}", rect.can_hold(&Rectangle {width: 10, height: 40} ))
}

fn build_user(email: String, username: String) -> User {
    User {
        // email: email,
        // username: username,
        // can be replaced by just writing the following
        email,
        username,
        active: true,
        sign_in_count: 1, 
    }
}