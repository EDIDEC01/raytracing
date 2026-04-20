# Ray Tracing in One Weekend (Rust)

A high-performance Ray Tracer implemented in Rust, following the "Ray Tracing in One Weekend" book series.

## Progress
Currently, this project implements the features up to **Chapter 9: Diffuse Materials**.

### Implemented Features
- **Vec3 Utility Class**: A robust 3D vector implementation with operator overloading and stochastic sampling methods.
- **Rays, Camera, and Background**: Basic ray generation and a gradient background.
- **Spheres**: Mathematical representation and ray-sphere intersection logic.
- **Surface Normals**: Calculation of surface normals for shading and determining front/back faces.
- **Hittable Objects & Lists**: An abstraction layer using Traits to handle multiple objects in a scene.
- **Interval-based Clipping**: Improved ray-hit detection using a dedicated `Interval` struct.
- **Dedicated Camera Class**: Centralized image configuration, viewport logic, and recursive rendering.
- **Antialiasing**: Multi-sampling per pixel with randomized offsets.
- **Diffuse Materials**: Implementation of matte surfaces using stochastic ray scattering (Lambertian-like).
- **Gamma Correction**: Conversion from linear to gamma space for more accurate color representation.
- **Recursive Ray Tracing**: Limited recursion to handle multiple ray bounces for indirect lighting.
- **PPM Image Generation**: Outputting the final render to the PPM image format.

## Mathematical Foundations

This project implements several key mathematical concepts from the book:

### 1. Ray Representation
A ray is modeled as a function of a parameter $t$:

$$
\mathbf{P}(t) = \mathbf{A} + t\mathbf{B}
$$

Where:
- $\mathbf{A}$ is the ray origin.
- $\mathbf{B}$ is the ray direction.
- $\mathbf{P}(t)$ is a 3D position along the ray.

### 2. Ray-Sphere Intersection
To find if a ray hits a sphere of radius $r$ centered at $\mathbf{C}$, we solve for $t$ in the quadratic equation:

$$
t^2 \mathbf{d} \cdot \mathbf{d} - 2t \mathbf{d} \cdot (\mathbf{C} - \mathbf{Q}) + (\mathbf{C} - \mathbf{Q}) \cdot (\mathbf{C} - \mathbf{Q}) - r^2 = 0
$$

Where $\mathbf{Q}$ is the ray origin and $\mathbf{d}$ is the ray direction.

### 3. Surface Normals
The normal vector $\mathbf{n}$ at a hit point $\mathbf{P}$ on a sphere centered at $\mathbf{C}$ is calculated as:

$$
\mathbf{n} = \frac{\mathbf{P} - \mathbf{C}}{r}
$$

This resulting vector is then normalized to unit length. To ensure the normal always points against the incident ray, the implementation determines whether the ray hit the "front" or "back" face of the surface.

### 4. Antialiasing (Multi-sampling)
To reduce jagged edges, each pixel is sampled multiple times with a small random offset within the pixel's boundaries. The final color is the average of these samples:

$$
\mathbf{C}_{pixel} = \frac{1}{N} \sum_{i=1}^{N} \mathbf{C}_{sample, i}
$$

Where $N$ is the number of samples per pixel.

### 5. Diffuse Reflection
Matte surfaces are modeled by scattering rays in random directions. The current implementation uses the **Lambertian distribution** by picking random points on a unit sphere tangent to the hit point:

$$
\mathbf{S} = \mathbf{P} + \mathbf{n} + \text{random\_unit\_vector()}
$$

Where $\mathbf{S}$ is the new target point for the scattered ray, $\mathbf{P}$ is the hit point, and $\mathbf{n}$ is the surface normal.

### 6. Gamma Correction
To transform linear light into a representation more suitable for displays, we use a gamma of 2.0 (approximated by a square root):

$$
\text{color}_{gamma} = \sqrt{\text{color}_{linear}}
$$

This correction is applied to each color component (R, G, B) before conversion to the 8-bit integer format.

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
