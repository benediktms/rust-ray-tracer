pub enum CoordType {
    Vector3 = 0,
    Point = 1,
}

impl CoordType {
    fn into(self) -> u8 {
        self as u8
    }
}

pub struct Coord {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: CoordType,
}

impl Coord {
    fn vector(x: f64, y: f64, z: f64) -> Self {
        Coord {
            x,
            y,
            z,
            w: CoordType::Vector3,
        }
    }

    fn point(x: f64, y: f64, z: f64) -> Self {
        Coord {
            x,
            y,
            z,
            w: CoordType::Point,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3() {
        let v = Coord::vector(4.3, -4.2, 3.1);
        assert_eq!(v.w.into(), 0);
        assert_eq!(v.x, 4.3);
        assert_eq!(v.y, -4.2);
        assert_eq!(v.z, 3.1);
    }

    #[test]
    fn test_point() {
        let p = Coord::point(4.3, -4.2, 3.1);
        assert_eq!(p.w.into(), 1);
        assert_eq!(p.x, 4.3);
        assert_eq!(p.y, -4.2);
        assert_eq!(p.z, 3.1);
    }
}
