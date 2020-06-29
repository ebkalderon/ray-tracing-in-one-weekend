# Ray Tracing in One Weekend

A straightforward Rust implementation of the books
[_Ray Tracing in One Weekend_][rtow] and [_Ray Tracing: The Next Week_][rtnw] by
Peter Shirley.

[rtow]: https://raytracing.github.io/books/RayTracingInOneWeekend.html
[rtnw]: https://raytracing.github.io/books/RayTracingTheNextWeek.html

![Picture of diffuse, metallic, and glass spheres](./preview.png)

## Features

This proof-of-concept ray tracer includes the following features:

### _Ray Tracing in One Weekend_

- [x] Lambertian diffuse shading
- [x] Hemispherical scattering diffuse shading
- [x] Metallic materials
- [x] Dielectric materials, such as water and glass
- [x] Depth-of-field blur effects

### _Ray Tracing: The Next Week_

- [x] Motion blur and moving objects
- [x] Bounding volume hierarchy (BVH) trees
- [x] Constant color and checkered texture mapping
- [ ] Perlin noise texture mapping
- [ ] Image texture mapping
- [ ] Light sources and emissive materials
- [ ] Model instancing

### Features not from any particular book

- [x] Scene abstraction
- [x] Parallel scanline rendering and multisampling with [rayon]
- [x] Parallel BVH computation with [rayon]

[rayon]: https://github.com/rayon-rs/rayon

The architecture is currently in the MVP (minimum viable product) state. Further
features and improvements are forthcoming, some chosen from the "next steps"
self-guided exercises and others from the book's seminal sequel _Ray Tracing The
Next Week_.
