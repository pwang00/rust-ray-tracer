use crate::vector::*;
use crate::sphere::*;

mod vector;
mod sphere;
fn main() {
    let v1 = VecR3{x: 1.0, y: 2.0, z: 3.0};
    let v2 = VecR3{x: 1.0, y: 1.0, z: 1.0};
    let v3 = &v1 + &v2;
    println!("Dot product of vec with itself: {}", v1.dot_product(&v2));
    println!("Norm of v1 and v2: {} {}", v1.norm(), v2.norm());
    println!("Cross product of v1 and v2: {}", v1.cross_product(&v2));
    println!("Sum of v1 and v2: {}", v3);
    println!("v1, v2: {}, {}", v1, v2);
    
}

