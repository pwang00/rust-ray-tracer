use ray_tracer::{self, ElementWise, Norm};
fn main() {
    let v1 : Vec<f64> = Vec::from([3.0, 4.0]);
    let v2 : Vec<f64> = Vec::from([4.0, 5.0]);
    println!("Dot product of vec with itself: {}", v1.dot_product(&v2));
    println!("Norm of v1 and v2: {} {}", v1.norm(), v2.norm());
}
