fn main() {
    let x = "    ";
    // shadowing
    let x = x.len();
    println!("{}", x);

    let mut meaning_of_life: i32 = "42".trim().parse().expect("some error!");
    mutateptr(&mut meaning_of_life);
    println!("meaning of life: {}", ptr(&meaning_of_life));

    let second_value = {
        let value = 3;
        value + 1
    };

    println!("second: {}", second_value);

    let val = if meaning_of_life == 42 {
        "Bunnies are the best!"
    } else {
        "Bunnies are still the best!"
    };
    if meaning_of_life < 10 {
        println!("less than")
    } else if meaning_of_life > 70 {
        println!("more than")
    } else {
        println!("else")
    }
    println!("val: {}", val);

    let mut pos = 0;
    let result = loop {
        if pos / 2 == 5 {
            break pos + 7;
        }
        pos += 1;
    };
    println!("result: {}", result);

    let a = [10, 20, 30];
    for elem in a.iter() {
        println!("element: {}", elem);
    }

    for i in (1..4).rev() {
        println!("{}!", i)
    }
    println!("LIFTOFF!!!");
}

fn mutateptr(x: *mut i32) {
    unsafe {
        *x = *x * 2;
    }
}

fn ptr(x: *const i32) -> i32 {
    unsafe {
        let m = *x;
        return m;
    }
}