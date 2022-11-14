# rust-ray-tracer

Self-learning Rust by implementing a ray tracer following this guide: [Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html). 

# Current progress:

The ray tracer is feature-complete with regards to the guide:

- [x] Shading via normal vector to sphere
- [x] World and hittable object abstraction
- [x] Anti-aliasing
- [x] Gamma-corrected color intensity rendering of diffuse sphere
- [x] Correct rendering of lambertian sphere
- [x] Correct rendering of regular / fuzzy metal spheres, refactoring tracing code
- [x] Correct rendering of dielectrics
- [x] Randomized scene generation

I have also implemented some additional features:

- [x] Parallelization via Rayon (between 3.5 - 4.3x speedup on default rendering parameters)
- [x] Saving rendered scenes to any image format
- [x] Argument parsing via argparse

![Final randomized scene](images/fix.png)

See [images](images/) for full list of images.