#![allow(unused_variables)]

fn main() {
    let greater = return_greater(10, 5);
    println!("{}", greater);
}

fn return_greater(first: u8, second: u8) -> u8{
    /*
    if first > second {
        first
    } else {
        second
    }*/
    let result = if first > second { first } else { second };
    result // Just the variable to result without any semicolon.
}

