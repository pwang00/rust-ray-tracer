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
- [x] Parallelization via Rayon (approx 4.1x speedup on default rendering parameters)

I do some pretty stupid stuff at the moment with copying to temp vectors in the parallel renderer since I'm not really familiar with Rayon, but hopefully I can change the current implementation to be more idiomatic and efficient.

![Final randomized scene](images/final.png)

See [images](images/) for full list of images.