mod structs;
mod traits;

pub use structs::*;
pub use traits::*;

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

        assert_eq!(&v1, &v1);
        assert_eq!(&p1, &p1);
    }

    #[test]
    fn adding_points_to_vectors_should_work() {
        let v1 = Vector3D::new(4.3, -4.2, 3.1);
        let p1 = Point3D::new(4.3, -4.2, 3.1);
        let v2 = v1 + p1;

        assert_eq!(v2, Vector3D::new(8.6, -8.4, 6.2));
    }

    #[test]
    fn adding_vectors_to_points_should_work() {
        let v1 = Vector3D::new(4.3, -4.2, 3.1);
        let p1 = Point3D::new(4.3, -4.2, 3.1);
        let p2 = p1 + v1;
        assert_eq!(p2, Point3D::new(8.6, -8.4, 6.2));
    }

    #[test]
    fn subtracting_points_should_give_a_new_vector() {
        let p1 = Point3D::new(4.3, -4.2, 3.1);
        let p2 = Point3D::new(2.3, -2.2, 1.5);
        let v1 = p1 - p2;
        assert_eq!(v1, Vector3D::new(2.0, -2.0, 1.6));
    }

    #[test]
    fn subtracting_vectors_from_points_should_give_a_new_vector() {
        let p1 = Point3D::new(4.3, -4.2, 3.1);
        let v1 = Vector3D::new(2.3, -2.2, 1.5);

        let v2 = p1 - v1;
        assert_eq!(v2, Vector3D::new(2.0, -2.0, 1.6));
    }

    #[test]
    fn should_be_able_to_get_the_opposite_vector() {
        let v1 = Vector3D::new(1.0, -2.0, 3.0);
        let opposite = v1.opposite();

        assert_eq!(opposite, Vector3D::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn should_be_able_to_multiply_vectors() {
        let v1 = Vector3D::new(1.0, -2.0, 3.0);
        let v2 = v1 * 3.5;
        let v3 = v1 * 0.5;
        let v4 = v1 * 3;

        assert_eq!(v2, Vector3D::new(3.5, -7.0, 10.5));
        assert_eq!(v3, Vector3D::new(0.5, -1.0, 1.5));
        assert_eq!(v4, Vector3D::new(3.0, -6.0, 9.0));
    }

    #[test]
    fn should_be_able_to_divide_vectors() {
        let v1 = Vector3D::new(1.0, -2.0, 3.0);
        let v2 = v1 / 2;
        let v3 = v1 / 0.5;

        assert_eq!(v2, Vector3D::new(0.5, -1.0, 1.5));
        assert_eq!(v3, Vector3D::new(2.0, -4.0, 6.0));
    }
}
