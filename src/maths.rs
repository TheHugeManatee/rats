#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec2 {
    // construct a new Vec2
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    // calculate the length of a Vec2
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    // calculate the squared length of a Vec2
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    // normalized Vec2
    pub fn normalized(&self) -> Vec2 {
        let length = self.length();
        Vec2::new(self.x / length, self.y / length)
    }

    // dot product
    pub fn dot(a: Vec2, b: Vec2) -> f64 {
        a.x * b.x + a.y * b.y
    }
}

// implement the Add trait
impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}
// implement the Add trait for scalar addition
impl std::ops::Add<f64> for Vec2 {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self::new(self.x + other, self.y + other)
    }
}
// implement the Sub trait
impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}
// implement the Sub trait for scalar subtraction
impl std::ops::Sub<f64> for Vec2 {
    type Output = Self;
    fn sub(self, other: f64) -> Self {
        Self::new(self.x - other, self.y - other)
    }
}
// implemlement the Mul trait
impl std::ops::Mul for Vec2 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y)
    }
}
// implement the Mul trait for scalar multiplication
impl std::ops::Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self::new(self.x * other, self.y * other)
    }
}
// implement the Div trait
impl std::ops::Div for Vec2 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self::new(self.x / other.x, self.y / other.y)
    }
}
// implement the Div trait for scalar division
impl std::ops::Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self::new(self.x / other, self.y / other)
    }
}

impl Vec3 {
    // construct a new Vec3
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    // calculate the cross product of two Vec3s
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    // calculate the length of a Vec3
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // calculate the squared length of a Vec3
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    // normalized Vec3
    pub fn normalized(&self) -> Vec3 {
        let length = self.length();
        Vec3::new(self.x / length, self.y / length, self.z / length)
    }

    // dot product
    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn lerp(a: Vec3, b: Vec3, t: f64) -> Vec3 {
        a * (1.0 - t) + b * t
    }
}

// implement the Add trait
impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self::new(self.x + other.x, self.y + other.y, self.z + other.z);
    }
}

// implement the Add trait for scalar addition
impl std::ops::Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self::new(self.x + other, self.y + other, self.z + other)
    }
}
// implement the Sub trait
impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
// implement the Sub trait for scalar subtraction
impl std::ops::Sub<f64> for Vec3 {
    type Output = Self;
    fn sub(self, other: f64) -> Self {
        Self::new(self.x - other, self.y - other, self.z - other)
    }
}
// implemlement the Mul trait
impl std::ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}
// implement the Mul trait for scalar multiplication
impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self::new(self.x * other, self.y * other, self.z * other)
    }
}
// implement the Div trait
impl std::ops::Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}
// implement the Div trait for scalar division
impl std::ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self::new(self.x / other, self.y / other, self.z / other)
    }
}

// prefix neg
impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * 2.0 * v.dot(n)
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
pub fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / std::f64::consts::PI
}

pub type Point = Vec3;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    pub fn empty() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }
    pub fn universe() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, value: f64) -> bool {
        value >= self.min && value <= self.max
    }
    pub fn surrounds(&self, value: f64) -> bool {
        value > self.min && value < self.max
    }

    pub fn clamp(&self, value: f64) -> f64 {
        value.max(self.min).min(self.max)
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_new_instance() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 3.0);
    }

    #[test]
    fn it_adds_two_vectors() {
        let vec_1 = Vec3::new(1.0, 2.0, 3.0);
        let vec_2 = Vec3::new(4.0, 5.0, 6.0);
        let result = vec_1 + vec_2;
        assert_eq!(result, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn it_subtracts_two_vectors() {
        let vec_1 = Vec3::new(4.0, 5.0, 6.0);
        let vec_2 = Vec3::new(1.0, 2.0, 3.0);
        let result = vec_1 - vec_2;
        assert_eq!(result, Vec3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn it_multiplies_two_vectors() {
        let vec_1 = Vec3::new(1.0, 2.0, 3.0);
        let vec_2 = Vec3::new(4.0, 5.0, 6.0);
        let result = vec_1 * vec_2;
        assert_eq!(result, Vec3::new(4.0, 10.0, 18.0));
    }

    #[test]
    fn it_divides_two_vectors() {
        let vec_1 = Vec3::new(4.0, 10.0, 18.0);
        let vec_2 = Vec3::new(4.0, 5.0, 6.0);
        let result = vec_1 / vec_2;
        assert_eq!(result, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn it_adds_a_scalar_to_a_vector() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let result = vec + 1.0;
        assert_eq!(result, Vec3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn it_subtracts_a_scalar_from_a_vector() {
        let vec = Vec3::new(4.0, 5.0, 6.0);
        let result = vec - 1.0;
        assert_eq!(result, Vec3::new(3.0, 4.0, 5.0));
    }

    #[test]
    fn it_multiplies_a_vector_by_a_scalar() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let result = vec * 2.0;
        assert_eq!(result, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn it_divides_a_vector_by_a_scalar() {
        let vec = Vec3::new(4.0, 10.0, 18.0);
        let result = vec / 2.0;
        assert_eq!(result, Vec3::new(2.0, 5.0, 9.0));
    }

    #[test]
    fn it_calculates_the_cross_product() {
        let vec_1 = Vec3::new(1.0, 0.0, 0.0);
        let vec_2 = Vec3::new(0.0, 1.0, 0.0);
        let result = vec_1.cross(vec_2);
        assert_eq!(result, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn it_calculates_the_length() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let result = vec.length();
        assert_eq!(result, 14.0_f64.sqrt());
    }

    #[test]
    fn it_calculates_the_squared_length() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let result = vec.length_squared();
        assert_eq!(result, 14.0);
    }

    #[test]
    fn it_normalizes_a_vector() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let result = vec.normalized();
        assert_eq!(
            result,
            Vec3::new(
                1.0 / 14.0_f64.sqrt(),
                2.0 / 14.0_f64.sqrt(),
                3.0 / 14.0_f64.sqrt()
            )
        );
    }

    #[test]
    fn it_creates_a_zero_vector() {
        let vec = Vec3::zero();
        assert_eq!(vec, Vec3::new(0.0, 0.0, 0.0));
    }
}
