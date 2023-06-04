// cow1.rs

// This exercise explores the Cow, or Clone-On-Write type.
// Cow is a clone-on-write smart pointer.
// It can enclose and provide immutable access to borrowed data, and clone the data lazily when mutation or ownership is required.
// The type is designed to work with general borrowed data via the Borrow trait.
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the TODO markers.

// cow1.rs

use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // 如果还没有持有，克隆到一个 vector。
            input.to_mut()[i] = -v;
        }
    }
    input
}

fn main() {
    // 没有 clone 发生，因为 `input` 不需要被改变。
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]);
    match abs_all(&mut input) {
        Cow::Borrowed(_) => println!("I borrowed the slice!"),
        _ => panic!("expected borrowed value"),
    }

    // 发生了 clone， 因为 `input` 需要被改变。
    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    match abs_all(&mut input) {
        Cow::Owned(_) => println!("I modified the slice and now own it!"),
        _ => panic!("expected owned value"),
    }

    // 没有 clone 发生，因为已经持有了 `input`。
    let slice = vec![-1, 0, 1];
    let mut input = Cow::from(slice);
    match abs_all(&mut input) {
        // TODO
        Cow::Borrowed(_) => println!("I own this slice!"),
        _ => panic!("expected borrowed value"),
    }
}
