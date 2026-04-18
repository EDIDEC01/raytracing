# Ray Tracing in One Weekend (Rust)

A high-performance Ray Tracer implemented in Rust, following the "Ray Tracing in One Weekend" book series.

## Progress
Currently, this project implements the features up to **Chapter 6: Surface Normals and Multiple Objects**.

### Implemented Features
- **Vec3 Utility Class**: A robust 3D vector implementation with operator overloading for seamless mathematical operations.
- **Rays, Camera, and Background**: Basic ray generation and a gradient background.
- **Spheres**: Mathematical representation and ray-sphere intersection logic.
- **Surface Normals**: Calculation of surface normals for shading based on the direction of the hit.
- **Hittable Objects & Lists**: An abstraction layer using Traits to handle multiple objects in a scene and find the closest intersection.
- **Interval-based Clipping**: Improved ray-hit detection using a dedicated `Interval` struct for cleaner code and robust range handling.
- **Dedicated Camera Class**: Refactored the rendering loop into a standalone `Camera` class, centralizing image configuration and viewport logic.
- **PPM Image Generation**: Outputting the final render to the PPM image format.

## Mathematical Foundations

This project implements several key mathematical concepts from the book:

### 1. Ray Representation
A ray is modeled as a function of a parameter $t$:
$$\mathbf{P}(t) = \mathbf{A} + t\mathbf{B}$$
Where:
- $\mathbf{A}$ is the ray origin.
- $\mathbf{B}$ is the ray direction.
- $\mathbf{P}(t)$ is a 3D position along the ray.

### 2. Ray-Sphere Intersection
To find if a ray hits a sphere of radius $r$ centered at $\mathbf{C}$, we solve for $t$ in the quadratic equation:
$$t^2 \mathbf{d} \cdot \mathbf{d} - 2t \mathbf{d} \cdot (\mathbf{C} - \mathbf{Q}) + (\mathbf{C} - \mathbf{Q}) \cdot (\mathbf{C} - \mathbf{Q}) - r^2 = 0$$
Where $\mathbf{Q}$ is the ray origin and $\mathbf{d}$ is the ray direction. The implementation uses the optimized "half-b" version of the quadratic formula to reduce redundant calculations.

### 3. Surface Normals
The normal vector $\mathbf{n}$ at a hit point $\mathbf{P}$ on a sphere centered at $\mathbf{C}$ is calculated as:
$$\mathbf{n} = \frac{\mathbf{P} - \mathbf{C}}{r}$$
This resulting vector is then normalized to unit length. To ensure the normal always points against the incident ray, the implementation determines whether the ray hit the "front" or "back" face of the surface.

## Technical Details
- **Language**: Rust
- **Floating Point Precision**: `f64` (Double precision) for high-accuracy calculations.
- **Performance Optimizations**:
  - Buffered I/O using `BufWriter` for efficient file writing.
  - Pass-by-value for small `Copy` types (`Vec3`, `Point`, `Color`) to optimize CPU register usage.
  - Library/Binary split for better modularity and testability.

## Getting Started

### Prerequisites
- [Rust and Cargo](https://rustup.rs/) installed on your system.

### Running the Renderer
To generate the `example.ppm` image, run:

```bash
cargo run --release
```

The `--release` flag is highly recommended for ray tracing as it enables compiler optimizations that significantly speed up the rendering process.

---
*Based on [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).*
