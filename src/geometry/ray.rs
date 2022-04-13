use super::vec3::{Point3, Vec3};

#[derive(Debug, Default, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        // debug_assert_eq!(direction.length_squared(), 1.0, "Ray must have unit direction!");
        Self { origin, direction }
    }

    pub fn at(self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn can_construct_ray() {
        Ray::new(Vec3::new(1., 2., 3.), Vec3::new(4., 5., 6.));
    }

    #[test]
    fn ray_at_works() {
        let ray = Ray::new(Vec3::new(1., 2., 3.), Vec3::new(1., 1., 1.));
        let expected = Vec3::new(4., 5., 6.);

        assert_eq!(expected, ray.at(3.));
    }
}
