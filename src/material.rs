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
        // TODO: Implement materials.
    }
}
