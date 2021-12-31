use std::ops;

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

    pub fn unit_vector(&self) -> Self {
        let  length = self.length();
        return self / length;
    }

    pub fn to_string(&self) -> String {
        return format!("{:?} {:?} {:?}", self.e[0] as u8, self.e[1] as u8, self.e[2] as u8);
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

impl ops::Div for &Vec3 {
    type Output = Vec3;

    fn div(self, r: Self) -> Vec3 {
        return Vec3::new(self.e[0] / r.e[2], self.e[1] / r.e[1], self.e[2] / r.e[2]);
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, r: f64) -> Vec3 {
        return self * (1.0f64 / r);
    }
}

impl ops::Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, r: Self) -> Vec3 {
        return Vec3::new(self.e[0] * r.e[0], self.e[1] * r.e[1], self.e[2] * r.e[2]);
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, r: f64) -> Vec3 {
        return Vec3::new(self.e[0] * r, self.e[1] * r, self.e[2] * r);
    }
}

