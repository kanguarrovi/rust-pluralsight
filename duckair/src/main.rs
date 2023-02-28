fn main() {
    let person_name_string = String::from("Jafet Soto");
    let person_name_slice = &person_name_string;
    let person_name_slice2 = person_name_string.as_str();
}
