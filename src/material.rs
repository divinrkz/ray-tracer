use crate::scene::Scene;
use crate::vector::Vector3;
use crate::{random, Ray};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
/// An enum with a variety of different materials for rendering. Available materials are:
/// - Emissive: A light source. Emits light of the given `color` with the given `intensity`.
/// - Diffuse: A Lambertian diffuse material with the given `color`.
/// - Specular: A glossy material with the given `color` and roughness.
pub enum Material {
    Emissive { color: Vector3, intensity: f32 },
    Diffuse { color: Vector3 },
    Specular { color: Vector3, roughness: f32 },
}

impl Material {
    /// Compute the surface color for this material at the given `position` given the `normal`
    /// and the given `scene` with a given `view` direction.
    pub fn lighting(
        &self,
        view: Vector3,
        position: Vector3,
        normal: Vector3,
        scene: &Scene,
        bounces: usize,
    ) -> Vector3 {
        let dir = loop {
            let dir = Vector3::unit(random::normal(), random::normal(), random::normal());

            if dir.dot(normal) > 0.0 {
                break dir;
            }
        };

        let incoming = scene.sample(
            Ray { origin: position, direction: dir }, 1.0e-3, bounces
        );

        match *self {
            Material::Emissive { color, intensity } => {
                color * intensity
            }
            Material::Diffuse { color } => {
                let brdf = (dir.dot(normal) / std::f32::consts::PI) * color;
                2.0 * std::f32::consts::PI * brdf.cwise_mul(incoming)
            }
            Material::Specular { color, roughness } => {
                let halfway = (view + dir).normalized();
                let d = (normal.dot(halfway).powi(2) / (roughness * roughness - 2.0 * roughness + std::f32::consts::PI)).clamp(0.0, 1.0);
                let g = 1.0 / (4.0 * (view.dot(halfway)).powi(2)).clamp(0.0, 1.0);
                let brdf = (d * g * color).cwise_mul(incoming);

                2.0 * std::f32::consts::PI * brdf
            }
        }
    }
}
