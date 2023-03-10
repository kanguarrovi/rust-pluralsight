#![allow(unused_variables)]

struct Waypoint { //Capitalize every first letter of each word.
    name: String,
    latitude: f64,
    longitude: f64
}

struct Segment {
    start: Waypoint,
    end: Waypoint
}

fn main(){
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    let mut kcle = Waypoint{
        name: "KCLE".to_string(),
        latitude: 41.4075,
        longitude: -81.851111
    };

    let ksle = Waypoint {
        name: "KSLC".to_string(), // Overwrite
      ..kcle // The name of the instance you want to reuse.
    };

    //kcle.latitude

}
