use crate::material::Material;
use crate::transform::Transform;
use crate::vector::Vector3;
use crate::Ray;

use serde::{Deserialize, Serialize};

/// A simple struct representing an intersection between a ray and a shape.
pub struct Intersection {
    pub position: Vector3,
    pub normal: Vector3,
}

/// A trait that represents a shape that can be intersected by a ray.
pub trait Renderable: Send + Sync {
    /// Attempt to find the intersection point of the ray and the shape, returning `None` if
    /// an intersection cannot be found, and returning an intersection otherwise.
    fn intersection(&self, ray: Ray) -> Option<(f32, Intersection)>;
}

/// A struct that allows the internal shape to be transformed by an arbitrary list of
/// transformations.
#[derive(Serialize, Deserialize)]
pub struct Object {
    pub object: Shape,
    pub material: Material,
    pub transforms: Vec<Transform>,
}

impl Renderable for Object {
    fn intersection(&self, ray: Ray) -> Option<(f32, Intersection)> {
        // Transform the ray by the inverse of the Object's transforms.
        let ray_t = self
            .transforms
            .iter()
            .rev()
            .fold(ray, |r, t| t.inverse().transform_ray(r));

        // Find the ray-object intersection in the internal object's local object-space.
        let (t, local) = self.object.intersection(ray_t)?;

        // Transform the local object-space position to world space.
        let position = self
            .transforms
            .iter()
            .fold(local.position, |p, t| t.transform(p));

        // Transform the local object-space normal to world space.
        let normal = self
            .transforms
            .iter()
            .fold(local.normal, |p, t| match t {
                Transform::Translate(_) => p,
                Transform::Rotate(_, _) => t.transform(p),
                Transform::Scale(_) => t.inverse().transform(p),
            })
            .normalized();

        Some((t, Intersection { position, normal }))
    }
}

/// An enum containing unit-size shapes that have analytical line-shape intersections.
#[derive(Serialize, Deserialize)]
pub enum Shape {
    Sphere,
    Plane,
}

impl Renderable for Shape {
    fn intersection(&self, ray: Ray) -> Option<(f32, Intersection)> {
        match *self {
            Shape::Sphere => {
                let a = ray.direction.squared_norm();
                let b = 2.0 * ray.direction.dot(ray.origin);
                let c = ray.origin.squared_norm() - 1.0;

                let d = b.powi(2) - 4.0 * a * c;

                if d < 0.0 {
                    return None;
                }

                let t: f32 = (-b - d.sqrt()) / (2.0 * a);

                let position = ray.origin + (ray.direction * t);
                let normal = position.normalized();

                Some((t, Intersection { position, normal }))
            }
            Shape::Plane => {
                let n = Vector3::new(0.0, 0.0, 1.0);

                let a = -ray.origin.dot(n);
                let b = ray.direction.dot(n);
                if b.abs() < 1.0e-6 {
                    return None;
                }

                let t = a / b;
                if t < 0.0 {
                    return None;
                }

                let position = ray.origin + (ray.direction * t);
                if position.x() > 1.0 || position.y() > 1.0 {
                    return None;
                }

                let normal = if b < 0.0 { n } else { -n };

                Some((t, Intersection { position, normal }))
            }
        }
    }
}
