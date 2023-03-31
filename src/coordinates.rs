use std::ops::{Add, Sub};

pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// this trait can also be implemented to compare Vector3D and Point3D
trait CoordCompare<T> {
    fn compare(&self, other: &T) -> bool;
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
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }
}

impl Point3D {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3() {
        let v = Vector3D::new(4.3, -4.2, 3.1);
        assert_eq!(v.x, 4.3);
        assert_eq!(v.y, -4.2);
        assert_eq!(v.z, 3.1);
    }

    #[test]
    fn test_point() {
        let p = Point3D::new(4.3, -4.2, 3.1);
        assert_eq!(p.x, 4.3);
        assert_eq!(p.y, -4.2);
        assert_eq!(p.z, 3.1);
    }

    #[test]
    fn comparison_should_work() {
        let v1 = Vector3D::new(4.3, -4.2, 3.1);
        let v2 = Vector3D::new(4.3, -4.2, 3.1);
        let v3 = Vector3D::new(4.7, -4.2, 3.2);

        let p1 = Point3D::new(4.3, -4.2, 3.1);
        let p2 = Point3D::new(4.3, -4.2, 3.1);
        let p3 = Point3D::new(4.7, -4.2, 3.2);

        assert!(v1.compare(&v2));
        assert!(p1.compare(&p2));
        assert!(!v3.compare(&v1));
        assert!(!p3.compare(&p1));
        // assert!(!v1.compare(&p1));
    }

    #[test]
    fn adding_points_to_vectors_should_work() {
        let v1 = Vector3D::new(4.3, -4.2, 3.1);
        let p1 = Point3D::new(4.3, -4.2, 3.1);

        let p2 = v1 + p1;
        assert_eq!(p2.x, 8.6);
        assert_eq!(p2.y, -8.4);
        assert_eq!(p2.z, 6.2);
    }

    #[test]
    fn adding_vectors_to_points_should_work() {
        let v1 = Vector3D::new(4.3, -4.2, 3.1);
        let p1 = Point3D::new(4.3, -4.2, 3.1);

        let p2 = p1 + v1;
        assert_eq!(p2.x, 8.6);
        assert_eq!(p2.y, -8.4);
        assert_eq!(p2.z, 6.2);
    }

    #[test]
    fn subtracting_points_should_give_a_new_vector() {
        let p1 = Point3D::new(4.3, -4.2, 3.1);
        let p2 = Point3D::new(2.3, -2.2, 1.5);

        let v1 = p1 - p2;
        assert_eq!(v1.x, 2.0);
        assert_eq!(v1.y, -2.0);
        assert_eq!(v1.z, 1.6);
    }

    #[test]
    fn subtracting_vectors_from_points_should_give_a_new_vector() {
        let p1 = Point3D::new(4.3, -4.2, 3.1);
        let v1 = Vector3D::new(2.3, -2.2, 1.5);

        let v2 = p1 - v1;
        assert_eq!(v2.x, 2.0);
        assert_eq!(v2.y, -2.0);
        assert_eq!(v2.z, 1.6);
    }
}
