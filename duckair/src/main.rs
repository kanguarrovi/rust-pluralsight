//#![allow(unused_variables)]

enum NavigationAids {
    NDB = 3,
    VOR = 7,
    VORDME, // This value will be 8.
    FIX {name: String, latitude: f32, longitude: f32}
}


fn main() {
    // u8 is casting into integer type.
    println!("NDB:\t{}", NavigationAids::NDB as u8);
    println!("VOR:\t{}", NavigationAids::VOR as u8);
    println!("VORDME:\t{}", NavigationAids::VORDME as u8);
}


