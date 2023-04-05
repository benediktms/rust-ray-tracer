use std::ops::{Add, Sub};

use crate::{Point3D, Vector3D};

pub trait CoordCompare<T> {
    /// structs of the same trait can be compared using the `PartialEq` and `PartialOrd` traits
    /// but when comparing strucsts of different types, e.g. `Vector3D` and `Point3D`, the `compare` method
    /// should be used
    fn compare(&self, other: &T) -> bool;
}

pub trait CoordOpposite {
    fn opposite(&self) -> Self;
}

impl CoordCompare<Vector3D> for Vector3D {
    fn compare(&self, other: &Vector3D) -> bool {
        self.x - other.x < std::f64::EPSILON
            && self.y - other.y < std::f64::EPSILON
            && self.z - other.z < std::f64::EPSILON
    }
}

impl CoordCompare<Point3D> for Point3D {
    fn compare(&self, other: &Point3D) -> bool {
        self.x - other.x < std::f64::EPSILON
            && self.y - other.y < std::f64::EPSILON
            && self.z - other.z < std::f64::EPSILON
    }
}

impl CoordOpposite for Vector3D {
    fn opposite(&self) -> Vector3D {
        Vector3D {
            x: 0.0 - self.x,
            y: 0.0 - self.y,
            z: 0.0 - self.z,
        }
    }
}

macro_rules! impl_Add {
    ( $( impl Add<$other:ty> for $self:ty; )+ ) => {
        $( impl Add<$other> for $self {
            type Output = Self;

            fn add(self, other: $other) -> Self::Output {
                Self {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                }
            }
        } )+
    };
}

macro_rules! impl_Sub {
    ( $( impl Sub<$other:ty> for $self:ty; )+ ) => {
        $( impl Sub<$other> for $self {
            type Output = Vector3D;

            fn sub(self, other: $other) -> Self::Output {
                Vector3D {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                }
            }
        } )+
    };
}

impl_Add! {
    impl Add<Vector3D> for Vector3D;
    impl Add<Point3D> for Vector3D;
    impl Add<Vector3D> for Point3D;
}

// Subtracting points always gives a vector. Subtracting a point from a vector redefines the vector.
impl_Sub! {
    impl Sub<Vector3D> for Point3D;
    impl Sub<Point3D> for Point3D;
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }
}
