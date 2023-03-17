#![allow(unused_variables)]

mod geo;
use geo::calculations::distance as distance_calc;

fn main(){
   const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;


   let kcle_latitude_degrees: f64 = 41.4075;
   let kcle_longitude_degrees: f64 = -81.851111;


   let kslc_latitude_degrees: f64 = 40.7861;
   let kslc_longitude_degrees: f64 = -111.9822;

   let distance = distance_calc(
      kcle_latitude_degrees,
      kcle_longitude_degrees,
      kslc_latitude_degrees,
      kslc_longitude_degrees
   );



   // prints this one decimal value
   println!("The distance between the two points is {:.1} kilometers", distance);

}
