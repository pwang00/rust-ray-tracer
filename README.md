# rust-ray-tracer

Self-learning Rust by implementing a ray tracer following this guide: [Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html). 

# Current progress:

- [x] Shading via normal vector to sphere
- [x] World and hittable object abstraction
- [x] Anti-aliasing
- [x] Gamma-corrected color intensity rendering of diffuse sphere
- [x] Correct rendering of lambertian sphere
- [x] Correct rendering of regular / fuzzy metal spheres, refactoring tracing code
- [x] Correct rendering of dielectrics
- [x] Randomized scene generation
- [x] Parallelization via Rayon (approx 4.2x speedup on default rendering parameters)

![Final randomized scene](images/fix.png)

See [images](images/) for full list of images.