# Rust Ray Tracer.

Ray tracing was first deployed in applications where taking a relatively long time to render could be tolerated, such as still CGI images, and film and television visual effects (VFX), but was less suited to real-time applications such as video games, where speed is critical in rendering each frame. [(Wikipedia)](https://en.wikipedia.org/wiki/Ray_tracing_(graphics))


### Rendering Workflow
``Ray Generation``: For each pixel, calculate the ray direction from the camera through the viewport. <br>
``Intersection Tests``: For each ray, test for intersections with scene objects (e.g., spheres, planes).<br>
``Shading``: Calculate the color of the intersected point based on material properties, light sources, and shadow occlusion.<br>
``Reflection & Refraction``: If the material is reflective or transparent, trace additional rays to simulate reflections or refractions.<br>
``Image Output``: Save the computed color values for each pixel to an image file.

### Future Improvements
``Multiple Object Types``: Extend support for additional shapes like cubes, cylinders, etc. <br>
``Advanced Lighting Models``: Add support for more complex lighting models, including area lights and global illumination. <br>
``Anti-Aliasing``: Implement anti-aliasing techniques to smooth edges. <br>
``Acceleration structures``: Improve performance with spatial partitioning structures like BVH. <br>

## Getting Started
### Prerequisites
Rust (version >= 1.56.0)

### Running the Ray Tracer
Clone this repository:
```bash
git clone https://github.com/divinrkz/ray-tracer.git
cd ray-tracer
```
Compile and run the application:
```
cargo run
```
The program will output rendered images to the /output directory.


## Contributing
Contributions are welcome! Please submit a pull request with your changes, and ensure your code follows Rust's standard coding practices.


## Acknowledgments
This project was part of a project in a Rust class at Caltech by Professor Michael Vanier.
