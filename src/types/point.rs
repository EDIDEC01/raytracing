use super::vector::Vec3;

pub type Point = Vec3;

impl Point {
    #[inline]
    pub fn x(&self) -> f32 {
        self.e[0]
    }
    #[inline]
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    #[inline]
    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn len_square(&self) -> f32 {
        f32::powi(self.x(), 2) + f32::powi(self.y(), 2) + f32::powi(self.z(), 2)
    }

    pub fn len(&self) -> f32 {
        f32::sqrt(self.len_square())
    }
}

pub fn dot(u: Point, v: Point) -> f32 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

pub fn cross(u: Point, v: Point) -> Point {
    Point::new(
        u.y() * v.z() - u.z() * v.y(),
        u.z() * v.x() - u.x() * v.z(),
        u.x() * v.y() - u.y() * v.x(),
    )
}

pub fn unit_vector(v: Point) -> Point {
    v / v.len()
}
