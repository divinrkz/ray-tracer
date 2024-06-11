use serde::{Deserialize, Serialize};

use crate::vector::Vector3;
use crate::Ray;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
/// An enum representing various transformations that can be applied to a 3D vector.
pub enum Transform {
    Scale(Vector3),
    Translate(Vector3),
    Rotate(Vector3, f32),
}

impl Transform {
    /// Return the inverse of this transform.
    pub fn inverse(self) -> Transform {
        // TODO: Implement Transform.
    }

    /// Return the input vector transformed by this transformation.
    pub fn transform(self, vector: Vector3) -> Vector3 {
        // TODO: Implement Transform.
    }

    /// Return a copy of an input ray transformed by self.
    pub fn transform_ray(self, ray: Ray) -> Ray {
        // TODO: Implement Transform.
    }
}
