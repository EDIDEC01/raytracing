# Ray Tracing in One Weekend (Rust)

A high-performance Ray Tracer implemented in Rust, following the "Ray Tracing in One Weekend" book series.

## Progress
Currently, this project implements the features up to **Chapter 11: Dielectrics**.

### Implemented Features
- **Vec3 Utility Class**: A robust 3D vector implementation with operator overloading and stochastic sampling methods.
- **Rays, Camera, and Background**: Basic ray generation and a gradient background.
- **Spheres**: Mathematical representation and ray-sphere intersection logic.
- **Surface Normals**: Calculation of surface normals for shading and determining front/back faces.
- **Hittable Objects & Lists**: Trait-based architecture (`Hittable` trait) for ray-object intersection detection. Objects stored in `HittableList` using `Box<dyn Hittable>` for polymorphism.
- **Material System**: Trait-based `Material` interface for surface behavior. Materials shared between objects using `Arc<dyn Material>` for efficient memory usage.
- **Lambertian (Diffuse)**: Realistic matte surfaces using a stochastic distribution.
- **Metal (Specular)**: Reflective surfaces with support for adjustable "fuzziness" (roughness).
- **Dielectrics (Refraction)**: Support for transparent materials like glass and water, including total internal reflection and Schlick's approximation for varying reflectivity by angle.
- **Interval-based Clipping**: Improved ray-hit detection using a dedicated `Interval` struct.
- **Dedicated Camera Class**: Centralized image configuration, viewport logic, and iterative rendering.
- **Antialiasing**: Multi-sampling per pixel with randomized offsets.
- **Gamma Correction**: Conversion from linear to gamma space for more accurate color representation.
- **Binary Image Output**: Switched to binary PPM (P6) format for more efficient image generation and storage.

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

### 4. Specular Reflection
For polished surfaces like metal, the reflected ray direction $\mathbf{r}$ is calculated based on the incident vector $\mathbf{v}$ and the surface normal $\mathbf{n}$:

$$
\mathbf{r} = \mathbf{v} - 2(\mathbf{v} \cdot \mathbf{n})\mathbf{n}
$$

Fuzzy reflection is achieved by adding a small random vector within a unit sphere to the perfect reflection vector, scaled by a "fuzz" factor.

### 5. Dielectric Refraction (Snell's Law)
Refraction is modeled using Snell's law:

$$
\eta \cdot \sin \theta = \eta' \cdot \sin \theta'
$$

Where $\eta$ and $\eta'$ are the refractive indices of the two media. When the ray cannot refract (total internal reflection), it reflects instead.

### 6. Schlick's Approximation
Real glass has reflectivity that varies with the angle. We use Schlick's approximation to model this:

$$
R(\theta) = R_0 + (1 - R_0)(1 - \cos \theta)^5
$$

Where $R_0 = \left( \frac{\eta - \eta'}{\eta + \eta'} \right)^2$.

### 7. Antialiasing (Multi-sampling)
To reduce jagged edges, each pixel is sampled multiple times with a small random offset within the pixel's boundaries. The final color is the average of these samples:

$$
\mathbf{C}_{pixel} = \frac{1}{N} \sum_{i=1}^{N} \mathbf{C}_{sample, i}
$$

### 8. Diffuse Reflection
Matte surfaces are modeled by scattering rays in random directions. The current implementation uses the **Lambertian distribution** by picking random points on a unit sphere tangent to the hit point:

$$
\mathbf{S} = \mathbf{P} + \mathbf{n} + \text{random\_unit\_vector()}
$$

### 9. Gamma Correction
To transform linear light into a representation more suitable for displays, we use a gamma of 2.0 (approximated by a square root):

$$
\text{color}_{gamma} = \sqrt{\text{color}_{linear}}
$$

## Technical Details
- **Language**: Rust
- **Floating Point Precision**: `f64` (Double precision) for high-accuracy calculations.
- **Memory Management**: 
  - `Arc<dyn Material>` for thread-safe shared material ownership.
  - `Box<dyn Hittable>` for polymorphic scene object storage.
- **Optimizations**:
  - Buffered I/O using `BufWriter` for efficient file writing.
  - Pass-by-value for `Copy` types (`Vec3`, `Point`, `Color`) to optimize stack allocation and CPU register usage.
  - Library/Binary split for better modularity and testability.
- **Dependencies**:
  - `rand` (v0.10.1) - Random number generation for stochastic sampling.

## Getting Started

### Prerequisites
- [Rust and Cargo](https://rustup.rs/) installed on your system (Edition 2024 or later).

### Building & Running

**Optimized release build** (slow compilation, fast rendering - recommended):
```bash
cargo build --release
cargo run --release
```

The `--release` flag is highly recommended for ray tracing as it enables compiler optimizations that significantly speed up the rendering process.

---
*Based on [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).*
