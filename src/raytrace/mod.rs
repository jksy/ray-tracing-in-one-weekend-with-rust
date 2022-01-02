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

    pub fn color(&self, world: &HitableList) -> Color {
        let (hit, record) = world.hit(self, 0.0, f64::INFINITY);
        if hit {
            return (record.normal + Color::new(1.0, 1.0, 1.0)) * 0.5
        }
        let unit_direction = self.direction.unit_vector();
        let t = (unit_direction.y() + 1.0) * 0.5;
        return Color::new(1.0, 1.0, 1.0) * (1.0 - t)  + Color::new(0.5, 0.7, 1.0) * t;
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

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord{
            p: Point3::origin(),
            normal: Vec3::origin(),
            t: 0.0,
            front_face: false,
        }
    }

    fn set_front_face(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction.dot(*outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal
        } else {
            self.normal = Vec3::origin() - (*outward_normal)
        }
    }
}

trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord);
}

#[derive(Clone, Copy)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hitable for Sphere  {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        let mut rec = HitRecord::new();
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return (false, rec);
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = -(half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return (false, rec);
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        return (true, rec);
    }
}

impl Sphere {
    pub fn new(center: Point3, radius:  f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}


pub struct HitableList {
    objects: Vec<Sphere>,
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList{
            objects: Vec::new()
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        for object in self.objects.iter() {
            let (hit, record) = object.hit(r, t_min, t_max);
            if hit {
                return (hit, record)
            }
        }

        return (false, HitRecord::new())
    }

    pub fn add(&mut self, object: &Sphere) {
        self.objects.push(*object)
    }
}

pub struct Camera  {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::origin();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin;
        Ray::new(self.origin, direction)
    }
}
