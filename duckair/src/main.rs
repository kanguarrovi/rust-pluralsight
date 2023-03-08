
fn main(){
    panic_vector();
}

fn panic_vector(){
    let vector = vec![1,2,3,4,5];
    println!("{}", vector[10]);
}