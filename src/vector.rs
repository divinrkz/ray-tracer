use std::fmt;
use std::intrinsics::sqrtf32;
use std::ops::{Add, Mul, Neg, Sub};

use serde::{Deserialize, Serialize};

macro_rules! unop_ref_impl {
    (impl $trait:ident for $self:ty, $method:ident -> $out:ty) => {
        impl $trait for &$self {
            type Output = $out;

            fn $method(self) -> $out {
                $trait::$method(*self)
            }
        }
    };
}

macro_rules! binop_ref_impl {
    (impl $trait:ident<$other:ty> for $self:ty, $method:ident -> $out:ty) => {
        
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    /// Create a new `Vector3` with specified `x`, `y`, and `z` components.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }

    /// Create a new `Vector3` of zeros.
    pub fn zeros() -> Self {
        Vector3 { x: 0f32, y: 0f32, z: 0f32 }
    }

    /// Create a new `Vector3` of ones.
    pub fn ones() -> Self {
        Vector3 { x: 1f32, y: 1f32, z: 1f32 }
    }

    /// Create a new unit `Vector3` in the direction of the vector with `x`, `y`, `z`.
    pub fn unit(x: f32, y: f32, z: f32) -> Self {
        let norm = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        Vector3{x: x/norm, y: y/norm, z: z/norm}

    }

    /// Compute the square of the Euclidean norm of this vector.
    pub fn squared_norm(self) -> f32 {
        self.x.powi(2) +  self.y.powi(2) +  self.z.powi(2)
    }

    /// Compute the Euclidean norm of this vector.
    pub fn norm(self) -> f32 {
        self.squared_norm().sqrt()
    }

    /// Return a normalized copy of this vector.
    pub fn normalized(self) -> Vector3 {
        let norm = self.norm();
        Vector3 { x: self.x / norm, y: self.y / norm, z: self.z / norm }
    }

    /// Compute the dot product of this vector and `other`.
    pub fn dot(self, other: Vector3) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    /// Compute the cross product of this vector and `other`.
    pub fn cross(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Apply a component-wise reduction operation `f` to the paired `x`, `y`, and `z`, returning
    /// the result as a new vector.
    pub fn cwise(self, other: Vector3, f: fn(f32, f32) -> f32) -> Vector3 {
        Vector3 { x: f(self.x, other.x), y: f(self.y, other.y), z: f(self.z, other.z) }
    }

    /// Component-wise multiplication of this vector and `other`.
    pub fn cwise_mul(self, other: Vector3) -> Vector3 {
        // TODO
    }

    /// Component-wise division of this vector and `other`.
    pub fn cwise_div(self, other: Vector3) -> Vector3 {
        // TODO
    }

    /// Return the `x` component of this vector.
    pub fn x(self) -> f32 {
        self.x
    }

    /// Return the `y` component of this vector.
    pub fn y(self) -> f32 {
        self.y
    }

    /// Return the `z` component of this vector.
    pub fn z(self) -> f32 {
        self.z
    }
}

impl Add<Vector3> for Vector3 {
    // TODO
}

binop_ref_impl! { impl Add<Vector3> for Vector3, add -> Vector3 }

impl Sub<Vector3> for Vector3 {
    // TODO
}

binop_ref_impl! { impl Sub<Vector3> for Vector3, sub -> Vector3 }

impl Mul<f32> for Vector3 {
    // TODO
}

binop_ref_impl! { impl Mul<f32> for Vector3, mul -> Vector3 }

impl Mul<Vector3> for f32 {
    // TODO
}

binop_ref_impl! { impl Mul<Vector3> for f32, mul -> Vector3 }

impl Neg for Vector3 {
    // TODO
}

unop_ref_impl! { impl Neg for Vector3, neg -> Vector3 }

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p = f.precision().unwrap_or(2);
        write!(f, "[{:.*}, {:.*}, {:.*}]", p, self.x, p, self.y, p, self.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_init() {
        let new_zeros = Vector3::new(0.0, 0.0, 0.0);
        let zeros = Vector3::zeros();
        assert_eq!(
            new_zeros, zeros,
            "Vector3::zeros() failed. Expected {}, got {}.",
            new_zeros, zeros
        );

        let new_ones = Vector3::new(1.0, 1.0, 1.0);
        let ones = Vector3::ones();
        assert_eq!(
            new_ones, ones,
            "Vector3::ones() failed. Expected {}, got {}.",
            new_ones, ones
        );
    }

    #[test]
    fn test_norm() {
        let test1 = Vector3::new(1.0, 2.0, 2.0);
        let test2 = Vector3::new(2.0, 3.0, 6.0);

        let sq_norm = test1.squared_norm();
        let norm = test1.norm();
        assert_eq!(
            sq_norm, 9.0,
            "Vector3::squared_norm() failed on {}. Expected {}, got {}.",
            test1, 9.0, sq_norm
        );

        assert_eq!(
            norm, 3.0,
            "Vector3::norm() failed on {}. Expected {}, got {}.",
            test1, 3.0, norm
        );

        let sq_norm = test2.squared_norm();
        let norm = test2.norm();
        assert_eq!(
            sq_norm, 49.0,
            "Vector3::squared_norm() failed on {}. Expected {}, got {}.",
            test2, 49.0, sq_norm
        );

        assert_eq!(
            norm, 7.0,
            "Vector3::norm() failed on {}. Expected {}, got {}.",
            test2, 7.0, norm
        );
    }

    #[test]
    fn test_normalize_unit() {
        let test1 = Vector3::new(1.0, 2.0, 2.0);
        let test2 = Vector3::new(2.0, 3.0, 6.0);

        let unit1 = Vector3::unit(1.0, 2.0, 2.0);
        let unit2 = Vector3::unit(2.0, 3.0, 6.0);

        let normalized = test1.normalized();
        assert_eq!(
            normalized, unit1,
            "Vector3::unit() =/= Vector3::normalized() on {}. Expected {}, got {}.",
            test1, normalized, unit1
        );

        let normalized = test2.normalized();
        assert_eq!(
            normalized, unit2,
            "Vector3::unit() =/= Vector3::normalized() on {}. Expected {}, got {}.",
            test2, normalized, unit2
        );
    }

    #[test]
    fn test_dot() {
        let test1 = Vector3::new(1.0, 2.0, 2.0);
        let test2 = Vector3::new(2.0, 3.0, 6.0);
        let test3 = Vector3::new(1.0, 4.0, 8.0);

        let dot = test1.dot(test2);
        assert_eq!(
            20.0, dot,
            "Vector3::dot() failed on {} . {}. Expected {}, got {}.",
            test1, test2, 20.0, dot
        );

        let dot = test2.dot(test3);
        assert_eq!(
            62.0, dot,
            "Vector3::dot() failed on {} . {}. Expected {}, got {}.",
            test2, test3, 62.0, dot
        );
    }

    #[test]
    fn test_cross() {
        let unit_x = Vector3::new(1.0, 0.0, 0.0);
        let unit_y = Vector3::new(0.0, 1.0, 0.0);
        let unit_z = Vector3::new(0.0, 0.0, 1.0);

        let cross = unit_x.cross(unit_y);
        assert_eq!(
            unit_z, cross,
            "Vector3::cross() failed on {} x {}. Expected {}, got {}.",
            unit_x, unit_y, unit_z, cross
        );

        let cross = unit_y.cross(unit_z);
        assert_eq!(
            unit_x, cross,
            "Vector3::cross() failed on {} x {}. Expected {}, got {}.",
            unit_z, unit_y, unit_x, cross
        );

        let cross = unit_z.cross(unit_x);
        assert_eq!(
            unit_y, cross,
            "Vector3::cross() failed on {} x {}. Expected {}, got {}.",
            unit_z, unit_x, unit_y, cross
        );

        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(-1.0, 10.0, 1.0);
        let v3 = Vector3::new(-28.0, -4.0, 12.0);
        let cross = v1.cross(v2);
        assert_eq!(
            v3, cross,
            "Vector3::cross() failed on {} x {}. Expected {}, got {}.",
            v1, v2, v3, cross
        );
    }

    #[test]
    fn test_cwise() {
        let test1 = Vector3::new(1.0, 1.0, 1.0);
        let test2 = Vector3::new(1.0, 2.0, 4.0);
        let mul = Vector3::new(1.0, 2.0, 4.0);
        let div = Vector3::new(1.0, 0.5, 0.25);

        let cwise_mul = test1.cwise_mul(test2);
        let cwise_div = test1.cwise_div(test2);
        assert_eq!(
            mul, cwise_mul,
            "Vector3::cwise_mul() failed on {} * {}. Expected {}, got {}.",
            test1, test2, mul, cwise_mul
        );

        assert_eq!(
            div, cwise_div,
            "Vector3::cwise_div() failed on {} / {}. Expected {}, got {}.",
            test1, test2, div, cwise_div
        );
    }

    #[test]
    fn test_ops() {
        let test1 = Vector3::new(1.0, 1.0, 1.0);
        let test2 = Vector3::new(1.0, 2.0, 4.0);
        let test3 = Vector3::new(2.0, 3.0, 5.0);
        let test4 = Vector3::new(2.0, 4.0, 8.0);
        let test5 = Vector3::new(-1.0, -2.0, -4.0);

        let sum = test1 + test2;
        assert_eq!(
            test3, sum,
            "add() failed on {} + {}. Expected {}, got {}.",
            test1, test2, test3, sum
        );

        let diff = test3 - test2;
        assert_eq!(
            test1, diff,
            "sub() failed on {} - {}. Expected {}, got {}.",
            test3, test2, test1, diff
        );

        let mulr = 2.0 * test2;
        assert_eq!(
            test4, mulr,
            "mul() failed on {} * {}. Expected {}, got {}.",
            2.0, test2, test4, mulr
        );

        let mull = test2 * 2.0;
        assert_eq!(
            test4, mull,
            "mul() failed on {} * {}. Expected {}, got {}.",
            test2, 2.0, test4, mull
        );

        let neg = -test2;
        assert_eq!(
            test5, neg,
            "neg() failed on -{}. Expected {}, got {}.",
            test2, test5, neg
        );

        // Make sure implementations exist
        let _ = &test1 + test2;
        let _ = test1 + &test2;
        let _ = &test1 + &test2;

        let _ = &test1 - test2;
        let _ = test1 - &test2;
        let _ = &test1 - &test2;

        let _ = 2.0 * &test2;
        let _ = &test2 * 2.0;

        let _ = -&test1;
    }
}
