use std::iter::Scan;

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
        match self {
            Transform::Scale(scale) => Transform::Scale(Vector3::new(
                1.0 / scale.x(),
                1.0 / scale.x(),
                1.0 / scale.x(),
            )),
            Transform::Rotate(axis, angle) => Transform::Rotate(axis, -angle),
            Transform::Translate(delta) => Transform::Translate(-delta),
        }
    }

    /// Return the input vector transformed by this transformation.
    pub fn transform(self, vector: Vector3) -> Vector3 {
        // TODO: Implement Transform.
        match self {
            Transform::Scale(scale) => Vector3::new(
                vector.x() * scale.x(),
                vector.y() * scale.y(),
                vector.z() * scale.z(),
            ),
            Transform::Rotate(axis, angle) => {
                let cos_theta = angle.cos();
                let sin_theta = angle.sin();
                let dot = vector.dot(axis);
                let cross = axis.cross(vector);

                (vector * cos_theta) + (cross * sin_theta) + axis * dot * (1.0 - cos_theta)
            }
            Transform::Translate(delta) => vector + delta,
        }
    }

    /// Return a copy of an input ray transformed by self.
    pub fn transform_ray(self, ray: Ray) -> Ray {
        // TODO: Implement Transform.
        match self {
            Transform::Scale(_) | Transform::Rotate(_, _) => Ray {
                origin: self.transform(ray.origin),
                direction: self.transform(ray.direction),
            },
            Transform::Translate(_) => Ray {
                origin: self.transform(ray.origin),
                direction: ray.direction,
            },
        }
    }
}
