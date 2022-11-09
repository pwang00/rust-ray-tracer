use crate::color::*;
use crate::hittable_list::*;
use crate::material::*;
use crate::ray::*;
use crate::sphere::*;
use crate::utilities::*;
use std::rc::Rc;

pub fn random_scene() -> HittableList {
    let mut world: HittableList = HittableList::new();
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    world.add_obj(Rc::new(Sphere {
        center: PointR3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat: ground_material,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double_01();
            let center = PointR3::new(
                a as f64 + 0.9 * random_double_01(),
                0.2,
                b as f64 + 0.9 * random_double_01(),
            );

            if (center - PointR3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    let albedo = Color::random_vec_01();
                    sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add_obj(Rc::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: sphere_material,
                    }));
                } else if choose_mat < 0.8 {
                    let albedo = Color::random_vec(0.5, 1.0);
                    let fuzz: f64 = random_double(0.0, 0.5);
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add_obj(Rc::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: sphere_material,
                    }));
                } else {
                    sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add_obj(Rc::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: sphere_material,
                    }))
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    let transparency = material1.clone();
    world.add_obj(Rc::new(Sphere {
        center: PointR3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        mat: material1,
    }));

    world.add_obj(Rc::new(Sphere {
        center: PointR3::new(0.0, 1.0, 0.0),
        radius: -0.9,
        mat: transparency,
    }));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add_obj(Rc::new(Sphere {
        center: PointR3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat: material2,
    }));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add_obj(Rc::new(Sphere {
        center: PointR3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        mat: material3,
    }));

    world
}