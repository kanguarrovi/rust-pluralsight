
fn main(){
    let mut original = String::from("original value");
    println!("\n outer scope original: \t\"{}\"\n", original);

    {
        print_original(&original);
        change_original(&mut original);
        // This is the &mut that needs to be used when calling our change_original function.
        println!("inner scope original: \t\"{}\"\n", original)
    }
}


fn print_original(original: &String){
    print!("fn print_original:\t\"{}\"\n", original);
}

fn change_original(original: &mut String){
    let next = original;
    *next = String::from("Next value");
    print!("fn change_original: \t\"{}\"\n", next);
}