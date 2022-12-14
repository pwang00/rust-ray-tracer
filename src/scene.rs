use crate::hittable_list::*;
use crate::material::*;
use crate::ray::*;
use crate::sphere::*;
use crate::utilities::*;
use crate::vector::*;
use std::sync::Arc;

pub fn random_scene() -> HittableList {
    let mut world: HittableList = HittableList::new();
    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    world.add_obj(Box::new(Sphere {
        center: PointR3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat: ground_material,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = Utils::random_double_01();
            let center = PointR3::new(
                a as f64 + 0.9 * Utils::random_double_01(),
                0.2,
                b as f64 + 0.9 * Utils::random_double_01(),
            );

            if (center - PointR3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                let sphere_material: Arc<dyn Material + Send + Sync>;

                if choose_mat < 0.8 {
                    let albedo = Color::random_vec_01();
                    sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add_obj(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: sphere_material,
                    }));
                } else if choose_mat < 0.8 {
                    let albedo = Color::random_vec(0.5, 1.0);
                    let fuzz: f64 = Utils::random_double(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add_obj(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: sphere_material,
                    }));
                } else {
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add_obj(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: sphere_material,
                    }))
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    let transparency = material1.clone();
    world.add_obj(Box::new(Sphere {
        center: PointR3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        mat: material1,
    }));

    world.add_obj(Box::new(Sphere {
        center: PointR3::new(0.0, 1.0, 0.0),
        radius: -0.9,
        mat: transparency,
    }));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add_obj(Box::new(Sphere {
        center: PointR3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat: material2,
    }));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add_obj(Box::new(Sphere {
        center: PointR3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        mat: material3,
    }));

    world
}
