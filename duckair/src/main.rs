#![allow(unused_variables)]

fn main(){
    let mut flights: Vec<&str> = Vec::new(); // Generics

    // let vec_macro = vec![1,2,3,4];

    flights.push("DA113\tto Boston departs at 06:20");
    flights.push("DA98\tto London departs at 09:43");
    flights.push("DA428\tto Salt Lake City departs at 12:05");
    flights.push("DA41\tto Berlin departs at 15:30");
    flights.push("DA2815\tto Nashville departs at 17:11");

    for flight in flights.iter() {
        println!("{}", flight);
    }

    let thrid = flights[2];
    println!("\nThe third entry in vector is : {}\n", thrid);

    let fourth = flights.get(3);
    match fourth {
        Some(flight) => {
            println!("{}", flight);
        }
        _ => {}
    }

    if let Some(flight_value) = flights.get(4) {
        println!("{}", flight_value);
    }

    flights.insert(2, "DA918\tto Orlando departs at 11:12");

    for flight in flights.iter(){
        println!("{}", flight);
    }

    flights.remove(1);


}
