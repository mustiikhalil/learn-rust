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