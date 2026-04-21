# Ray Tracing in One Weekend (Rust)

A high-performance Ray Tracer implemented in Rust, following the "Ray Tracing in One Weekend" book series.

## Progress
Currently, this project implements the features up to **Chapter 13: Defocus Blur**.

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
- **Positionable Camera**: Advanced camera controls including field-of-view (vfov), look-from/look-at orientation, and customizable "up" vector.
- **Defocus Blur (Depth of Field)**: Simulated aperture and focus distance for realistic blur effects.
- **Interval-based Clipping**: Improved ray-hit detection using a dedicated `Interval` struct.
- **Dedicated Camera Class**: Centralized image configuration with an `init()` system and iterative rendering.
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

### 2. Ray-Sphere Intersection
Solving for $t$ in the quadratic equation to find hits on a sphere of radius $r$ centered at $\mathbf{C}$.

### 3. Positionable Camera (Orthonormal Basis)
To orient the camera, we define an orthonormal basis $(u, v, w)$ using the `look_from`, `look_at`, and `vup` vectors:
- $w = \text{unit}(\text{look\_from} - \text{look\_at})$
- $u = \text{unit}(\text{vup} \times w)$
- $v = w \times u$

### 4. Defocus Blur
Rays are sampled from a disk of radius $r_{aperture}$ centered at the camera's `look_from` position:

$$
r_{aperture} = \text{focus\_dist} \times \tan(\text{defocus\_angle} / 2)
$$

The rays are then directed towards the focus plane at `focus_dist`.

### 5. Field of View (Vertical FOV)
The viewport height is determined by the vertical field of view $\theta$ and the focus distance $d$:

$$
h = \tan(\theta/2) \cdot d
$$

### 6. Specular Reflection & Refraction
- **Reflection**: $\mathbf{r} = \mathbf{v} - 2(\mathbf{v} \cdot \mathbf{n})\mathbf{n}$
- **Refraction (Snell's Law)**: $\eta \sin \theta = \eta' \sin \theta'$
- **Schlick's Approximation**: For angle-dependent reflectivity.

### 7. Antialiasing & Diffuse Reflection
- **Antialiasing**: $\mathbf{C}_{pixel} = \frac{1}{N} \sum \mathbf{C}_{sample}$
- **Lambertian**: $\mathbf{S} = \mathbf{P} + \mathbf{n} + \text{random\_unit\_vector()}$

## Technical Details
- **Language**: Rust
- **Camera Implementation**: 
  - Uses `Default` trait for configuration.
  - Decoupled configuration from initialization via an internal `init()` method called at the start of `render()`.
- **Memory Management**: 
  - `Arc<dyn Material>` for thread-safe shared material ownership.
  - `Box<dyn Hittable>` for polymorphic scene object storage.
- **Optimizations**:
  - Buffered I/O using `BufWriter`.
  - Pass-by-value for `Copy` types (`Vec3`, `Point`, `Color`).
- **Dependencies**:
  - `rand` (v0.10.1) - Random number generation.

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
