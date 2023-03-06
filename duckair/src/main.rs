#![allow(unused_variables)]

fn main() {
    let mut original = String::from("original value");
    println!("\noriginal: \t\"{}\"\n", original);

    {
        let next = &mut original; //This is a pointer to the memory address.
        *next = String::from("next value"); //Memory address
        // Go to the memory address that stored 'next' and assign this value.

        println!("inner scope next: \t\"{}\"", next);
        println!("inner scope original: \t\"{}\"", original);
    }

    println!("Outer original value: \t\"{}\"\n", original);
}



