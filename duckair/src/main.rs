fn main() {
    // let location: [f32;2] = [0.0, 0.0];
    // let location: [f32;2] = [41.4094069, -81.8546911];
    let location = ("KCLE", 41.4094069, -81.8546911);
    // Destructuring the array of the tuple.
    let (name, latitude, longitude) = location
    println!("Location name: {}, latitude: {}, longitude: {}", location.0, location.1, location.2);
}
