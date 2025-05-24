use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    pub fn length_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        let length = self.length();
        Vec3::new(self.x / length, self.y / length, self.z / length)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, val: f64) -> Self::Output {
        Vec3 {
            x: self.x + val,
            y: self.y + val,
            z: self.z + val,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, val: f64) -> Self::Output {
        Vec3 {
            x: self.x * val,
            y: self.y * val,
            z: self.z * val,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, val: f64) -> Self::Output {
        Vec3 {
            x: self.x - val,
            y: self.y - val,
            z: self.z - val,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, val: f64) -> Self::Output {
        Vec3 {
            x: self.x / val,
            y: self.y / val,
            z: self.z / val,
        }
    }
}
