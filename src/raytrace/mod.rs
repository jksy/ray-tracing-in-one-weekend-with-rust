use std::ops;

#[derive(Clone,Copy,Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn origin() -> Self {
        Vec3{
            e: [0f64, 0f64, 0f64]
        }
    }

    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3{
            e: [e0, e1, e2]
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }


    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        return
            self.e[0] * self.e[0] +
            self.e[1] * self.e[1] +
            self.e[2] * self.e[2];
    }

    pub fn dot(&self, right: Vec3) -> f64 {
        return
            self.e[0] * right.e[0] +
            self.e[1] * right.e[1] +
            self.e[2] * right.e[2];
    }

    pub fn cross(&self, right: Vec3) -> Self {
        return
            Self::new(
                self.e[1] * right.e[2] - self.e[2] * right.e[1],
                self.e[2] * right.e[0] - self.e[0] * right.e[2],
                self.e[0] * right.e[1] - self.e[1] * right.e[0],
            );
    }

    pub fn unit_vector(self) -> Self {
        let  length = self.length();
        return self / length;
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, r: Self) -> Self {
        return Self::new(self.e[0] + r.e[0], self.e[1] + r.e[1], self.e[2] + r.e[2]);
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, r: Self) -> Self {
        return Self::new(self.e[0] - r.e[0], self.e[1] - r.e[1], self.e[2] - r.e[2]);
    }
}

impl ops::Div for Vec3 {
    type Output = Vec3;

    fn div(self, r: Self) -> Vec3 {
        return Vec3::new(self.e[0] / r.e[2], self.e[1] / r.e[1], self.e[2] / r.e[2]);
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, r: f64) -> Vec3 {
        return self * (1.0f64 / r);
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, r: Self) -> Vec3 {
        return Vec3::new(self.e[0] * r.e[0], self.e[1] * r.e[1], self.e[2] * r.e[2]);
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, r: f64) -> Vec3 {
        return Vec3::new(self.e[0] * r, self.e[1] * r, self.e[2] * r);
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Debug)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            origin: origin,
            direction: direction
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        let origin = self.origin;
        return origin + self.direction * t;
    }

    pub fn color(&self) -> Color {
        let t =  self.hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5);
        if t > 0.0 {
            let at = self.at(t);
            let n = at - Vec3::new(0.0, 0.0, -1.0);
            return Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
        }
        let unit_direction = self.direction.unit_vector();
        let t = unit_direction.y() + 1.0;
        return Color::new(1.0, 1.0, 1.0) * (1.0 - t)  + Color::new(0.5*1.0, 0.7*1.0, 1.0*1.0) * t;
    }

    fn hit_sphere(&self, center: Point3, radius: f64) -> f64 {
        let oc = self.origin - center;
        let a = self.direction.length_squared();
        let half_b = oc.dot(self.direction);
        let c = oc.length_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return -1.0;
        } else {
            return (-half_b - discriminant.sqrt() ) / a;
        }
    }
}

impl Color {
    pub fn to_string(&self) -> String {
        return format!("{:?} {:?} {:?}",
            (self.e[0] * 255.999) as u8,
            (self.e[1] * 255.999) as u8,
            (self.e[2] * 255.999) as u8);
    }
}
