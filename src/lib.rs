pub mod material;
pub mod object;
pub mod transform;
pub mod vector;

use serde::{Deserialize, Serialize};

use vector::Vector3;

#[derive(Clone, Copy, Debug)]
/// A ray in 3D space with direction and origin.
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    /// Create a new ray with the given `origin` in the given `direction`.
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Ray { origin, direction }
    }

    /// Get the position at the specified time `t`.
    pub fn at(self, t: f32) -> Vector3 {
        self.origin + self.direction * t
    }
}

/// A struct representing a raytracing camera.
#[derive(Deserialize, Serialize)]
pub struct Camera {
    pub focal_len: f32,
    pub width: f32,
}

impl Camera {
    /// Find the ray for the pixel at `x`, `y` if the image has resolution `x_res`, `y_res`.
    pub fn ray(&self, x: u32, y: u32, x_res: u32, y_res: u32) -> Ray {
        let aspect_ratio = x_res as f32 / y_res as f32;

        let dw = self.width / x_res as f32;
        let dh = self.width / (y_res as f32 * aspect_ratio);

        // Relative x and y positions
        let x_i = (x as f32 - x_res as f32 / 2.0) as f32 * dw;
        let y_i = (y as f32 - y_res as f32 / 2.0) as f32 * dh;

        // Ray direction
        let direction = Vector3::unit(x_i, -y_i, self.focal_len);

        Ray::new(Vector3::zeros(), direction)
    }
}

pub mod image {
    use crate::Vector3;

    use std::fs::File;
    use std::io::BufWriter;
    use std::path::Path;

    use png::{BitDepth, ColorType, Encoder};

    #[derive(Clone, Copy, Debug)]
    pub enum ImageError {
        FileCreateError,
        EncoderError,
        ImageWriteError,
    }

    impl From<ImageError> for &'static str {
        fn from(err: ImageError) -> Self {
            match err {
                ImageError::FileCreateError => "Error creating file.",
                ImageError::EncoderError => "Error encoding image.",
                ImageError::ImageWriteError => "Error writing image.",
            }
        }
    }

    pub struct Image {
        data: Vec<Vector3>,
        width: u32,
        height: u32,
    }

    impl Image {
        pub fn new(data: Vec<Vector3>, width: u32, height: u32) -> Self {
            Image {
                data,
                width,
                height,
            }
        }

        pub fn save(&self, path: impl AsRef<Path>) -> Result<(), ImageError> {
            let mut buffer = Vec::with_capacity(4 * self.data.len());
            for pixel in &self.data {
                buffer.push((pixel.x() * 255.0) as u8);
                buffer.push((pixel.y() * 255.0) as u8);
                buffer.push((pixel.z() * 255.0) as u8);
            }

            let file = File::create(path).map_err(|_| ImageError::FileCreateError)?;
            let writer = BufWriter::new(file);

            let mut encoder = Encoder::new(writer, self.width, self.height);
            encoder.set_color(ColorType::RGB);
            encoder.set_depth(BitDepth::Eight);

            let mut writer = encoder
                .write_header()
                .map_err(|_| ImageError::EncoderError)?;
            writer
                .write_image_data(&buffer)
                .map_err(|_| ImageError::ImageWriteError)?;

            Ok(())
        }
    }
}

pub mod random {
    use rand::prelude::*;
    use std::f32::consts::TAU;

    pub fn uniform() -> f32 {
        thread_rng().gen()
    }

    pub fn normal() -> f32 {
        let u: f32 = thread_rng().gen();
        let v: f32 = thread_rng().gen();

        (-2.0 * u.ln()).sqrt() * (TAU * v).cos()
    }
}

pub mod scene {
    use crate::image::Image;
    use crate::object::{Intersection, Object, Renderable};
    use crate::vector::Vector3;
    use crate::{Camera, Ray};

    use rayon::prelude::*;
    use serde::{Deserialize, Serialize};

    use std::fs::File;
    use std::io::{self, BufReader};

    /// A simple scene with a camera and some objects.
    #[derive(Deserialize, Serialize)]
    pub struct Scene {
        pub camera: Camera,
        pub objects: Vec<Object>,
    }

    impl Scene {
        /// Load a scene from a JSON file.
        pub fn from_json(path: &str) -> std::io::Result<Self> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);

            serde_json::from_reader(reader)
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "Unable to load JSON."))
        }

        /// Find the closest intersection between a ray and an object in the scene
        pub fn closest_intersection(
            &self,
            ray: Ray,
            tmin: f32,
        ) -> Option<(f32, Intersection, &Object)> {
            self.objects
                .iter()
                .map(|o| (o, o.intersection(ray)))
                .fold(None, |c, (o, i)| match (&c, i) {
                    (None, Some((t, int))) if t > tmin => Some((t, int, o)),
                    (Some((t_c, _, _)), Some((t, int))) if t < *t_c && t > tmin => {
                        Some((t, int, o))
                    }
                    _ => c,
                })
        }

        pub fn sample(&self, ray: Ray, tmin: f32, bounces: usize) -> Vector3 {
            if bounces == 0 {
                Vector3::zeros()
            } else {
                let closest = self.closest_intersection(ray, tmin);

                if let Some((_t, intersection, obj)) = closest {
                    let position = intersection.position;
                    let normal = intersection.normal;

                    obj.material
                        .lighting(-ray.direction, position, normal, self, bounces - 1)
                } else {
                    Vector3::zeros()
                }
            }
        }

        pub fn render(&self, xres: u32, yres: u32, samples: usize) -> Image {
            let pixels: Vec<_> = (0..xres * yres)
                .into_par_iter()
                .map(|i| (i % xres, i / xres))
                .map(|(x, y)| {
                    let ray = self.camera.ray(x, y, xres, yres);

                    let mut color = Vector3::zeros();
                    for _ in 0..samples {
                        color = color + self.sample(ray, 0.0, 3)
                    }

                    let srgb_gamma = |u: f32, _| {
                        if u < 0.0031308 {
                            12.92 * u
                        } else {
                            1.055 * u.powf(1.0 / 2.4) - 0.055
                        }
                    };

                    color = (1.0 / samples as f32) * color;
                    color = color.cwise(Vector3::ones(), srgb_gamma);
                    color.cwise(Vector3::ones(), f32::min)
                })
                .collect();

            Image::new(pixels, xres, yres)
        }
    }
}
