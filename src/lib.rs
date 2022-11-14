pub mod benchmark;
pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod ray;
pub mod render;
pub mod render_params;
pub mod scene;
pub mod sphere;
pub mod utilities;
pub mod vector;

#[cfg(test)]
mod vector_tests {
    use crate::vector::VecR3;

    #[test]
    fn vector_add() {
        let v1: VecR3 = VecR3::new(1.0, 2.0, 3.0);
        let v2: VecR3 = VecR3::new(1.0, 1.0, 1.0);
        let v3: VecR3 = v1 + v2;
        assert_eq!(v3, VecR3::new(2.0, 3.0, 4.0))
    }

    #[test]
    fn vector_dot() {
        let v1: VecR3 = VecR3::new(1.0, 2.0, 3.0);
        let v2: VecR3 = VecR3::new(1.0, 1.0, 1.0);
        let v3: VecR3 = VecR3::zero();
        let prod1: f64 = v1.dot_product(v2);
        let prod2: f64 = v1.dot_product(v3);
        assert_eq!(prod1, 6.0);
        assert_eq!(prod2, 0.0)
    }

    #[test]
    fn vector_cross() {
        let v1: VecR3 = VecR3::new(1.0, 2.0, 3.0);
        let v2: VecR3 = VecR3::new(1.0, 1.0, 1.0);
        let v3: VecR3 = VecR3::zero();
        let cross1: VecR3 = v1.cross_product(v2);
        let cross2: VecR3 = v1.cross_product(v3);
        assert_eq!(
            cross1,
            VecR3::new(-1.0, 2.0, -1.0)
        );
        assert_eq!(cross2, v3);
    }
    #[test]
    fn vector_norm() {
        let v1: VecR3 = VecR3::new(1.0, 2.0, 3.0);
        let v2: VecR3 = VecR3::new(1.0, 1.0, 1.0);
        assert_eq!(v1.norm(), 14.0f64.sqrt());
        assert_eq!(v2.norm(), 3.0f64.sqrt())
    }
    #[test]
    fn vector_normalize() {
        let v1: VecR3 = VecR3::new(3.0, 4.0, 0.0);
        assert_eq!(
            v1.normalize(),
            VecR3::new(3.0/5.0, 4.0/5.0, 0.0)
        )
    }
    #[test]
    fn vector_is_near_zero() {
        let v1: VecR3 = VecR3::new(1E-9, 1E-9, 1E-9);
        assert_eq!(v1.is_near_zero(), true)
    }

    #[test]
    fn vector_reflect() {
        let v1: VecR3 = VecR3::new(1.0, 1.0, 1.0);
        let v2: VecR3 = VecR3::new(1.0, -1.0, 1.0);
        assert_eq!(VecR3::reflect(v1, v2), VecR3::new(-1.0, 3.0, -1.0))
    }
}
